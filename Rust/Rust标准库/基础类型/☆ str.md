
[file:///D:/dev/RustDev/rustup/toolchains/stable-x86_64-pc-windows-gnu/share/doc/rust/html/std/primitive.str.html](file:///D:/dev/RustDev/rustup/toolchains/stable-x86_64-pc-windows-gnu/share/doc/rust/html/std/primitive.str.html)

# str

* `str`是最原始的字符串类型，通常以借用的形式出现，即`&str`
* 字面字符串的类型是`&'static str`
* `str`总是有效的`UTF-8`

## 最常用

* `pub fn len(&self) -> usize`: 字节长度
* `pub fn is_empty(&self) -> bool`: 判断是否为空
* `pub fn is_char_boundary(&self, index: usize) -> bool`：判断指定位置处是否是字符边界，索引等于长度时认为是字符边界
* `pub fn contains<'a, P>(&'a self, pat: P) -> bool where P: Pattern<'a>`// 判断是否包含子串
* `pub fn starts_with<'a, P>(&'a self, pat: P) -> bool where P: Pattern<'a>`// 前缀判断
* `pub fn ends_with<'a, P>(&'a self, pat: P) -> bool where P: Pattern<'a>,<P as Pattern<'a>>::Searcher: ReverseSearcher<'a>` // 后缀判断
* `pub fn find<'a, P>(&'a self, pat: P) -> Option<usize> where P: Pattern<'a>`// 找子串位置
* `pub fn rfind<'a, P>(&'a self, pat: P) -> Option<usize> where P: Pattern<'a>,<P as Pattern<'a>>::Searcher: ReverseSearcher<'a>`// 反向查找子串
* `pub fn replace<'a, P>(&'a self, from: P, to: &str) -> String where P: Pattern<'a>`// 替换指定的模式，存储结果到新分配的内存中返回，不修改原字符串
* `pub fn replacen<'a, P>(&'a self, pat: P, to: &str, count: usize) -> String where P: Pattern<'a>`
* `pub fn is_ascii(&self) -> bool`
* `pub fn eq_ignore_ascii_case(&self, other: &str) -> bool`
* `pub fn make_ascii_uppercase(&mut self)`
* `pub fn make_ascii_lowercase(&mut self)`
* `pub fn into_boxed_bytes(self: Box<str>) -> Box<[u8]>`

## `String`系列

* `pub fn to_lowercase(&self) -> String`
* `pub fn to_uppercase(&self) -> String`
* `pub fn escape_debug(&self) -> String`
* `pub fn escape_default(&self) -> String`
* `pub fn escape_unicode(&self) -> String`
* `pub fn into_string(self: Box<str>) -> String`
* `pub fn repeat(&self, n: usize) -> String`
* `pub fn to_ascii_uppercase(&self) -> String`
* `pub fn to_ascii_lowercase(&self) -> String`

## `trim`系列

* `pub fn trim(&self) -> &str`
* `pub fn trim_start(&self) -> &str`
* `pub fn trim_end(&self) -> &str`
* `pub fn trim_matches<'a, P>(&'a self, pat: P) -> &'a str where P: Pattern<'a>,<P as Pattern<'a>>::Searcher: DoubleEndedSearcher<'a>`
* `pub fn trim_start_matches<'a, P>(&'a self, pat: P) -> &'a str where P: Pattern<'a>`
* `pub fn trim_end_matches<'a, P>(&'a self, pat: P) -> &'a str where P: Pattern<'a>,<P as Pattern<'a>>::Searcher: ReverseSearcher<'a>`

## 元素迭代器

* `pub fn as_bytes(&self) -> &[u8]`
* `pub unsafe fn as_bytes_mut(&mut self) -> &mut [u8]`
* `pub fn as_ptr(&self) -> *const u8`
* `pub fn chars(&self) -> Chars` // 变成字符迭代器
* `pub fn char_indices(&self) -> CharIndices` // 变成位置和字符迭代器
* `pub fn bytes(&self) -> Bytes` // 变成字节迭代器
* `pub fn encode_utf16(&self) -> EncodeUtf16` // 变成u16迭代器

```rust
fn main() {
    let xyz = "China中国";
    assert_eq!(11,xyz.len());
    
    let mut def = xyz.chars();
    assert_eq!(Some('C'),def.next());
    assert_eq!(Some('h'),def.next());
    assert_eq!(Some('i'),def.next());
    assert_eq!(Some('n'),def.next());
    assert_eq!(Some('a'),def.next());
    assert_eq!(Some('中'),def.next());
    assert_eq!(Some('国'),def.next());
    assert_eq!(None,def.next());
    
    let mut def = xyz.char_indices();
    assert_eq!(Some((0,'C')),def.next());
    assert_eq!(Some((1,'h')),def.next());
    assert_eq!(Some((2,'i')),def.next());
    assert_eq!(Some((3,'n')),def.next());
    assert_eq!(Some((4,'a')),def.next());
    assert_eq!(Some((5,'中')),def.next());
    assert_eq!(Some((8,'国')),def.next());
    assert_eq!(None,def.next());
}
```

## 取子串

* `pub fn get<I>(&self, i: I) -> Option<&<I as SliceIndex<str>>::Output> where I: SliceIndex<str>`
* `pub fn get_mut<I>(&mut self, i: I) -> Option<&mut <I as SliceIndex<str>>::Output> where I: SliceIndex<str>`
* `pub unsafe fn get_unchecked<I>(&self, i: I) -> &<I as SliceIndex<str>>::Output where I: SliceIndex<str>`
* `pub unsafe fn get_unchecked_mut<I>(&mut self, i: I) -> &mut <I as SliceIndex<str>>::Output where I: SliceIndex<str>`

```rust
fn main() {
    let mut xyz = "China中国".to_string();
    let xyz = xyz.as_mut_str();// 无法直接将字面字符串变成&mut str，只有通过String转化
    assert_eq!(xyz.get(6..),None);// 索引不在字符边界上
    assert_eq!(xyz.get(20..25),None);// 索引超出范围
    let def = xyz.get_mut(..);// def 的类型是 Option<&mut str>
    let def = def.map(|c|{c.make_ascii_uppercase();&*c});// 这里c的类型是&mut str,必须用&*c变成&str
    assert_eq!(def,Some("CHINA中国"));
}
```

## 切分

* `pub fn split_at(&self, mid: usize) -> (&str, &str)`// 根据索引位置切分
* `pub fn split_at_mut(&mut self, mid: usize) -> (&mut str, &mut str)`// 根据索引位置切分
* `pub fn split_whitespace(&self) -> SplitWhitespace` // 根据Unicode空白字符切分成多个子串
* `pub fn split_ascii_whitespace(&self) -> SplitAsciiWhitespace`// 根据ASCII空白字符切分成多个子串
* `pub fn lines(&self) -> Lines`// 切分成行
* `pub fn split<'a, P>(&'a self, pat: P) -> Split<'a, P> where P: Pattern<'a>`// 根据给定的字符串切分
* `pub fn rsplit<'a, P>(&'a self, pat: P) -> RSplit<'a, P> where P: Pattern<'a>,<P as Pattern<'a>>::Searcher: ReverseSearcher<'a>,`// 与split相同,但返回的元素是反向的
* `pub fn split_terminator<'a, P>(&'a self, pat: P) -> SplitTerminator<'a, P> where P: Pattern<'a>`// 同 split，但是，如果字符串以给定模式结束，最后不返回一个空白
* `pub fn rsplit_terminator<'a, P>(&'a self, pat: P) -> RSplitTerminator<'a, P> where P: Pattern<'a>,<P as Pattern<'a>>::Searcher: ReverseSearcher<'a>`
* `pub fn splitn<'a, P>(&'a self, n: usize, pat: P) -> SplitN<'a, P> where P: Pattern<'a>`// 同split，但是至多返回n个元素
* `pub fn rsplitn<'a, P>(&'a self, n: usize, pat: P) -> RSplitN<'a, P> where P: Pattern<'a>,<P as Pattern<'a>>::Searcher: ReverseSearcher<'a>`

```rust
let v: Vec<&str> = "Mary had a little lamb".split(' ').collect();
assert_eq!(v, ["Mary", "had", "a", "little", "lamb"]);

let v: Vec<&str> = "".split('X').collect();
assert_eq!(v, [""]);

let d: Vec<_> = "010".split("0").collect();
assert_eq!(d, &["", "1", ""]);// 注意: 切分模式左右作为空白返回

let f: Vec<_> = "rust".split("").collect();
assert_eq!(f, &["", "r", "u", "s", "t", ""]);// 注意：空白模式的效果

let v: Vec<&str> = "lionXXtigerXleopard".split('X').collect();
assert_eq!(v, ["lion", "", "tiger", "leopard"]);// 注意: 认为两个连续的X之间存在空白

let v: Vec<&str> = "lion::tiger::leopard".split("::").collect();
assert_eq!(v, ["lion", "tiger", "leopard"]);

let v: Vec<&str> = "abc1defXghi".split(|c| c == '1' || c == 'X').collect();// 使用闭包作为切分模式
assert_eq!(v, ["abc", "def", "ghi"]);
```

## 匹配

* `pub fn matches<'a, P>(&'a self, pat: P) -> Matches<'a, P> where P: Pattern<'a>`// 返回匹配的部分
* `pub fn rmatches<'a, P>(&'a self, pat: P) -> RMatches<'a, P> where P: Pattern<'a>,<P as Pattern<'a>>::Searcher: ReverseSearcher<'a>`
* `pub fn match_indices<'a, P>(&'a self, pat: P) -> MatchIndices<'a, P> where P: Pattern<'a>`// 返回索引位置和匹配的部分：(索引,匹配)
* `pub fn rmatch_indices<'a, P>(&'a self, pat: P) -> RMatchIndices<'a, P> where P: Pattern<'a>,<P as Pattern<'a>>::Searcher: ReverseSearcher<'a>`

## 字符串模式

* `str`的很多方法带一个`pat : P where P : Pattern`参数，这里的`Pattern`的完全路径是`std::str::pattern::Pattern`，表示一个字符串模式，其定义为

```rust
pub trait Pattern<'a> {
    type Searcher: Searcher<'a>;
    fn into_searcher(self, haystack: &'a str) -> Self::Searcher;// 仅这一个方法必须实现
    // 下面三个方法有默认实现
    fn is_contained_in(self, haystack: &'a str) -> bool { ... }
    fn is_prefix_of(self, haystack: &'a str) -> bool { ... }
    fn is_suffix_of(self, haystack: &'a str) -> bool where Self::Searcher: ReverseSearcher<'a> { ... }
}
```

* 关联类型被限定为必须实现了`std::str::pattern::Searcher`的类型，这个特性定义为：

```rust
pub unsafe trait Searcher<'a> {
    fn haystack(&self) -> &'a str;// 返回被搜索的字符串
    fn next(&mut self) -> SearchStep;// 返回下个匹配/拒绝/完成结果
    fn next_match(&mut self) -> Option<(usize, usize)> { ... }
    fn next_reject(&mut self) -> Option<(usize, usize)> { ... }
}
```

* 标准库为`char`、`&str`、`&String`、`&[char]`、`&&str`实现了该特性，即这些类型的数据可以用作模式参数
* 标准库也包含`impl<'a, F> Pattern<'a> for F where F: FnMut(char) -> bool`，所以函数、闭包也可以作为模式