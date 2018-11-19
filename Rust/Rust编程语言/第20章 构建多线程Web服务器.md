# 第20章 构建多线程Web服务器

## 1 单线程Web服务器

### 1.1 启动监听、接受连接

```rust
use std::net::TcpListener;
pub fn step1(){
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming(){
        let stream = stream.unwrap();
        println!("接受一个连接");
    }
}
```

### 1.2 读取HTTP请求

```rust
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

pub fn step1(){
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming(){
        handle_connection(stream.unwrap());
    }
}

fn handle_connection(mut stream: TcpStream){
    let mut buf = [0;512];
    let size = stream.read(&mut buf).unwrap();
    println!("读取到HTTP请求: {}字节\n{}\n",size,String::from_utf8_lossy(&buf[..size]));
}
```

### 1.3 发送回应

```rust
fn handle_connection(mut stream: TcpStream){
    let mut buf = [0;512];
    let size = stream.read(&mut buf).unwrap();
    println!("读取到HTTP请求: {}字节\n{}\n",size,String::from_utf8_lossy(&buf[..size]));

    let ack = "HTTP/1.1 200 OK\r\n";
    stream.write(ack.as_bytes()).unwrap();
    stream.flush().unwrap();
}
```

### 1.4 从文件中读取HTML

```rust
fn handle_connection(mut stream : TcpStream){
    let mut buf = [0;512];
    let size = stream.read(&mut buf).unwrap();
    println!("读取到HTTP请求: {}字节\n{}\n",size,String::from_utf8_lossy(&buf[..size]));

    let mut file = File::open("./src/index.html").unwrap();
    let mut cont = String::new();
    file.read_to_string(&mut cont).unwrap();

    let mut file = File::open("./src/index.html").unwrap();
    let mut cont = String::new();
    file.read_to_string(&mut cont).unwrap();

    let ack = format!("HTTP/1.1 200 OK\r\n{}",cont);
    stream.write(ack.as_bytes()).unwrap();
    stream.flush().unwrap();
}
```

### 1.5 区分请求路径

```rust
fn handle_connection(mut stream : TcpStream){

    let mut buf = [0;512];

    let size = stream.read(&mut buf).unwrap();
    println!("读取到HTTP请求: {}字节\n{}\n",size,String::from_utf8_lossy(&buf[..size]));

    let root = b"GET / HTTP/1.1";
    if buf.starts_with(root) {
        let mut file = File::open("./src/index.html").unwrap();
        let mut cont = String::new();

        file.read_to_string(&mut cont).unwrap();

        let ack = format!("HTTP/1.1 200 OK\r\n{}", cont);
        stream.write(ack.as_bytes()).unwrap();
        stream.flush().unwrap();
    }else{
        let mut file = File::open("./src/404.html").unwrap();
        let mut cont = String::new();

        file.read_to_string(&mut cont).unwrap();

        let ack = format!("HTTP/1.1 400 NOT FOUND\r\n{}", cont);
        stream.write(ack.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
```

### 1.6 提取函数

```rust
fn handle_connection(mut stream : TcpStream){

    let mut buf = [0;512];

    let size = stream.read(&mut buf).unwrap();
    println!("读取到HTTP请求: {}字节\n{}\n",size,String::from_utf8_lossy(&buf[..size]));

    let root = b"GET / HTTP/1.1";

    if buf.starts_with(root) {
        return_page(&mut stream,"HTTP/1.1 200 OK","./src/index.html");
    }else{
        return_page(&mut stream,"HTTP/1.1 400 NOT FOUND","./src/404.html");
    }
}

fn return_page(stream : &mut TcpStream,status:&str,file:&str){

    let mut file = File::open(file).unwrap();
    let mut cont = String::new();

    file.read_to_string(&mut cont).unwrap();

    let ack = format!("{}\r\n{}",status, cont);
    stream.write(ack.as_bytes()).unwrap();
    stream.flush().unwrap();
}
```

## 2 扩展成多线程

### 2.1 线程池

```rust
use std::thread;

pub struct ThreadPool{
    size : usize,
    workers : Vec<Worker>,
}

struct Worker{
    id: usize,
    handle: thread::JoinHandle<()>,
}

impl Worker{
    pub fn new(id: usize) -> Worker{
        let thread = thread::spawn(|| {});//什么也不做的空线程
        Worker{
            id:id,
            handle:thread,
        }
    }
}

impl ThreadPool{

    pub fn new(max : usize) -> ThreadPool{
        assert!(max > 0);

        // 创建多个工作者
        let mut list = Vec::with_capacity(max);
        for id in 0..max{
            list.push(Worker::new(id));
        }

        ThreadPool{
            size: max,
            workers: list,
        }
    }

    // 后续需要将f传递到Worker中
    pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static {
    }
}
```

* `std::thread::spawn()`的签名如下：

```rust
pub fn spawn<F,T>(f: F) -> JoinHandle<T> where F:FnOnce() -> T + Send + 'static{},T: Send + 'static
```

* 分析如下
  * 不关心返回值的类型`T`
  * 参数类型`F`的特性限定`FnOnce()`在`execute()`中也是需要的：因为参数需要最终传递到`execute()`中
  * 参数类型`F`的特性限定`Send`是必须的：`Send`允许将参数从一个线程传递到另一个线程（新创建的线程）
  * 参数类型`F`的生命周期限定`'static`是必须的：不知道线程会运行多长时间

### 2.2 使用通道传递工作函数

```rust
use std::sync;
use std::sync::mpsc;
use std::thread;

// 要传递的工作函数由特性限定表示；但是不能直接传递大小不确定的特性类型，要用Box封装
type Job = Box<FnOnce() + Send + 'static>;

pub struct ThreadPool {
    size: usize,
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Worker {
    id: usize,
    handle: thread::JoinHandle<()>,
}

impl Worker {
    pub fn new(id: usize, receiver: sync::Arc<sync::Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {// 需要将receiver的所有权移动到闭包中，所以使用move
            let job = receiver.lock().unwrap().recv().unwrap();
            // 这里无法通过编译: 被封装的 FnOnce() 特性在作为函数进行调用时，需要获取资源所有权
            // 但是Box不允许移出资源的所有权：资源是属于Box的，不能移出
            // 解决方案见下一节
            (*job)();
        });
        Worker {
            id: id,
            handle: thread,
        }
    }
}

impl ThreadPool {
    pub fn new(max: usize) -> ThreadPool {
        assert!(max > 0);

        let (tx, rx) = mpsc::channel();
        let receiver = sync::Arc::new(sync::Mutex::new(rx));

        let mut list = Vec::with_capacity(max);

        for id in 0..max {
            // 不能直接使用receiver，因为所有权被转移走，后续循环中就不能再使用了
            // 使用Arc和Mutex进行封装
            // Arc适用于资源需要被多线程、多处只读使用、无法确定哪部分最后结束使用（然后释放资源）的情况
            // Mutex保证资源的互斥使用
            list.push(Worker::new(id, receiver.clone()));
        }

        ThreadPool {
            size: max,
            workers: list,
            sender: tx,
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // 用Box封装工作函数，然后发送到通道中
        let job = Box::new(f);
        self.sender.send(job);
    }
}
```

### 2.3 使用`Box<Self>`

```rust
use std::sync;
use std::sync::mpsc;
use std::thread;

pub trait FnBox {
    fn call_box(self: Box<Self>);
}

// 总括实现: 用Box<Self>可以取得Box中的值的所有权
impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<Self>) {
        (*self)();
    }
}

type Job = Box<FnBox + Send + 'static>;

pub struct ThreadPool {
    sender: mpsc::Sender<Job>,
}

struct Worker;

impl Worker {
    fn run(id: usize, receiver: sync::Arc<sync::Mutex<mpsc::Receiver<Job>>>) {
        thread::spawn(move ||
            loop {
                // lock()的返回值为LockResult<MutexGuard<T>>类型,其中的MutexGuard<T>失效时，锁被自动释放
                // let 语句中,对MutexGuard<T>调用recv().unwrap()方法后，将结果赋值给job后，临时的MutexGuard<T>就失效了，锁被释放
                let job = receiver.lock().unwrap().recv().unwrap();
                println!("线程{}执行任务", id);
                job.call_box();//通过Box<Self>类型获取Box中的值的所有权,然后将值作为函数进行调用
            }
            /*
            // while 表达式中的值在整个while块中一直有效，MutexGuard<T>一直有效，锁一直被持有：在调用job.call_box()期间锁也被持有
            // 无法实现多个请求的并行执行
            while let Ok(job) = receiver.lock().unwrap().recv() {
                println!("Worker {} got a job; executing.", id);
                job.call_box();
            }
            */
        );
    }
}

impl ThreadPool {
    pub fn new(max: usize) -> ThreadPool {
        assert!(max > 0);

        let (tx, rx) = mpsc::channel();
        let receiver = sync::Arc::new(sync::Mutex::new(rx));

        for id in 0..max {
            Worker::run(id, receiver.clone());
        }

        ThreadPool { sender: tx }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnBox + Send + 'static // 这里FnOnce() 改成 FnBox
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}
```

* 将第一节的单线程服务器改成多线程时，仅需要修改1.1节的step1()函数

```rust
pub fn step1(){
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // 创建线程池
    let pool = super::section2::ThreadPool::new(4);
    for stream in listener.incoming(){
        // 使用线程池执行函数
        pool.execute(||handle_connection(stream.unwrap()));
    }
}
```

## 3 优雅停机与清理

```rust
use std::sync;
use std::sync::mpsc;
use std::thread;

pub trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    // 使用Box<Self>可以从Box<T>中取出T的值，获得所有权
    fn call_box(self: Box<Self>) {
        (*self)();
    }
}

type Job = Box<FnBox + Send + 'static>;

// 使用枚举类型粘结异构类型
enum Msg {
    NewJob(Job),
    Exit,
}

pub struct ThreadPool {
    sender: mpsc::Sender<Msg>,
    list: Vec<Worker>,
}

struct Worker {
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn run(id: usize, receiver: sync::Arc<sync::Mutex<mpsc::Receiver<Msg>>>) -> Worker {
        let t = thread::spawn(move || loop {
            // 使用if let
            if let Msg::NewJob(job) = receiver.lock().unwrap().recv().unwrap() {
                println!("线程{}执行任务", id);
                job.call_box();
            } else {
                println!("线程{}收到退出请求", id);
                break;
            }

            // 使用match
            /*
            let msg = receiver.lock().unwrap().recv().unwrap();
            match msg {
                Msg::NewJob(job) => {
                    println!("线程{}执行任务", id);
                    job.call_box();
                }
                Msg::Exit => {
                    println!("线程{}收到退出请求", id);
                    break;
                }
            }
            */        });

        Worker { thread: Some(t) }
    }
}

impl ThreadPool {
    pub fn new(max: usize) -> ThreadPool {
        assert!(max > 0);

        let (tx, rx) = mpsc::channel();
        // 使用Arc和Mutex封装类型,使得类型可以安全地在多线程中使用
        let receiver = sync::Arc::new(sync::Mutex::new(rx));
        let mut list = Vec::with_capacity(max);

        for id in 0..max {
            list.push(Worker::run(id, receiver.clone()));
        }

        ThreadPool {
            sender: tx,
            list: list,
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnBox + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Msg::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // 发送多个退出请求,让每个线程收到一个
        for _ in &self.list {
            self.sender.send(Msg::Exit).unwrap();
        }
        // 等待每个线程退出
        for item in &mut self.list {
            if let Some(t) = item.thread.take() {
                match t.join() {
                    Ok(_) => 0,
                    Err(_) => 1,
                };
            }
        }
    }
}
```