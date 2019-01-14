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

### 4.2 

