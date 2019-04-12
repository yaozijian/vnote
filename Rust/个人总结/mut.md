# mut

# 特性中的`mut`

```rust
trait Demo{
	fn add(a:i32,b:i32) -> i32;
}

struct ImplIt{}

impl Demo for ImplIt{
	// 注意: 这里用mut修饰b,但是Demo特性定义中没有mut
	fn add(a:i32,mut b:i32)->i32{
		b = 100;
		a+b
	}
}
```

# `mut`与引用

```rust
let a = 123;
let b = 456;

let pi :&i32 = &a;
//pi = &b;// 错误: 没有用mut修饰pi,不可以对pi重新赋值

let mut pi : &i32 = &a;
pi = &b;// 正确：用mut修饰了pi,可以重新赋值
//*pi = 456;// 错误: 没有对引用类型&i32使用mut,不可以对引用的资源进行修改


let mut a = 123;
let pi : &mut i32 = &mut a;
//pi = &mut b;// 错误: 没有用mut修饰pi,不可以对pi重新赋值
*pi = 456;// 正确: 对数据类型&i32使用mut,可以对引用的资源进行修改

let mut a = 123;
let mut b = 456;
let mut pi :&mut i32 = &mut a;
pi = &mut b;// 正确: 用mut修饰了pi,可以重新赋值
*pi = 456;// 正确: 对数据类型&i32使用mut,可以对引用的资源进行修改
```
