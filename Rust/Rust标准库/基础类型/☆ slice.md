[file:///D:/dev/RustDev/rustup/toolchains/stable-x86_64-pc-windows-gnu/share/doc/rust/html/std/primitive.slice.html](file:///D:/dev/RustDev/rustup/toolchains/stable-x86_64-pc-windows-gnu/share/doc/rust/html/std/primitive.slice.html)

# slice

* 切片是到连续序列`[T]`(数组)的动态大小的视图。
* 切片是到一块由指针和长度表示的内存的视图。
* 切片就是到数组的引用

## 泛型方法：`impl<T> [T]`

* `pub fn len(&self) -> usize`
* `pub fn is_empty(&self) -> bool`
* `pub fn iter(&self) -> Iter<T>`
* `pub fn iter_mut(&mut self) -> IterMut<T>`
* `pub fn contains(&self, x: &T) -> bool where T: PartialEq<T>`// 查找元素
* `pub fn swap(&mut self, a: usize, b: usize)`// 交换两个元素
* `pub fn starts_with(&self, needle: &[T]) -> bool where T: PartialEq<T>`// 前缀判断
* `pub fn ends_with(&self, needle: &[T]) -> bool where T: PartialEq<T>`// 后缀判断
* `pub fn reverse(&mut self)`// 颠倒

* `pub fn into_vec(self: Box<[T]>) -> Vec<T>`// 转化成 Vec<T>
* `pub fn repeat(&self, n: usize) -> Vec<T> where T: Copy`// 重复多次并且转化成 Vec<T>

* `pub fn first(&self) -> Option<&T>`
* `pub fn first_mut(&mut self) -> Option<&mut T>`
* `pub fn last(&self) -> Option<&T>`
* `pub fn last_mut(&mut self) -> Option<&mut T>`

* `pub const fn as_ptr(&self) -> *const T`
* `pub fn as_mut_ptr(&mut self) -> *mut T`

* `pub fn rotate_left(&mut self, mid: usize)`// 循环左移
* `pub fn rotate_right(&mut self, k: usize)`// 循环右移

### 通过下标获取元素

* `pub fn get<I>(&self, index: I) -> Option<&<I as SliceIndex<[T]>>::Output> where I: SliceIndex<[T]>`
* `pub fn get_mut<I>(&mut self,index: I) -> Option<&mut <I as SliceIndex<[T]>>::Output> where I: SliceIndex<[T]>`
* `pub unsafe fn get_unchecked<I>(&self,index: I) -> &<I as SliceIndex<[T]>>::Output where I: SliceIndex<[T]>`
* `pub unsafe fn get_unchecked_mut<I>(&mut self,index: I) -> &mut <I as SliceIndex<[T]>>::Output where I: SliceIndex<[T]>`

### 排序

* `pub fn sort_unstable(&mut self) where T: Ord`
* `pub fn sort_unstable_by<F>(&mut self, compare: F) where F: FnMut(&T, &T) -> Ordering`
* `pub fn sort_unstable_by_key<K, F>(&mut self, f: F) where F: FnMut(&T) -> K,K: Ord`
* `pub fn sort(&mut self) where T: Ord`
* `pub fn sort_by<F>(&mut self, compare: F) where F: FnMut(&T, &T) -> Ordering`
* `pub fn sort_by_key<K, F>(&mut self, f: F) where F: FnMut(&T) -> K,K: Ord`
* `pub fn sort_by_cached_key<K, F>(&mut self, f: F) where F: FnMut(&T) -> K,K: Ord`

### 数据交换

* `pub fn clone_from_slice(&mut self, src: &[T]) where T: Clone`
* `pub fn copy_from_slice(&mut self, src: &[T]) where T: Copy`
* `pub fn copy_within<R>(&mut self, src: R, dest: usize) where R: RangeBounds<usize>,T: Copy`
* `pub fn swap_with_slice(&mut self, other: &mut [T])`
* `pub unsafe fn align_to<U>(&self) -> (&[T], &[U], &[T])`
* `pub unsafe fn align_to_mut<U>(&mut self) -> (&mut [T], &mut [U], &mut [T])`
* `pub fn to_vec(&self) -> Vec<T> where T: Clone`

### 分块

* `pub fn windows(&self, size: usize) -> Windows<T>`
* `pub fn chunks(&self, chunk_size: usize) -> Chunks<T>`
* `pub fn chunks_mut(&mut self, chunk_size: usize) -> ChunksMut<T>`
* `pub fn chunks_exact(&self, chunk_size: usize) -> ChunksExact<T>`
* `pub fn chunks_exact_mut(&mut self, chunk_size: usize) -> ChunksExactMut<T>`
* `pub fn rchunks(&self, chunk_size: usize) -> RChunks<T>`
* `pub fn rchunks_mut(&mut self, chunk_size: usize) -> RChunksMut<T>`
* `pub fn rchunks_exact(&self, chunk_size: usize) -> RChunksExact<T>`
* `pub fn rchunks_exact_mut(&mut self, chunk_size: usize) -> RChunksExactMut<T>`

### 切分

* `pub fn split_first(&self) -> Option<(&T, &[T])>`
* `pub fn split_first_mut(&mut self) -> Option<(&mut T, &mut [T])>`
* `pub fn split_last(&self) -> Option<(&T, &[T])>`
* `pub fn split_last_mut(&mut self) -> Option<(&mut T, &mut [T])>`
* `pub fn split_at(&self, mid: usize) -> (&[T], &[T])`
* `pub fn split_at_mut(&mut self, mid: usize) -> (&mut [T], &mut [T])`
* `pub fn split<F>(&self, pred: F) -> Split<T, F> where F: FnMut(&T) -> bool`
* `pub fn split_mut<F>(&mut self, pred: F) -> SplitMut<T, F> where F: FnMut(&T) -> bool`
* `pub fn rsplit<F>(&self, pred: F) -> RSplit<T, F> where F: FnMut(&T) -> bool`
* `pub fn rsplit_mut<F>(&mut self, pred: F) -> RSplitMut<T, F> where F: FnMut(&T) -> bool`
* `pub fn splitn<F>(&self, n: usize, pred: F) -> SplitN<T, F> where F: FnMut(&T) -> bool`
* `pub fn splitn_mut<F>(&mut self, n: usize, pred: F) -> SplitNMut<T, F> where F: FnMut(&T) -> bool`
* `pub fn rsplitn<F>(&self, n: usize, pred: F) -> RSplitN<T, F> where F: FnMut(&T) -> bool`
* `pub fn rsplitn_mut<F>(&mut self, n: usize, pred: F) -> RSplitNMut<T, F> where F: FnMut(&T) -> bool`

### 二分查找

* `pub fn binary_search(&self, x: &T) -> Result<usize, usize> where T: Ord`
* `pub fn binary_search_by<'a, F>(&'a self, f: F) -> Result<usize, usize> where F: FnMut(&'a T) -> Ordering`
* `pub fn binary_search_by_key<'a, B, F>(&'a self,b: &B,f: F) -> Result<usize, usize> where B: Ord,F: FnMut(&'a T) -> B`

### 分区

* `pub fn partition_dedup(&mut self) -> (&mut [T], &mut [T]) where T: PartialEq<T>`
* `pub fn partition_dedup_by<F>(&mut self, same_bucket: F) -> (&mut [T], &mut [T]) where F: FnMut(&mut T, &mut T) -> bool`
* `pub fn partition_dedup_by_key<K, F>(&mut self, key: F) -> (&mut [T], &mut [T]) where F: FnMut(&mut T) -> K,K: PartialEq<K>`

## 泛型特例化：`impl [u8]`

* `pub fn is_ascii(&self) -> bool`
* `pub fn eq_ignore_ascii_case(&self, other: &[u8]) -> bool`
* `pub fn make_ascii_uppercase(&mut self)`
* `pub fn make_ascii_lowercase(&mut self)`
* `pub fn to_ascii_uppercase(&self) -> Vec<u8>`
* `pub fn to_ascii_lowercase(&self) -> Vec<u8>`