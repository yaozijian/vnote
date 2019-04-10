# io

`std::io`模块包含执行输入输出时常用的东西。这个模块的核心是提供读取输入和写入输出的`Read`和`Write`特性。

## 1 `Read`和`Write`特性

* 很多其他类型（如`File`和`TcpStream`）实现了这两个特性，也可以为自定义类型实现这两个特性
* 通常将实现了这两个特性的类型称作`reader`和`writer`

```rust
use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()>{
	let mut f = File::open("foo.txt")?;
	let mut buffer = [0;10];

	f.read(&mut buffer)?;
	println!("bytes: {:?}",buffer);
	Ok(())
}
```


### 1.1 `Read`特性

* 仅仅要求实现一个`read()`方法，在此基础上提供了其他额外方法
* 每次`read()`调用可能对应一次系统调用，所以应该使用`BufReader`之类实现了`BufRead`的类型，以提升性能

#### 1.1.1 `Read::read()`

* `fn read(&mut self,buf:&mut [u8]) -> Result<usize>`// 唯一需要提供的方法
* 不保证是否会阻塞；但如果会阻塞，通常会返回`Err`
* 返回`Ok(n)`时，0<=n<=buf.len()，非零的返回值表示读取到了数据，而零的含义通常有两种：
	* EOF：通常表示不能再读取到数据了，但是不是一定不能再读取到数据了
	* 给定的缓冲区长度为零
* 出错时必须保证没有读取任何数据

#### 1.1.2 提供的方法

* `fn read_vectored(&mut self,bufs:& mut [IoVecMut]) -> Result<usize>`// 分散读取
* `fn read_to_end(&mut self,buf: &mut Vec<u8>) -> Result<usize>`// 读取直到出错
* `fn read_to_string(&mut self,buf:& mut String) -> Result<usize>`
* `fn read_exact()`// 读取指定字节数
* `fn bytes(self) -> Bytes<Self>`// 变成字节迭代器
* `fn chain()`// 串接两个Read
* `fn take()`// 得到至多返回指定数量数据的类型

### 1.3 `Write`特性

* 仅需要实现两个方法：write和flush

#### 1.3.1`Write::write()`

* `fn write(&mut self,buf:&[u8]) -> Result<usize>`
* 不保证将等待数据写入完成，写入将阻塞时可能会返回错误
* 返回Ok(0)表示不能再接受数据，通常也表示将来也不能再接受更多数据(EOF)；或者提供的缓冲区为空
* 只写入了部分字节不认为是错误
* ErrorKind::Interrupted表示通常可以重试的错误

#### 1.3.2 `Write::flush()`

* `fn flush(&mut self) -> Result<()>`
* 保证中间缓冲的数据都到达目标位置
* 没有将所有数据送达目标位置被认为是错误

#### 1.3.3 提供的方法

* `fn write_vectored()`
* `fn write_all()`
* `fn write_fmt()`

## 2 `Seek`和`BufRead`

* 这两个特性建立在reader之上，控制如何进行读取操作
* `BufRead`使用内部缓冲区以提供各种方式的读取操作

### 2.1 `std::io::Seek`特性

* `fn seek(&mut self,pos: SeekFrom) -> Result<u64>`
* `std::io::SeekFrom`枚举类型

```rust
pub enum SeekFrom{
	Start(u64),
	End(i64),
	Current(i64)
}
```

### 2.2 `std::io::BufRead`特性

* 对于实现了`Read`的实例，可以用`std::io::BufReader`将其包装成`BufRead`
* `fn fill_buf(&mut self)-> Result<&[u8]>`// 必须与consume配合使用
* `fn consume(&mut self,amt: usize)`
* 以上两个为必须实现的方法，以下为特性提供的方法
* `fn read_until()`// 读取直到遇到指定的分隔符或者EOF
* `fn read_line()`// 读取一行
* `fn split()`// 返回迭代器，根据指定的字节分割读取到的内容
* `fn lines()`// 返回迭代器，按行分割读取到的内容

## 3 `BufReader`和`BufWriter`

* `BufReader`和`BufWriter`封装reader与writer，通过使用内部缓冲区，减少对系统底层IO接口的调用
* `BufReader`提供额外的读取方法
* `BufWriter`没有提供额外的写入方法，仅仅是缓存写入的数据

### 3.1 `std::io::BufReader`结构体

* 为`Read`特性提供缓冲功能，以便在小量重复读取的情况下提升性能
* `fn buffer(&self) -> &[u8]`
* `fn into_inner(self) -> R`

### 3.2 `std::io::BufWriter`结构体

* 包装`Write`特性，提供缓冲功能，以便在小量重复写入的情况下提升性能
* `BufWriter`被释放时，将自动flush还没有写入的内容，但是会忽略flush时发生的错误；代码应该手动调用flush方法

## 4 标准输入和输出

* `fn std::io::stdout() -> std::io::Stdout`
* `std::io::Stdout`结构体代表到当前进程的标准输出流的句柄，每个句柄共享一个全局缓冲区，每个访问都通过锁同步，可以明确调用`lock`方法以请求锁定
* Windows系统中不支持写入非UTF-8字符
* `std::io::stdin()`函数返回一个`std::io::Stdin`结构体实例，表示到进程标准输入流的一个句柄。每个句柄共享进程的输入缓冲区，每个读取在内部是相互锁定而同步的，也可以明确调用`lock`方法以请求锁定
* Windows系统中不支持读取非UTF8字符

## 5 迭代器类型

* 本模块的各种结构体提供了各种对IO进行迭代的方法，如使用`lines()`方法来按行读取

## 6 函数

* 本模块提供了访问各种特征的函数
* `std::io::copy()`// 复制reader到writer
* `std::io::empty()`// 构造一个空的reader，读取操作总是返回Ok(0)
* `std::io::repeat()`// 构造一个reader，读取操作总是返回一个指定的字节
* `std::io::sink()`// 创建一个writer，支持写入任何数据(相当于/dev/null)
* `std::io::stderr()`
* `std::io::stdin()`
* `std::io::stdout()`

## 7 `io::Result`

* 很多可能导致错误的函数/方法返回这个类型，并且常常与`?`操作符配合使用
* `type Result<T> = std::result::Result<T,std::io::Error>`
* 结构体`std::io::Error`的常用方法：
	`fn kind(&self) -> ErrorKind`
	`fn raw_os_error(&self) -> Option<i32>`
	`fn from_raw_os_error(code: i32) -> Error`
	`fn new<E>(kind: ErrorKind,error: std::error::Error)`
* ErrorKind枚举
	* NotFound
	* PermissionDenied
	* ConnectionRefused
	* ConnectionReset
	* ConnectionAborted
	* NotConnected
	* AddrInUse
	* AddrNotAvailable
	* BrokenPipe
	* AlreadyExists
	* WouldBlock
	* InvalidInput
	* InvalidData
	* TimedOut
	* WriteZero: 通常表示不足写入
	* Interrupted：通常可以重试
	* Other
	* UnexpectedEof
* `std::error::Error`特性

```rust
trait Error: Debug + Display{
	fn description(&self) -> &str;// 已经废除，应该使用to_string()方法获取错误描述
	fn cause(&self) -> Option<&dyn Error>;
	fn source(&self) -> Option<&(dyn Error + 'static)>;
}
```

## 8 平台特性的行为

* 很多函数是对系统调用或者其他库的封装
