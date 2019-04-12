# async_await详解

参考[https://rustlang-cn.org/office/rust/async-rust/async_await/chapter.html](https://rustlang-cn.org/office/rust/async-rust/async_await/chapter.html)

## 1 `async`

* `async`是为异步编程增加的关键字，可用于函数、闭包、代码块，使返回值成为实现了`Future`特性的值

```rust
async fn foo() -> u8 { 5 }// 返回类型为: Future<Output=u8>

fn bar() -> impl Future<Output = u8> {
    async {
        let x: u8 = await!(foo());
        x + 5
    }
}

fn baz() -> impl Future<Output = u8> {
    let closure = async |x: u8| {
        await!(bar()) + x
    };
    closure(5)
}
```

### 1.1 `async`与生命周期

```rust
async fn foo(x: &u8) -> u8 { *x }
// 上面的函数，等价于下面的函数
fn foo<'a>(x: &'a u8) -> impl Future<Output = ()> + 'a {
    async { *x }
}
```

* 对于取得的`Future`，通常应该立即`await!`，以便其中的非`'static`生命周期的参数仍然有效
* 如果存储`Future`或者将其发送到其他线程，要注意参数的生命周期
* 一个常见的解决生命周期相关问题的方法是：将参数也放到异步函数（闭包、代码块）中

```rust
async fn foo(x: &u8) -> u8 { *x }

fn bad() -> impl Future<Output = ()> {
    let x = 5;
    foo(&x) // 错误：x 的生命周期不够长
}

fn good() -> impl Future<Output = ()> {
    async {
        let x = 5;// 将参数x放到异步代码块中,解决生命周期的问题
        await!(foo(&x))
    }
}
```

### 1.2 `async move`

* 对闭包使用`async`时，可同时使用`move`

```rust
// 其中的多个async块可以访问同一个局部变量,只要它们都在变量的生命周期域中执行
async fn foo() {
    let my_string = "foo".to_string();

    let future_one = async {
        ...
        println!("{}", my_string);
    };

    let future_two = async {
        ...
        println!("{}", my_string);
    };
    // Run both futures to completion, printing "foo" twice
    let ((), ()) = join!(future_one, future_two);
}

// 使用async move将变量移动到async闭包中，使得可以在变量生命周期域之外使用Future
fn foo() -> impl Future<Output = ()> {
    let my_string = "foo".to_string();
    async move {
        ...
        println!("{}", my_string);
    }
}
```

## 2 `await!`与多线程

* `await!`可能会导致线程切换：如果`Future`还没有完成，系统可能会安排当前线程做其他事情；下次继续执行（因为被wake)`await!`的可能是其他线程
* 所以：使用`Rc`、`RefCell`等没有实现`Send`的类型，或者引用没有实现`Sync`的类型，是不安全的
* 同样：在`await!`期间使用传统的、不支持`Future`的锁也是不安全的，可能会导致死锁。为避免死锁，必须使用`future::lock`包中的`Mutex`