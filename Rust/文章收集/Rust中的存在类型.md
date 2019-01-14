# Rust中的存在限定类型

原文：[https://www.tuicool.com/articles/YFjYrub](https://www.tuicool.com/articles/YFjYrub)

* 很多语言支持**<font color="red">通用限定类型（universally quantified types）</font>**，更为人知的名字是泛型或者参数化类型
* 较少人知道**<font color="red">存在限定类型（existentially quantified types）</font>**

## 1 通用限定类型

```rust
// 通用限定类型A.可以使用特性限定来限制A必须实现的特性
fn take<A>(vec: Vec<A>, n: usize) -> Vec<A> {
  ...
}

take::<i32>(vec![1, 2, 3], 3);
take::<&str>(vec!["hello", " ", "world"], 1);
```

## 2 存在限定类型

```rust
trait Token {
  fn render(&self) -> String;
}

impl Token for String {
  fn render(&self) -> String {
    self.clone()
  }
}

impl Token for i32{
    fn render(&self) -> String{
        String::from(format!("{}",self))
    }
}

// 存在一个实现了Token特性的类型,函数将返回这个类型
// 这里给出了两种实现,可以使用任意一种
// 而这对调用方(main函数中的get_token调用)是透明的
fn get_token() -> impl Token {
  //"this is not a token".to_string()
  100
}

fn main(){
    let token = get_token();
    println!("{}",token.render());
}
```

* 调用方被限制为只能调用返回的特性类型（`Token`）的方法，而不能调用实际类型的方法（不能调用`String`类型的其他方法）
* 被调方（`get_token()`）可以修改内部实现，这对调用方是透明的
* `impl Token`是在Rust 1.26版本引入的，以前的版本不支持这种语法

## 3 问题

### 3.1 无法限定多个存在类型是相同的类型

```rust
fn renew_token(token: impl Token) -> impl Token {
    println!("{}",token.render());
    "this is not a token".to_string()
}

fn main(){
    let token = get_token();
    println!("{}",token.render());
    let token = renew_token(token);
    println!("{}",token.render());
}
```

* 文章的说法是错的：对于`renew_token()`方法，为什么要限制两个存在类型是相同的呢？我们只要求两个存在类型都实现了`Token`，没有其他限制。
* 在Rust 1.31版本中，上述代码可以编译通过

## 4 关联类型

* 关联类型是一种实现泛型的方式
* 带关联类型的特性就是实现了泛型的特性
* 不同的函数返回的`impl Token`类型是不同的，即使返回的原始类型相同：编译器不会具体分析返回的原始类型是什么，只是认为同一个函数总是返回某种相同的、实现了`Token`特性的类型

```rust
trait Token {
    type ItemType;
    fn render(&self) -> String;
    fn get_token() -> Self::ItemType;
    fn renew_token(t : Self::ItemType) -> Self::ItemType;
    fn renew_token2(&self,Self::ItemType) -> Self::ItemType;
}

impl Token for String {
    type ItemType = String;
    fn render(&self) -> String {
        self.clone()
    }
    fn get_token() -> Self::ItemType{
        "This is a string token".to_string()
    }
    fn renew_token(t : Self::ItemType) -> Self::ItemType{
        t + " being renewed"
    }
    fn renew_token2(&self,t : Self::ItemType) -> Self::ItemType{
        t + " being renewed 2"
    }
}

impl Token for i32{
    type ItemType = i32;
    fn render(&self) -> String{
        String::from(format!("{}",self))
    }
    fn get_token()->Self::ItemType{
        100
    }
    fn renew_token(t : Self::ItemType) -> Self::ItemType{
        t + 100
    }
    fn renew_token2(&self,t : Self::ItemType) -> Self::ItemType{
        t + 200
    }
}

fn get_and_renew<T: Token>() -> T::ItemType {
    let token = T::get_token();
    T::renew_token(token)
}

fn main(){

    println!("{}",get_and_renew::<i32>());
    println!("{}",get_and_renew::<String>());
    let t = i32::get_token();
    String::renew_token(t.to_string());

    // 这样写是正确的
    let x = 10 as <i32 as Token>::ItemType;
    println!("{:?}",x);

    10.renew_token2(x);

    // 完全限定语法
    <i32 as Token>::renew_token2(&10,x);
    <i32 as Token>::renew_token2(&x,x);

    fn get() -> impl Token{ 10 }
    fn get2() -> impl Token{ String::from("abc") }
    fn get3() -> impl Token{ 10u32 }
    fn get4() -> impl Token{ 10 }
    let mut t = get();
    //t = get2();// 不正确: 带不同关联类型的特性是不同的类型,不可以相互赋值
    //t = get3();// 不正确: 带相同关联类型的特性也不是相同的类型
    //t = get4();  // 不正确: 虽然get()和get4()返回的原始类型实际上是相同的,但是编译器没法进行这种分析
    //t = 100;// 不正确: 编译器不知道100的类型与get()返回的原始类型相同
    t = get();// 实际上编译器认为 get() 返回某种实现了 Token 的类型,但不分析具体的原始类型是什么
    t.render();
    let t = get();
    t.render();
    // 下面的语句都不对: 10 作为 impl Token 类型返回的时候,原始类型信息丢失
    // 不知道这个Token类型的关联类型(ItemType类型)如何表达
    /*
    t.renew_token2(10);
    t.renew_token2(10 as <i32 as Token>::ItemType);
    t.renew_token2(10 as (<i32 as Token>::ItemType));
    t.renew_token2(x);// 为什么这里不正确?
    <i32 as Token>::renew_token2(t,x);
    <i32 as Token>::renew_token2(&t,x);
    <i32 as Token>::renew_token2(&t as &i32,x);
    <i32 as Token>::renew_token2(t as i32,x);
    */
}
```
