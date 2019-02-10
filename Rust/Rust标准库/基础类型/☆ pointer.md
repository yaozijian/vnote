
[file:///D:/dev/RustDev/rustup/toolchains/stable-x86_64-pc-windows-gnu/share/doc/rust/html/std/primitive.pointer.html](file:///D:/dev/RustDev/rustup/toolchains/stable-x86_64-pc-windows-gnu/share/doc/rust/html/std/primitive.pointer.html)

# pointer

* 原始的不安全指针，包括`*const T`和`*mut T`。通常不需要使用原始指针。
* 用`std::ptr::null()`和`std::ptr::null_mut()`函数创建空指针

## 1 创建原始指针的方法

### 1.1 直接将引用转化成原始指针

```rust
let my_num: i32 = 10;
let my_num_ptr: *const i32 = &my_num;
let mut my_speed: i32 = 88;
let my_speed_ptr: *mut i32 = &mut my_speed;
```

#### 1.1.1 获取装箱值的指针

```rust
let my_num: Box<i32> = Box::new(10);
let my_num_ptr: *const i32 = &*my_num;// 先解引用，然后再进行引用
let mut my_speed: Box<i32> = Box::new(88);
let my_speed_ptr: *mut i32 = &mut *my_speed;
```

### 1.2 消费装箱值

```rust
let my_speed: Box<i32> = Box::new(88);
let my_speed: *mut i32 = Box::into_raw(my_speed);// 会获取装箱值的所有权
// 已经获取了原来的Box<T>的所有权,必须在使用完成时手动执行资源释放
unsafe {
    drop(Box::from_raw(my_speed));
}
```

### 1.3 从`C`代码获取

```rust
extern crate libc;
use std::mem;
fn main() {
    unsafe {
        // 使用C中的malloc分配内存
        let my_num: *mut i32 = libc::malloc(mem::size_of::<i32>()) as *mut i32;
        if my_num.is_null() {
            panic!("failed to allocate memory");
        }
        // 释放内存
        libc::free(my_num as *mut libc::c_void);
    }
}
```

## 2 常用方法

* `pub fn is_null(self) -> bool`
* `pub unsafe fn as_ref<'a>(self) -> Option<&'a T>`// 转化成引用
* `pub unsafe fn offset(self, count: isize) -> *const T`
* `pub fn wrapping_offset(self, count: isize) -> *const T`
* `pub unsafe fn offset_from(self, origin: *const T) -> isize`
* `pub fn wrapping_offset_from(self, origin: *const T) -> isize`
* `pub unsafe fn add(self, count: usize) -> *const T`
* `pub unsafe fn sub(self, count: usize) -> *const T`
* `pub fn wrapping_add(self, count: usize) -> *const T`
* `pub fn wrapping_sub(self, count: usize) -> *const T`
* `pub unsafe fn read(self) -> T`
* `pub unsafe fn read_volatile(self) -> T`
* `pub unsafe fn read_unaligned(self) -> T`
* `pub unsafe fn copy_to(self, dest: *mut T, count: usize)`
* `pub unsafe fn copy_to_nonoverlapping(self, dest: *mut T, count: usize)`
* `pub fn align_offset(self, align: usize) -> usize`

### 2.1 以下仅对`*mut T`

* `pub unsafe fn as_mut<'a>(self) -> Option<&'a mut T>`
* `pub unsafe fn drop_in_place(self)`
* `pub unsafe fn copy_from(self, src: *const T, count: usize)`
* `pub unsafe fn copy_from_nonoverlapping(self, src: *const T, count: usize)`
* `pub unsafe fn write(self, val: T)`
* `pub unsafe fn write_bytes(self, val: u8, count: usize)`
* `pub unsafe fn write_volatile(self, val: T)`
* `pub unsafe fn write_unaligned(self, val: T)`
* `pub unsafe fn swap(self, with: *mut T)`