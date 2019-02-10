[file:///D:/dev/RustDev/rustup/toolchains/stable-x86_64-pc-windows-gnu/share/doc/rust/html/std/primitive.array.html](file:///D:/dev/RustDev/rustup/toolchains/stable-x86_64-pc-windows-gnu/share/doc/rust/html/std/primitive.array.html)

# array

* 数组表示为`[T;N]`，其中`T`是元素类型；`N`是编译时非负常量。
* 数组的字面量表示形式有两种：
    * 每个元素的列表：`[x,y,z]`
    * 重复表达式：`[x;N]`,其中元素`x`必须实现了`Copy`
* 含有0到32个元素的数组，如果元素类型实现了下述特性，则数组也实现了这个特性：
    * `Debug`、`IntoIterator`
    * `PartialEq`、`PartialOrd`、`Eq`、`Ord`
    * `Hash`、`AsRef`、`AsMut`
    * `Borrow`、`BorrowMut`
    * `Default`
* 上述限制的原因是，Rust当前不支持对数组大小的泛型：
    * `[Foo;3]`和`[Bar;3]`都是泛型类型`[T;3]`的实例
    * `[Foo;3]`和`[Foo;5]`则是完全不同的类型
* 作为一个补救措施，上述特性的实现是静态的，数组大小的上限是32
* <font color="red">但是，任意大小的数组都实现了`Copy`和`Clone`，只要元素类型实现了这两个特性。因为编译器做了特别处理。</font>
* 数组可以强制（coerce）转化为切片类型`[T]`，所以可以对数组调用切片类型的方法。实际上，数组的大部分方法都是通过切片提供的。切片的大小是动态的，不能强制转化成数组。
* 不能从数组中移除元素，但可以试试[std::mem::replace](file:///D:/dev/RustDev/rustup/toolchains/stable-x86_64-pc-windows-gnu/share/doc/rust/html/std/mem/fn.replace.html)
* `pub fn replace<T>(dest: &mut T, src: T) -> T`