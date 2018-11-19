# Rust与C指针对比

## `*const T `与 `*mut T`

* 原始指针，通常不应该使用，因为只有unsafe代码才可以对原始指针进行解引用
* 原始指针与C中的指针基本相同，即等同于C中的`struct T *ptr`

## `&T` 与 `&mut T`

* 引用，与原始指针占用同样大小的空间
* 行为与原始指针相同（编译后生成的机器码相同）
* 与原始指针的差别
  * 引用从来不会指向错误的地址（从来不会为NULL或者未初始化)
  * 不能对引用进行随意的指针运算
  * 借用检查器会让你怀疑人生

## `Box<T>`

* 所有权的“指针”，实际上是结构体，含有指向堆上内存的指针
* 堆内存分配和释放是自动进行的
  * `Box::new()`执行堆内存分配
  * 通过`Box<T>`实现的`Drop`特性来释放内存
* Rust：`let x = Box::new(T{...});`
* 创建

```c
struct box_of_T {
	struct T *heap_ptr;
};

struct box_of_t x;

x.heap_ptr = malloc(sizeof(struct T));
if (!x.heap_ptr)
	oom();

*x.heap_ptr = ...;
```

## `&[T]`与`&mut [T]`

* 借用的切片，是使用肥指针(fat pointers)的引用
* 肥指针：包含指针和长度，从而允许运行时进行下标检查

```c
struct fat_pointer_to_T {
	struct T *ptr;
	size_t nelem;
};
```

## `&[T;n]`与`&mut [T;n]`

* 对数组的引用
* 数组长度是在编译时确定的常量，下标检查在编译时发生
* 不需要随指针传递长度，所以不需要使用肥指针

## `T`、`[T;n]`和`[T]`

* `T`等同于C中的结构体
* `[T;n]`等同于C中的数组
* `[T]`
  * 无法创建`[T]`，不确定`[T]`的大小，无法在编译时预留空间
  * 与`Sized`特性有关

## 总结

||Rust|C|32/64位系统中的大小|
|:--|:--|:--|:--|
|类型定义|`struct T {    stuff: [u8; 100]}`|`struct T { uint8_t stuff[100];};`|100/100|
|值|`let x: T;`|`struct T x;`|100/100|
|原始指针|`let x: *const T;let x: *mut T;`|`struct T* x;`|4/8|
|引用|`let x: &T;let x: &mut T;`|`struct T* x;`|4/8|
|Box|`let x: Box<T>;`|`struct box_of_T {   struct T *heap_ptr;};`<br/>`struct box_of_T x;`|4/8|
|2元素数组|`let x: [T;2];`|`struct T x[2];`|200/200|
|2元素数组的引用|`let x: &[T;2];`|`struct T* x;`|4/8|
|切片|`let x: [T];`|`struct T x[];`|编译时大小未知|
|切片的引用|`let x: &[T];`|`struct fat_ptr_to_T {`<br/>`    struct T *ptr;`<br/>`size_t nelem;`<br/>`};`<br/>`struct fat_ptr_to_T x;`|8/16|
