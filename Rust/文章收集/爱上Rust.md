# 爱上Rust

原文: [https://www.tuicool.com/articles/hit/emM3AnF](https://www.tuicool.com/articles/hit/emM3AnF)
推酷转载：[https://www.tuicool.com/articles/emM3AnF](https://www.tuicool.com/articles/emM3AnF)

## 1 优雅的错误处理

* 各种错误
    * 程序逻辑错误
    * 操作错误：影响程序的外部错误条件
    * C 中臭名昭著的`SIGSEGV`信号
    * Java 中的`java.lang.NullPointerException`

* 问题：对于可能返回一个值，也可能会失败的函数，调用方如何区分这两种情况？
    * C语言：程序员自己处理
        * 有的使用哨兵值（如Linux系统将返回值一分为二，用负值表示错误）
        * 有的返回表示成功和失败的值，然后额外使用一个错误码
        * 有的直接忽略错误（eat errors entirely）
   * C++和Java：试图用异常来解决这个问题
       * 异常是有害的：[http://www.lighterra.com/papers/exceptionsharmful/](http://www.lighterra.com/papers/exceptionsharmful/)
       * 异常隐藏了错误：错误出现时，高层软件不知道是什么导致了错误。Java试图用Checked异常解决这个问题，但[在实践中有严重的缺点](https://blog.philipphauer.de/checked-exceptions-are-evil/)。
       * 异常是开发速度与长期操作性之间的交易：我们太关注开发速度，盲目地忽视了长期后果。开发者可以宣称错误是别人的问题，或者不会出错。
   * node.js：用参数传递错误，但是可能会被忽略或者滥用（可以不输入这个参数）。
   * Go：返回两个值，一个表示结果，一个表示错误，这相对于C来说是一种改进，但是[也不太好，容易出错](https://bluxte.net/musings/2018/04/10/go-good-bad-ugly/#noisy-error-management)。
   * Rust: 结合多种技术来处理错误
       * 代数数据类型：一个数据可以是一系列类型中的一种，程序员必须显式处理各种可能类型的值
       * 参数化类型
       * 函数返回值可以是两种类型之一：一种类型表示成功；另一种类型表示失败
   * 调用方使用模式匹配来处理返回值
     * 对表示成功的类型，可以取得正确的返回值
     * 对表示错误的类型，可以取得底层错误，然后可以处理错误、传播错误、或者改进错误（添加额外的上下文）并传播错误
     * 不能（至少不能隐含地）忽略错误：必须显式处理错误，无论采用哪种方式。

```rust
fn do_it(filename: &str) -> Result<(), io::Error> {
    let stat = match fs::metadata(filename) {
        Ok(result) => { result },
        Err(err) => { return Err(err); }
    };

    let file = match File::open(filename) {
        Ok(result) => { result },
        Err(err) => { return Err(err); }
    };

    /* ... */

    Ok(())
}
```

* 上述处理错误的方式已经很好了：清晰、健壮。Rust更进一步引入了传播操作符，这是优雅的表达 与 性能和健壮性 之间的完美平衡。


```rust
fn do_it_better(filename: &str) -> Result<(), io::Error> {
    let stat = fs::metadata(filename)?;
    let file = File::open(filename)?;
    /* ... */
    Ok(())
}
```

## 2 不可思议的宏

* C中的宏很危险，但是如果正确使用，则可以产生清晰的、更好的代码，所以危险不是主要问题，预处理器能力有限才是主要问题，比如说，预处理器不能访问抽象语法树。
* Rust中的卫生宏不仅解决了基于预处理的宏的很多安全问题，还及其强大：因为可以访问抽象语法树，宏几乎可以对语法进行无限扩展。
* 用一个感叹号作为指示，程序员明确地知道什么时候在使用宏。
* 灵活而强大的宏允许有效的实验。比如说，传播运算符（问号）最开始只是`try!`宏。这个`try!`使用的地方太多了，最终成了一个基于语言的解决方案，这就是传播运算符。
* 参考文档：[Rust宏简介](https://danielkeep.github.io/practical-intro-to-macros.html)

## 3 用着很爽的`format!`宏

* 几乎所有语言中都有类似C语言中`sprintf`的机制，在Rust中这种机制就是`format!`宏
* 可以对任何实现了`std::fmt::Display`的类型使用`{}`格式指定
* 可以对任何实现了`std::fmt::Debug`的类型使用`{:?}`格式指定
* 一般来说，每种格式指定符都对应一个特性

## 4 `include_str!`宏

```rust
// 原始字符串
let str = r##""What a curious felling!" said Alice"##;

// 编译时从文件系统找到文件，作为字符串加载
let lib = include_str!("statemap-svg.js");
```

## 5 好用的序列化/反序列化包`Serde`

* `Serde`使用过程宏生成结构体特定的序列化、反序列化代码，让程序员不必在抽象和性能之间进行艰难抉择
* 可选的字段用`Option`表示

```rust
#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct StatemapInputMetadata {
    start: Vec<u64>,
    title: String,
    host: Option<string>,
    entityKind: Option<string>,
    states: HashMap<string statemapinputstate="">,
}

let metadata: StatemapInputMetadata = serde_json::from_str(payload)?;
```

## 6 元组

* 有时候想使用一个小结构表示一些数据，但是不想定义结构体，这时候可以使用元组
* 元组是强类型的，可以用索引访问元素

```rust
let colors = vec![
            ("aliceblue", (240, 248, 255)),
            ("antiquewhite", (250, 235, 215)),
            ("aqua", (0, 255, 255)),
            ("aquamarine", (127, 255, 212)),
            ("azure", (240, 255, 255)),
            /* ... */
        ];
```





