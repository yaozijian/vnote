
[file:///D:/dev/RustDev/rustup/toolchains/stable-x86_64-pc-windows-gnu/share/doc/rust/html/std/primitive.char.html](file:///D:/dev/RustDev/rustup/toolchains/stable-x86_64-pc-windows-gnu/share/doc/rust/html/std/primitive.char.html)

# char

* `char`代表单个字符，是一个Unicode标量值，与Unicode编码点类似，但不完全相同
* <font color="red">`char`的大小总是4字节，不同于作为字符串一部分的字符</font>

```rust
let v = vec!['h', 'e', 'l', 'l', 'o'];
assert_eq!(20, v.len() * std::mem::size_of::<char>());// 5 * 4 = 20
let s = String::from("hello");
assert_eq!(5, s.len() * std::mem::size_of::<u8>());// 5 * 1 = 5

let s = String::from("love: ❤️");
let v: Vec<char> = s.chars().collect();// 每字符的处理占用更多内存
assert_eq!(12, std::mem::size_of_val(&s[..]));
assert_eq!(32, std::mem::size_of_val(&v[..]));
```

* 人类直觉上的一个字符，可能不等于Unicode定义的字符。

```rust
let mut chars = "é".chars();
// U+00e9: 'latin small letter e with acute'
assert_eq!(Some('\u{00e9}'), chars.next());
assert_eq!(None, chars.next());

let mut chars = "é".chars();
// U+0065: 'latin small letter e'
assert_eq!(Some('\u{0065}'), chars.next());
// U+0301: 'combining acute accent'
assert_eq!(Some('\u{0301}'), chars.next());
assert_eq!(None, chars.next());
```

## `is`系列方法

* `pub fn is_digit(self, radix: u32) -> bool`
* `pub fn is_alphabetic(self) -> bool`
* `pub fn is_lowercase(self) -> bool`
* `pub fn is_uppercase(self) -> bool`
* `pub fn is_whitespace(self) -> bool`
* `pub fn is_alphanumeric(self) -> bool`
* `pub fn is_control(self) -> bool`
* `pub fn is_numeric(self) -> bool`

## `is_ascii`系列方法

* `pub fn is_ascii(&self) -> bool`
* `pub fn is_ascii_alphabetic(&self) -> bool`
* `pub fn is_ascii_uppercase(&self) -> bool`
* `pub fn is_ascii_lowercase(&self) -> bool`
* `pub fn is_ascii_alphanumeric(&self) -> bool`
* `pub fn is_ascii_digit(&self) -> bool`
* `pub fn is_ascii_hexdigit(&self) -> bool`
* `pub fn is_ascii_punctuation(&self) -> bool`
* `pub fn is_ascii_graphic(&self) -> bool`
* `pub fn is_ascii_whitespace(&self) -> bool`
* `pub fn is_ascii_control(&self) -> bool`

## 编码方法

* `pub fn len_utf8(self) -> usize`
* `pub fn len_utf16(self) -> usize`
* `pub fn encode_utf8(self, dst: &mut [u8]) -> &mut str`
* `pub fn encode_utf16(self, dst: &mut [u16]) -> &mut [u16]`
* `pub fn escape_unicode(self) -> EscapeUnicode`
* `pub fn escape_debug(self) -> EscapeDebug`
* `pub fn escape_default(self) -> EscapeDefault`

```rust
for c in '❤'.escape_unicode() {
    print!("{}", c);// \u{2764}
}
println!("{}", '❤'.escape_unicode());
println!("\\u{{2764}}");
assert_eq!('❤'.escape_unicode().to_string(), "\\u{2764}");
```

## 其他方法

* `pub fn to_digit(self, radix: u32) -> Option<u32>`
* `pub fn to_lowercase(self) -> ToLowercase`
* `pub fn to_uppercase(self) -> ToUppercase`
* `pub fn to_ascii_uppercase(&self) -> char`
* `pub fn to_ascii_lowercase(&self) -> char`
* `pub fn eq_ignore_ascii_case(&self, other: &char) -> bool`
* `pub fn make_ascii_uppercase(&mut self)`
* `pub fn make_ascii_lowercase(&mut self)`