# any

* `std::any`模块实现了`Any`特性，通过运行时反射对任何`'static`类型实现了动态类型
* `std::any::Any`特性包含`fn get_type_id(&self) -> TypeId`方法，可以取得类型ID
* 此外，`std::any::Any`特性实现了以下方法，可以判断实例代表的是否是某具体类型，或者取得具体类型的引用。
* 而`Box<Any>`类型实现了`downcast`方法，可以将实例转化成`Box<T>`类型

<font color="red">

* 注意：`&Any`可以判断是否是某具体类型，或者取具体类型的引用值；却不能判断类型是否实现了某特性。
* 注意：大部分类型实现了`Any`，大部分类型的引用可以用作`&Any`类型的参数值
* 注意：含有非`'static`引用的类型没有实现`Any`

</font>

```rust
pub fn is<T>(&self) -> bool where T: Any;
pub fn downcast_ref<T>(&self) -> Option<&T> where T: Any;
pub fn downcast_mut<T>(&mut self) -> Option<&mut T> where T: Any;
```

```rust
use std::fmt::Debug;
use std::any::Any;

fn log<T: Any + Debug>(value: &T) {
    let value_any = value as &dyn Any;
    match value_any.downcast_ref::<String>() {
        Some(as_string) => {
            println!("String ({}): {}", as_string.len(), as_string);
        }
        None => {
            println!("{:?}", value);
        }
    }
}

fn do_work<T: Any + Debug>(value: &T) {
    log(value);
}

fn main() {
    let my_string = "Hello World".to_string();
    do_work(&my_string);
    let my_i8: i8 = 100;
    do_work(&my_i8);
}
```