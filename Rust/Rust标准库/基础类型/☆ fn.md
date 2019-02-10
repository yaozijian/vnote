[file:///D:/dev/RustDev/rustup/toolchains/stable-x86_64-pc-windows-gnu/share/doc/rust/html/std/primitive.fn.html](file:///D:/dev/RustDev/rustup/toolchains/stable-x86_64-pc-windows-gnu/share/doc/rust/html/std/primitive.fn.html)

# fn

* 函数指针有两种获取方式：通过函数名，或者通过不捕获环境的闭包
* 函数指针可以分成两类：安全的和不安全的
* 此外，函数指针还因ABI而不同，通过在类型名前增加`extern ABI名称`来表示。比如说，`fn()`不同于`extern "C" fn()`，又不同于`extern "stdcall" fn()`。
    * 非外部函数的ABI是`rust-call`
    * 外部函数不指定ABI时，默认ABI为`C`
    * ABI是`C`或者`cdecl`的函数可以是可变参数的（variadic），而通常的Rust函数，不能带可变个数的参数。
* 上述标记可以组合，所以`unsafe extern "stdcall" fn()`是有效的类型
* 与Rust中的引用相同，函数指针是不能为空的，如果要将函数指针通过FFI传递，并且允许空指针，则应该使用`Option<fn()>`
* 函数指针（受限于当前的类型系统，仅对不大于12个参数、且ABI为Rust或者C的函数）实现了下述特性
    * `Clone`
    * `PartialEq`
    * `Eq`
    * `Ord`
    * `Hash`
    * `Pointer`
    * `Debug`
* <font color="red">具有任何签名、ABI、安全性的函数指针，都实现了`Copy`；而所有安全的函数指针都实现了`Fn`、`FnMut`、`FnOnce`，因为编译器做了特别处理。</font>

```rust
// 闭包作为函数指针
let clos: fn(usize) -> usize = |x| x + 5;
assert_eq!(clos(5), 10);

fn add_one(x: usize) -> usize {
    x + 1
}

unsafe fn add_one_unsafely(x: usize) -> usize {
    x + 1
}

let safe_ptr: fn(usize) -> usize = add_one;

// 错误：类型不匹配：需要普通的fn，给出的是unsafe fn
// 安全性（有无unsafe）是函数类型的一部分
// let bad_ptr: fn(usize) -> usize = add_one_unsafely;
let unsafe_ptr: unsafe fn(usize) -> usize = add_one_unsafely;
let really_safe_ptr: unsafe fn(usize) -> usize = add_one;
```

## `std::ops::FnOnce`

```rust
#[lang = "fn_once"]
pub trait FnOnce<Args> {
    type Output;
    extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
}
```

<font color="red">
* 采用传值接收器的（函数）调用运算符，只能被调用一次
* 可能消费捕获变量的闭包自动实现`FnOnce`
* 所有实现`FnMut`特性的类型（如安全的函数指针）都自动实现了`FnOnce`
* `Fn`和`FnMut`都是`FnOnce`的子特性（subtrait），所以任何`Fn`或者`FnMut`实例都可以用在需要`FnOnce`的地方
* 在需要接收类似函数类型的参数、并且只需要调用一次的地方，应该使用`FnOnce`；如果需要重复调用，则使用`FnMut`；如果不允许改变状态，则使用`Fn`
</font>

### 示例1：调用传值的闭包

```rust
let x = 5;
let square_x = move || x * x;
assert_eq!(square_x(), 25);
```

### 示例2：使用`FnOnce`参数

```rust
fn consume_with_relish<F>(func: F) where F: FnOnce() -> String{
    // `func`消费了捕获的变量，所以不能再次调用
    println!("Consumed: {}", func());
    println!("Delicious!");
    // 试图再次调用`func()`将导致编译错误：使用已经移动的值func
    // println!("Consumed: {}", func());
}

let x = String::from("x");
let consume_and_return_x = move || x;
consume_with_relish(consume_and_return_x);
// 这里不能再次使用consume_and_return_x，因为它已经被移动
// consume_with_relish(consume_and_return_x);
```

## `std::ops::FnMut`

```rust
#[lang = "fn_mut"]
pub trait FnMut<Args>: FnOnce<Args> {
    extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
}
```

<font color="blue">
* 使用可变接收器的（函数）调用运算符，可以被重复调用，可能会修改状态
* 采用可变引用方式捕获变量的闭包会自动实现`FnMut`
* 所有实现了`Fn`的类型（如安全的函数指针）都自动实现了`FnMut`
* 此外，如果类型`F`实现了`FnMut`，则`&mut F`将自动实现`FnMut`
* `FnOnce`是`FnMut`的超特性（supertrait），任何`FnMut`实例都可以用在需要`FnOnce`的地方
* `Fn`是`FnMut`的子特性（subtrait），任何`Fn`实例都可以用在需要`FnMut`的地方
* 在需要接收类似函数类型的参数、并且需要重复调用、同时允许修改状态的地方，应该使用`FnMut`；如果不允许修改状态，则使用`Fn`；如果不需要重复调用，则使用`FnOnce`
</font>

### 示例1：调用可变捕获的闭包

```rust
let mut x = 5;
{
    let mut square_x = || x *= x;
    square_x();
}
assert_eq!(x, 25);
```

### 示例2：使用`FnMut`参数

```rust
// 注意：必须用mut修饰参数，因为call_mut方法的接收器类型是&mut self
fn do_twice<F>(mut func: F) where F: FnMut(){
    func();
    func();// 可重复调用
}
let mut x: usize = 1;
{
    let add_two_to_x = || x += 2;
    do_twice(add_two_to_x);
}
assert_eq!(x, 5);
```

## `std::ops::Fn`

```rust
#[lang = "fn"]
pub trait Fn<Args>: FnMut<Args> {
    extern "rust-call" fn call(&self, args: Args) -> Self::Output;
}
```

<font color="green">
* 采用不可变接收器的（函数）调用运算符
* `Fn`实例可以被重复调用，而不会修改状态
* 不要将`Fn`与`fn`（函数指针）相混淆
* 采用不可变引用方式捕获变量的闭包，或者不捕获变量的闭包，将自动实现`Fn`
* 安全的函数指针也将自动实现`Fn`
* 此外，对任何实现了`Fn`的类型`F`，`&F`将自动实现`Fn`
* 因为`FnMut`和`FnOnce`都是`Fn`的超特性（supertrait），所以`Fn`实例可用于需要`FnMut`和`FnOnce`的地方
* 在需要接收类似函数类型的参数、并且需要重复调用、但是不允许修改状态的地方，应该使用`Fn`；如果没有如此严格的要求，应该使用`FnMut`或者`FnOnce`
</font>

### 示例1：调用闭包

```rust
let square = |x| x * x;
assert_eq!(square(5), 25);
```

### 示例2：使用`Fn`参数

```rust
fn call_with_one<F>(func: F) -> usize where F: Fn(usize) -> usize {
    func(1)
}

let double = |x| x * 2;
assert_eq!(call_with_one(double), 2);
```