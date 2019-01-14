# Rust异步IO

参考文档：[https://hexilee.me/2018/12/17/rust-async-io/](https://hexilee.me/2018/12/17/rust-async-io/)

## 1 异步IO的基石：mio

* `mio`是一个极简的底层异步IO库，几乎所有异步IO程序都基于`mio`
* 两个核心功能

1. 对操作系统异步IO的封装
   * `Linux（Android）`=> `epoll`
   * `Windows` => `iocp`
   * `MacOS（iOS）、FreeBSD`=>kqueue
2. 用户自定义事件队列

### 1.1 示例代码

```toml
[dependencies]
mio = "0.6.16"
failure = "0.1.3"
```

```rust

#![allow(unused_must_use)]

use mio::*;
use mio::net::{TcpListener,TcpStream};
use std::io::{Read,Write,self};
use failure::Error;
use std::time::{Duration,Instant};

const TOKEN_LISTEN_SOCKET : Token = Token(0);
const TOKEN_SERVE_SOCKET : Token = Token(1);
const TOKEN_CLIENT_SOCKET: Token = Token(2);
const MSG_PING : &[u8] = b"Ping";
const MSG_PONG : &[u8] = b"Pong";

fn main() -> Result<(),Error>{

    let addr = "127.0.0.1:13265".parse()?;
    let listener = TcpListener::bind(&addr)?;

    let poll = Poll::new()?;
    poll.register(&listener,TOKEN_LISTEN_SOCKET,Ready::readable(),PollOpt::edge());

    let mut client_socket = TcpStream::connect(&addr)?;
    poll.register(&client_socket,TOKEN_CLIENT_SOCKET,Ready::readable() | Ready::writable(),PollOpt::edge());

    let mut serve_socket = None;

    let mut events = Events::with_capacity(1024);
    let start = Instant::now();
    let timeout = Duration::from_millis(10);

    'top: loop{

        if start.elapsed() >= timeout{
            break 'top;
        }

        poll.poll(&mut events,None)?;

        for event in events.iter(){
            match event.token(){
                TOKEN_LISTEN_SOCKET => {
                    let (socket,addr) = listener.accept()?;
                    println!("接受客户端连接: {}",&addr);
                    poll.register(&socket,TOKEN_SERVE_SOCKET,Ready::readable(),PollOpt::edge());
                    serve_socket = Some(socket);
                },
                TOKEN_SERVE_SOCKET => {
                    if event.readiness().is_readable() {
                        let mut hello = [0;4];
                        if let Some(ref mut handler) = &mut serve_socket{
                            match handler.read_exact(&mut hello){
                                Ok(_) => {

                                    assert_eq!(MSG_PING,&hello);
                                    println!("服务器收到Ping");

                                    match handler.write(MSG_PONG){
                                        Ok(_) => println!("服务器发送Pong完成"),
                                        Err(ref err) if err.kind() == io::ErrorKind::WouldBlock => continue,
                                        err => {err?;}
                                    }
                                }
                                Err(ref err) if err.kind() == io::ErrorKind::WouldBlock => continue,
                                err => err?
                            }
                        }
                    }
                },
                TOKEN_CLIENT_SOCKET => {
                    if event.readiness().is_readable() {
                        let mut hello = [0; 4];
                        match client_socket.read_exact(&mut hello) {
                            Ok(_) => {
                                assert_eq!(MSG_PONG, &hello);
                                println!("客户端收到Pong\n");
                            }
                            Err(ref err) if err.kind() == io::ErrorKind::WouldBlock => continue,
                            err =>{ err?;}
                        }
                    }
                    if event.readiness().is_writable(){
                        match client_socket.write(MSG_PING) {
                            Ok(_) => println!("客户端发送Ping完成"),
                            Err(ref err) if err.kind() == io::ErrorKind::WouldBlock => continue,
                            err => {err?;}
                        }
                    }
                },
                _ => unreachable!()
            }
        }
    }
    Ok(())
}
```

### 1.2 注释

* 容错性：不能完全相信`events`：上面示例代码中的`Err(ref err) if err.kind() == io::ErrorKind::WouldBlock`就是处理假事件的
* `PollOpt`有三种可选值：`edge`、`level`、`oneshot`，分别表示边沿触发、水平触发、单次触发
* 使用`PollOpt::edge`时应该遵循`排尽原则（Draining readiness）`：一直读取，直到得到`io::ErrorKind::WouldBlock`错误

## 2 使用线程执行异步操作

```rust
#![allow(dead_code)]

use std::sync::mpsc::{Sender, channel};

#[derive(Clone)]
pub struct Fs {
    task_sender: Sender<Task>,
}

pub enum Task {
    Exit,
    Println(String),
}

impl Fs {
    pub fn new() -> Self {
        let (sender, receiver) = channel();// 创建通道
        std::thread::spawn(move || {
            loop {
                match receiver.recv() {// 从通道接收任务信息
                    Ok(task) => {
                        match task {// 处理任务
                            Task::Println(ref string) => println!("{}", string),
                            Task::Exit => return
                        }
                    },
                    Err(_) => {
                        return;
                    }
                }
            }
        });
        Fs { task_sender: sender }
    }

    pub fn println(&self, string: String) {// 使用通道传递任务信息
        self.task_sender.send(Task::Println(string)).unwrap()
    }
}
```

### 2.1 需要返回值的异步操作

* 可使用回调函数实现：异步操作完成时调用指定的回调函数

```rust
use crossbeam_channel::{unbounded, Sender};
use failure::Error;
use std::boxed::FnBox;
use std::fs::File;
use std::io::Read;
use std::thread;

// 定义回调函数类型: 需要使用Box,因为函数是大小不确定类型
type FileCallback = Box<FnBox(Fs, File) -> Result<(), Error> + Sync + Send + 'static>;
type StringCallback = Box<FnBox(Fs, String) -> Result<(), Error> + Sync + Send + 'static>;

pub enum Task {
    Exit,
    Println(String),
    Open(FileCallback, Fs, String),
    ReadToString(StringCallback, Fs, File),
}

pub enum TaskResult {
    Exit,
    Open(FileCallback, Fs, File),
    ReadToString(StringCallback, Fs, String),
}

#[derive(Clone)]
pub struct Fs {
    task_sender: Sender<Task>,
}

pub struct FsHandler {
    io_worker: thread::JoinHandle<Result<(), Error>>,
    executor: thread::JoinHandle<Result<(), Error>>,
}

// 转发请求到通道中,对需要返回值的请求,需要给定回调函数
impl Fs {
    pub fn println(&self, string: String) -> Result<(), Error> {
        Ok(self.task_sender.send(Task::Println(string))?)
    }

    pub fn open<F>(&self, path: &str, callback: F) -> Result<(), Error>
    where
        F: FnOnce(Fs, File) -> Result<(), Error> + Sync + Send + 'static,
    {
        Ok(self.task_sender.send(Task::Open(
            Box::new(callback),
            self.clone(),
            path.to_string(),
        ))?)
    }

    pub fn read_to_string<F>(&self, file: File, callback: F) -> Result<(), Error>
    where
        F: FnOnce(Fs, String) -> Result<(), Error> + Sync + Send + 'static,
    {
        Ok(self
            .task_sender
            .send(Task::ReadToString(Box::new(callback), self.clone(), file))?)
    }

    pub fn close(&self) -> Result<(), Error> {
        Ok(self.task_sender.send(Task::Exit)?)
    }
}

impl FsHandler {
    pub fn join(self) -> Result<(), Error> {
        self.io_worker.join().unwrap()?;
        self.executor.join().unwrap()
    }
}

pub fn fs_async() -> (Fs, FsHandler) {
    let (task_sender, task_receiver) = unbounded();
    let (result_sender, result_receiver) = unbounded();

    // 接收并处理请求线程
    let io_worker = std::thread::spawn(move || {
        loop {
            match task_receiver.recv() {
                Ok(task) => {
                    match task {
                        Task::Println(ref string) => println!("{}", string), // 简单任务直接处理
                        Task::Open(callback, fs, path) => {
                            // 复杂任务需要将执行结果返回到执行线程
                            result_sender.send(TaskResult::Open(callback, fs, File::open(path)?))?
                        }
                        Task::ReadToString(callback, fs, mut file) => {
                            let mut value = String::new();
                            file.read_to_string(&mut value)?;
                            result_sender.send(TaskResult::ReadToString(callback, fs, value))?
                        }
                        Task::Exit => {
                            result_sender.send(TaskResult::Exit)?;
                            break;
                        }
                    }
                }
                Err(_) => {
                    break;
                }
            }
        }
        Ok(())
    });

    let executor = std::thread::spawn(move || {
        loop {
            // 执行线程负责调用回调函数
            let result = result_receiver.recv()?;
            match result {
                TaskResult::ReadToString(callback, fs, value) => callback.call_box((fs, value))?,
                TaskResult::Open(callback, fs, file) => callback.call_box((fs, file))?,
                TaskResult::Exit => break,
            };
        }
        Ok(())
    });

    (
        Fs { task_sender },
        FsHandler {
            io_worker,
            executor,
        },
    )
}

const TEST_FILE_VALUE: &str = "Hello, World!";

pub fn main() -> Result<(), Error> {
    let (fs, fs_handler) = fs_async();
    fs.open("./examples/test.txt", |fs, file| {
        fs.read_to_string(file, |fs, value| {
            assert_eq!(TEST_FILE_VALUE, &value);
            fs.println(value)?;
            fs.close()
        })
    })?;
    fs_handler.join()?;
    Ok(())
}
```

## 3 `mio`中的自定义事件

```rust
use crossbeam_channel::{unbounded, Sender};
use failure::Error;
use mio::*;
use std::boxed::FnBox;
use std::fs::File;
use std::io::Read;
use std::thread;

// 定义回调函数类型: 需要使用Box,因为函数是大小不确定类型
type FileCallback = Box<FnBox(Fs, File) -> Result<(), Error> + Sync + Send + 'static>;
type StringCallback = Box<FnBox(Fs, String) -> Result<(), Error> + Sync + Send + 'static>;

pub enum Task {
    Exit,
    Println(String),
    Open(FileCallback, Fs, String),
    ReadToString(StringCallback, Fs, File),
}

pub enum TaskResult {
    Exit,
    Open(FileCallback, Fs, File),
    ReadToString(StringCallback, Fs, String),
}

#[derive(Clone)]
pub struct Fs {
    task_sender: Sender<Task>,
}

pub struct FsHandler {
    io_worker: thread::JoinHandle<Result<(), Error>>,
    executor: thread::JoinHandle<Result<(), Error>>,
}

// 转发请求到通道中,对需要返回值的请求,需要给定回调函数
impl Fs {
    pub fn println(&self, string: String) -> Result<(), Error> {
        Ok(self.task_sender.send(Task::Println(string))?)
    }

    pub fn open<F>(&self, path: &str, callback: F) -> Result<(), Error>
    where
        F: FnOnce(Fs, File) -> Result<(), Error> + Sync + Send + 'static,
    {
        Ok(self.task_sender.send(Task::Open(
            Box::new(callback),
            self.clone(),
            path.to_string(),
        ))?)
    }

    pub fn read_to_string<F>(&self, file: File, callback: F) -> Result<(), Error>
    where
        F: FnOnce(Fs, String) -> Result<(), Error> + Sync + Send + 'static,
    {
        Ok(self
            .task_sender
            .send(Task::ReadToString(Box::new(callback), self.clone(), file))?)
    }

    pub fn close(&self) -> Result<(), Error> {
        Ok(self.task_sender.send(Task::Exit)?)
    }
}

impl FsHandler {
    pub fn join(self) -> Result<(), Error> {
        self.io_worker.join().unwrap()?;
        self.executor.join().unwrap()
    }
}

const FS_TOKEN: Token = Token(0);

pub fn fs_async() -> (Fs, FsHandler) {
    let (task_sender, task_receiver) = unbounded();
    let (result_sender, result_receiver) = unbounded();

    // 创建事件循环
    let poll = Poll::new().unwrap();
    let (registration, set_readiness) = Registration::new2();
    poll.register(
        &registration,
        FS_TOKEN,
        Ready::readable(),
        PollOpt::oneshot(),
    )
    .unwrap();

    // 接收并处理请求线程
    let io_worker = std::thread::spawn(move || {
        loop {
            match task_receiver.recv() {
                Ok(task) => {
                    match task {
                        Task::Println(ref string) => println!("{}", string), // 简单任务直接处理
                        Task::Open(callback, fs, path) => {
                            // 复杂任务需要将执行结果返回到执行线程
                            result_sender.send(TaskResult::Open(
                                callback,
                                fs,
                                File::open(path)?,
                            ))?;
                            // 通知事件就绪
                            set_readiness.set_readiness(Ready::readable())?;
                        }
                        Task::ReadToString(callback, fs, mut file) => {
                            let mut value = String::new();
                            file.read_to_string(&mut value)?;
                            result_sender.send(TaskResult::ReadToString(callback, fs, value))?;
                            // 通知事件就绪
                            set_readiness.set_readiness(Ready::readable())?;
                        }
                        Task::Exit => {
                            result_sender.send(TaskResult::Exit)?;
                            break;
                        }
                    }
                }
                Err(_) => {
                    break;
                }
            }
        }
        Ok(())
    });

    let executor = std::thread::spawn(move || {
        let mut events = Events::with_capacity(10);
        'mainloop: loop {
            // 在有事件发生时才尝试从通道接收消息
            poll.poll(&mut events, None)?;
            for event in events.iter() {
                match event.token() {
                    FS_TOKEN => {
                        // 执行线程负责调用回调函数
                        loop {
                            match result_receiver.try_recv() {
                                Ok(result) => match result {
                                    TaskResult::ReadToString(callback, fs, value) => {
                                        callback.call_box((fs, value))?
                                    }
                                    TaskResult::Open(callback, fs, file) => {
                                        callback.call_box((fs, file))?
                                    }
                                    TaskResult::Exit => break 'mainloop,
                                },
                                Err(_) => break,
                            }
                        }
                        poll.register(
                            &registration,
                            FS_TOKEN,
                            Ready::readable(),
                            PollOpt::oneshot(),
                        )
                        .unwrap();
                    }
                    _ => unreachable!(),
                }
            }
        }
        Ok(())
    });

    (
        Fs { task_sender },
        FsHandler {
            io_worker,
            executor,
        },
    )
}

const TEST_FILE_VALUE: &str = "Hello, World!";

pub fn main() -> Result<(), Error> {
    let (fs, fs_handler) = fs_async();
    fs.open("./examples/test.txt", |fs, file| {
        fs.read_to_string(file, |fs, value| {
            assert_eq!(TEST_FILE_VALUE, &value);
            fs.println(value)?;
            fs.close()
        })
    })?;
    fs_handler.join()?;
    Ok(())
}
```

### 要点1：创建事件循环

```rust
const FS_TOKEN: Token = Token(0);
let poll = Poll::new().unwrap();
let (registration, set_readiness) = Registration::new2();
poll.register(&registration,FS_TOKEN,Ready::readable(),PollOpt::oneshot()).unwrap();
```

### 要点2：发布事件

```rust
set_readiness.set_readiness(Ready::readable())?;
```

### 要点3：读取事件

```rust
let mut events = Events::with_capacity(10);
poll.poll(&mut events, None)?;
```

## 4 协程

* 写回调函数比较难受，而对于Rust，闭包中的生命周期问题更加复杂
* Rust最终选择协程作为异步IO的抽象
* Rust的协程是基于`生成器(generator)`的`无栈协程(stackless coroutine)`

```rust

#![feature(generators,generator_trait)]

use std::ops::{Generator, GeneratorState};

pub fn main() {
    let mut gen = fab(5);
    loop {
        match unsafe { gen.resume() } {// 消费一个产品
            GeneratorState::Yielded(value) => println!("下一个值: {}", value),
            GeneratorState::Complete(ret) => {
                println!("返回值: {}", ret);
                break;
            }
        }
    }
}

// 返回某种实现了生成器特性的类型
fn fab(mut n: u64) -> impl Generator<Yield = u64, Return = u64> {
    move || {
        let mut last = 0u64;
        let mut current = 1u64;
        while n > 0 {
            yield current;// 产生一个产品
            let tmp = current;
            current += last;
            last = tmp;
            n -= 1;
        }
        return current;
    }
}
```

### 4.1 生成器问题1：`yield`点处不能有引用存在

* 编译器错误号E0626：[https://doc.rust-lang.org/error-index.html#E0626](https://doc.rust-lang.org/error-index.html#E0626)

```rust
fn self_ref_generator() -> impl Generator<Yield=u64, Return=()> {
    || {
        let x: u64 = 1;
        let ref_x: &u64 = &x;
        // 下面两个语句交换次序就没有问题
        // borrow may still be in use when generator yields
        yield 0; // 在生成器yield点处借用可能仍然在使用中
        yield *ref_x;
    }
}
```

<font color="red">不明白后面几个小节的论述与前面的生成器有什么关系。为什么要在生成器中使用自引用来导致与生命周期相关的复杂问题呢？然后为什么NonNull就可以跳过生命周期检查呢？</font>

#### 4.1.1 自引用问题

* 上述编译器错误E0626与自引用问题有关

```rust
struct A<'a> {
    b: u64,
    ref_b: Option<&'a u64>
}

impl<'a> A<'a> {
    fn new() -> Self {
        let mut a = A{b: 1, ref_b: None};
        a.ref_b = Some(&a.b);
        a// 编译通不过：1 不能移动正被借用的变量 a (其实是a.b被借用) 2 不能返回含有本地引用(a.ref_b引用了a.b)的值
    }
}
```

* 改成如下的样子也不行：

```rust
use std::borrow::{BorrowMut};
impl<'a> A<'a> {
    fn boxed() -> Box<Self> {
        let mut a = Box::new(A{b: 1, ref_b: None});
        let mut_ref: &mut A = a.borrow_mut();
        mut_ref.ref_b = Some(&mut_ref.b);
        a // 编译通不过：1 不能返回引用本地变量a的值 2 返回a要求用生命周期'a借用a
    }
}
```

* 这样才可以

```rust
struct A<'a> {
    b: u64,
    ref_b: Option<&'a u64>
}

impl<'a> A<'a> {
    fn new() -> Self { A{b: 1, ref_b: None}}
    fn mute(&mut self) {}
}

fn main2() {
    let mut a = A::new();
    a.ref_b = Some(&a.b);
    a.mute();// 但是这一句编译通不过: 已经有对a的不可变借用,不能再进行可变借用了
}
```

#### 4.1.2 用`NonNull`解决自引用问题

* 可以用`NonNull`避过编译器检查，但是要自己保证内存安全，绝对不能 `move`，也不能对其可变引用使用 `mem::replace` 或 `mem::swap`

```rust
use std::ptr::NonNull;

struct A {
    b: u64,
    ref_b: NonNull<u64>
}

impl A {
    fn new() -> Self {
        A{b: 1, ref_b: NonNull::dangling()}
    }
}

fn main() {
    let mut a = A::new();
    a.ref_b = NonNull::from(&a.b);
}
```

#### 4.1.3 用`Pin`保证变量不能被移动、不能被取可变引用


