
[file:///D:/dev/RustDev/rustup/toolchains/stable-x86_64-pc-windows-gnu/share/doc/rust/html/std/primitive.tuple.html](file:///D:/dev/RustDev/rustup/toolchains/stable-x86_64-pc-windows-gnu/share/doc/rust/html/std/primitive.tuple.html)

# tuple

* 元组是一个有限的异构序列
    * 有限：元组有长度
    * 异构：每个元素可以有不同的类型
    * 序列：可以用索引访问各个元素
* 如果每个元素都实现了下述特性中的某一个，则元组也实现了这个特性：
* <font color="red">受到类型系统的限制，仅当元素个数不大于12时才实现了这些特性，这一点以后也许会改进</font>
    * `Clone`、`Copy`
    * `PartialEq`、`Eq`、`PartialOrd`、`Ord`
    * `Debug`、`Default`、`Hash`