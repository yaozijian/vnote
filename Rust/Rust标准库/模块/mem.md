# mem

* `const fn align_of<T>() -> usize`
* `const fn align_of_val<T>(val: &T) -> usize where T: ?Sized`
* `const fn size_of<T>() -> usize`
* `fn size_of_val<T>(val:&T) -> usize where T: ?Sized`
* `fn discriminant<T>(v:&T)->Discriminant<T>`// 比较唯一表示枚举类型的值
* `fn drop<T>(_x: T)`// 主动丢弃值
* `fn forget<T>(t: T)`// 主动忘记值
* `fn replace<T>(dest:&mut T,src: T) -> T`// 交换值
* `fn swap<T>(x:&mut T,y: &mut T)`

```rust
let mut v: Vec<i32> = vec![1,2];
let old = std::mem::replace(&mut v,vec![3,4,5]);
assert_eq!(old.len(),2);
assert_eq!(v.len(),3);
```

## 重新解释

* `unsafe extern "rust-intrinsic" fn transmute<T,U>(e: T) -> U`
* `unsafe fn transmute_copy<T,U>(src:&T) -> U`
* 按比特重新解释一种类型的值为另一种类型的值
* 两种类型必须大小相同，但不必是有效的值
* 类似C语言中的memcpy