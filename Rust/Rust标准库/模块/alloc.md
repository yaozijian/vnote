# alloc

* 每个程序中，标准库将一个全局内存分配器用于`Box<T>`和`Vec<T>`等需要在堆上分配内存的地方。
* 对于程序，当前未指定默认的全局内存分配器；`cdylibs`和`staticlibs`之类的库，默认使用`System`内存分配器。
* 可以用`#[global_allocator]`属性指定全局内存分配器。

```rust
use std::alloc::{GlobalAlloc, System, Layout};

struct MyAllocator;

// 实现内存分配器
unsafe impl GlobalAlloc for MyAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        System.alloc(layout)
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout)
    }
}

// 定义定制的内存分配器为默认的全局内存分配器
#[global_allocator]
static GLOBAL: MyAllocator = MyAllocator;

fn main() {
    // 将使用上面定义的GLOBAL内存分配器
    let mut v = Vec::new();
    v.push(1);
}
```

* 内存分配器可以由外部库提供

```rust
extern crate jemallocator;

use jemallocator::Jemalloc;

#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

fn main() {}
```