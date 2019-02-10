# Rust错误处理

原文：[https://www.tuicool.com/articles/RBvu2yN](https://www.tuicool.com/articles/RBvu2yN)

## 1 常用的调试宏

* `panic!`可带任意类型的数据，支持`println!`风格的参数
* `unimplemented!`
* `unreachable!`
* `assert!`、`assert_eq!`、`assert_ne!`：除前两个参数外，可以带额外的`println!`风格的参数
* `debug_assert!`、`debug_assert_eq!`、`debug_assert_ne!`

## 2 `Result`与`Option`

* `is_some()`、`is_none()`、`is_ok()`、`is_err()`
* `Result`类型的`ok()`和`err()`方法将`Result`类型转换成`Option`类型
* `unwrap()`、`expect()`，反过来是`unwrap_err()`、`expect_err()`
* `unwrap_or()`、`unwrap_or()`（失败时使用指定的默认值）、`unwrap_or_default()`（失败时使用指定类型的默认值）
* `unwrap_or_else()`（失败时使用给定的闭包的返回值）

## 3 错误传播

* `?`可用于`Option`和`Result`返回类型
* 从Rust 1.26版本开始，也可以在`main()`中使用`?`

## 4 组合器

* `or()`、`and()`、`or_else()`、`and_then()`：组合类型的两个值，返回相同类型；前两个方法使用相同类型的参数，后两个方法的第二个参数是闭包
* `filter()`：对于`Option`类型，使用闭包作为条件函数来过滤，返回相同类型
* `map()`、`map_err()`：使用闭包进行类型转换，可更改内部值的类型，如`Some<&str>`可变成`Some<usize>`
* `map_or()`、`map_or_else()`：使用闭包进行类型转换，对于`None`或者`Err`，返回默认值，或者调用另一个闭包
* `ok_or()`、`ok_or_else()`：用于将`Option`类型转化成`Result`类型
* `as_ref()`、`as_mut()`：将类型转化成引用或者可变引用

### 4.1 `filter()`

* 示例在`Option`类型上使用`filter()`，也可以在迭代器上应用`filter()`方法

```rust
fn main() {
    let s1 = Some(3);
    let s2 = Some(6);
    let n = None;

    let fn_is_even = |x: &i8| x % 2 == 0;

    assert_eq!(s1.filter(fn_is_even), n);  // Some(3) -> 3 is not even -> None
    assert_eq!(s2.filter(fn_is_even), s2); // Some(6) -> 6 is even -> Some(6)
    assert_eq!(n.filter(fn_is_even), n);   // None -> no value -> None
}
```

### 4.2 `map()`

```rust
fn main() {
    let s1 = Some("abcde");
    let s2 = Some(5);

    let n1: Option<&str> = None;
    let n2: Option<usize> = None;

    let o1: Result<&str, &str> = Ok("abcde");
    let o2: Result<usize, &str> = Ok(5);
    
    let e1: Result<&str, &str> = Err("abcde");
    let e2: Result<usize, &str> = Err("abcde");
    
    let fn_character_count = |s: &str| s.chars().count();

    assert_eq!(s1.map(fn_character_count), s2); // Some("abcde") 经过函数映射成字符串长度 Some(5)
    assert_eq!(n1.map(fn_character_count), n2); // None 映射成 None
    assert_eq!(o1.map(fn_character_count), o2); // Ok("abcde") 经过函数映射成字符串长度 Ok(5)
    assert_eq!(e1.map(fn_character_count), e2); // Err 映射成 Err
}
```

### 4.3 `map_err()`

```rust
fn main() {
    let o1: Result<&str, &str> = Ok("abcde");
    let o2: Result<&str, isize> = Ok("abcde");

    let e1: Result<&str, &str> = Err("404");
    let e2: Result<&str, isize> = Err(404);

    let fn_character_count = |s: &str| -> isize { s.parse().unwrap() }; // convert str to isize

    assert_eq!(o1.map_err(fn_character_count), o2); // map_err 不改变 Ok 值
    assert_eq!(e1.map_err(fn_character_count), e2); // map_err 通过函数映射 Err("404") 到字符串长度 Err(404)
}
```

### 4.4 `map_or()`

```rust
fn main() {
    const V_DEFAULT: i8 = 1;

    let s = Some(10);
    let n: Option<i8> = None;
    let fn_closure = |v: i8| v + 2;

    assert_eq!(s.map_or(V_DEFAULT, fn_closure), 12);// Some(10) 经过闭包映射成 12
    assert_eq!(n.map_or(V_DEFAULT, fn_closure), V_DEFAULT);// None 不映射,返回默认值
}
```

## 5 `Display`/`Debug`/`Error`

```rust
use std::fmt;

struct AppError {
    code: usize,
    message: String,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let err_msg = match self.code {
            404 => "Sorry, Can not find the Page!",
            _ => "Sorry, something is wrong! Please Try Again!",
        };
        write!(f, "{}", err_msg)
    }
}

impl fmt::Debug for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"AppError {{ code: {}, message: {} }}",self.code, self.message)
    }
}

fn produce_error() -> Result<(), AppError> {
    Err(AppError {
        code: 404,
        message: String::from("Page not found"),
    })
}

fn main() {
    match produce_error() {
        Err(e) => eprintln!("{}", e),
        _ => println!("No error"),
    }
    eprintln!("{:?}", produce_error()); // Err(AppError { code: 404, message: Page not found })
    eprintln!("{:#?}", produce_error());
}
```

## 6 `From`

```rust
use std::fs::File;
use std::io::{self, Read};
use std::num;

#[derive(Debug)]
struct AppError {
    kind: String,
    message: String,
}

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError {
            kind: String::from("io"),
            message: error.to_string(),
        }
    }
}

impl From<num::ParseIntError> for AppError {
    fn from(error: num::ParseIntError) -> Self {
        AppError {
            kind: String::from("parse"),
            message: error.to_string(),
        }
    }
}

fn main() -> Result<(), AppError> {
    // 如果不能打开文件,则得到 io::Error
    // 与返回类型 AppError 不匹配
    // 但是 AppError 实现了 From 特性,可以将 io::Error 类型转化成 AppError
    let mut file = File::open("hello_world.txt")?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let _number: usize;
    // 类似地,可以将可能的 num::ParseIntError 转化成 AppError
    _number = content.parse()?;

    Ok(())
}
// --------------- 一些可能的运行时错误 ---------------
// 01. 如果不存在 hello_world.txt 文件
Error: AppError { kind: "io", message: "No such file or directory (os error 2)" }
// 02. 如果用户没有访问 hell_world.txt 文件的权限
Error: AppError { kind: "io", message: "Permission denied (os error 13)" }
// 03. 如果 hello_world.txt 的内容不是数
Error: AppError { kind: "parse", message: "invalid digit found in string" }
```

