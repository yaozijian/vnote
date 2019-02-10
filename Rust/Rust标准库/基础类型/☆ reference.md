[file:///D:/dev/RustDev/rustup/toolchains/stable-x86_64-pc-windows-gnu/share/doc/rust/html/std/primitive.reference.html](file:///D:/dev/RustDev/rustup/toolchains/stable-x86_64-pc-windows-gnu/share/doc/rust/html/std/primitive.reference.html)

# 引用

* 引用代表对值的借用：通过`&`或者`&mut`运算符获取；或者通过`ref`或者`ref mut`模式获取。
* 可以说引用就是不允许为空的指针。<font color="red">实际上，`Option<&T>`的内存表示与可以为空的指针相同，可以跨越`FFI`边界进行传递。</font>
* <font color="red">多数情况下，引用可以像原始值那样使用：字段访问、方法调用、索引都与原始值相同。</font>
* 引用有关联的生命周期。如果一个生命周期与另一个同样长，或者更长，则说这个生命周期更“长寿（outlive）”。
	* `'static`代表整个程序生存期间，是最长寿的生命周期。
* `&mut T`可以自由地转化成`&T`；具有更长生命周期的引用可以自由地转化成具有较短生命周期的引用。

<font color="red">

* 引用的比较
	* 比较运算符（等号）透明地对被引用的值进行比较。
	* `std::ptr::eq`函数比较引用是否指向相同的值，即比较指针地址是否相等

</font>

## 实现的特性

* 对于`&T`，无论被引用的类型是什么，都实现了以下特性
	* `std::marker::Copy`
	* `std::clone::Clone`：注意：即使`T`实现了`Clone`，也不等同于`T`的`Clone`实现
	* `std::ops::Deref`
	* `std::borrow::Borrow`
	* `std::fmt::Pointer`
* 对于`&mut T`，实现了上述后三个特性（`Deref`、`Borrow`、`Pointer`），以及以下特性
	* `std::ops::DerefMut`
	* `std::borrow::BorrowMut`
* 如果被引用的类型`T`实现了下述特性，则`&T`和`&mut T`也实现了下述特性
	* `std::fmt`模块定义的所有特性，除`std::fmt::Pointer`和`std::fmt::Write`之外
	* `std::cmp::PartialOrd`
	* `std::cmp::Ord`
	* `std::cmp::PartialEq`
	* `std::cmp::Eq`
	* `std::convert::AsRef`
	* `std::ops::Fn`
	* `std::hash::Hash`
	* `std::net::ToSocketAddrs`(仅`&T`实现了)
* 如果被引用的类型`T`实现了下述特性，则`&mut T`也实现了下述特性
	* `std::convert::AsMut`
	* `std::ops::FnMut`
	* `std::fmt::Write`
	* `std::iter::Iterator`
	* `std::iter::DoubleEndedIterator`
	* `std::iter::ExactSizeIterator`
	* `std::iter::FusedIterator`
	* `std::iter::TrustedLen`
	* `std::marker::Send`
	* `std::io::Write`
	* `std::io::Read`
	* `std::io::Seek`
	* `std::io::BufRead`
