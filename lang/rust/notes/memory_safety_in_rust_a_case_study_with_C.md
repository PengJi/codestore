先看一段程序，这段程序有 7 个 bug，我们逐个分析一下。
```c
#include <stdio.h>
#include <stdlib.h>
#include <assert.h>

// There are at least 7 bugs relating to memory on this snippet.
// Find them all!

// Vec is short for "vector", a common term for a resizable array.
// For simplicity, our vector type can only hold ints.
typedef struct {
    int* data;     // Pointer to our array on the heap
    int  length;   // How many elements are in our array
    int  capacity; // How many elements our array can hold
} Vec;

// <bug 1>
// 下面的函数返回一个悬垂指针。
// 该函数返回一个执行该 Vec 的指针，当该函数返回时，指针指向的内容（位于栈上）被释放，所以后续对该指针的使用将会异常。
// 修复的方法是：一使用 malloc(sizeof(Vec)) 将变量内容分配在堆上；二将类型签名更改为返回结构本身，而不是指针。
Vec* vec_new() {
    Vec vec;  // vec 被分配在栈上
    vec.data = NULL;
    vec.length = 0;
    // <bug 2>
    // 初始化容量为0，当调用 vec_push 时，容量将会翻倍但 2*0=0，导致没有额外的内存空间被分配.
    // 因此需要预先分配至少一个元素的空间。
    vec.capacity = 0;
    return &vec;
}

void vec_push(Vec* vec, int n) {
    if (vec->length == vec->capacity) {
        int new_capacity = vec->capacity * 2;
        // <bug 3>
        // malloc 的参数是要分配的内存大小(以字节为单位)。
        // 然而 new_capacity 只是整型的容量。
        // 修复：malloc(sizeof(int) * new_capacity)。
        int* new_data = (int*) malloc(new_capacity);
        assert(new_data != NULL);

        for (int i = 0; i < vec->length; ++i) {
            new_data[i] = vec->data[i];
        }

        // <bug 4>
        // 在 vec->data 调整大小时，没有 free 旧数据指针，将会导致内容泄露。
        vec->data = new_data;
        vec->capacity = new_capacity;
    }

    vec->data[vec->length] = n;
    ++vec->length;
}

// <bug 5>
// free 的顺序不正确。
// 在释放 vector 之后，vec->data 指针就不可用。
// 我们应该先释放 vec->data，然后释放 vector。
void vec_free(Vec* vec) {
    free(vec);
    free(vec->data);
}

void main() {
    Vec* vec = vec_new();
    vec_push(vec, 107);

    // <bug 7>
    // 迭代器 n 失效，这个 bug 不易发现。
    // 首先取一个指向 vector 第一个元素的指针，然而再调用 vec_push 之后，将会发生 resize，
    // 释放旧的数据再分配新的 array。因此 n 是一个悬垂指针，在 printf 中解引用它是内存不安全的。
    // 这是一个称为迭代器失效的常见问题的特殊情况，即当容器被修改时，指向容器的指针将失效。
    int* n = &vec->data[0];
    vec_push(vec, 110);
    printf("%d\n", *n);

    // <bug 6>
    // vec->data 被 free 两次。
    // vec_free 中已经执了一次 free(vec->data)，但这里又被 free 了一次。
    // 修复方式是这里只使用 vec_free。
    free(vec->data);
    vec_free(vec);
}
```
上述程序有 7 个 bug，但这段代码依然可以通过编译（尽管有一些警告），编译后运行该程序将会发生 `Segmentation fault (core dumped)`。
接下来我们使用 rust 实现相同的代码。

```rust
// 使用 Vec2 避免与 std::vec:Vec 冲突。
struct Vec2 {
    data: Box<[isize]>,
    length: usize,
    capacity: usize
}

impl Vec2 {
    fn new() -> &Vec2 {
        let v = Vec2 {
            data: Box::new([]),
            length: 0,
            capacity: 0
        };
        return &v;
    }
}

fn main () {}
```
如果仿照 C 代码来写 rust，将会编译失败。
```
error[E0106]: missing lifetime specifier
 --> src/main.rs:8:17
  |
8 |     fn new() -> &Vec2 {
  |                 ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
  |
8 |     fn new() -> &'static Vec2 {
  |                 ~~~~~~~~

For more information about this error, try `rustc --explain E0106`.
```

Rust 可以识别悬垂指针问题，甚至不需要查看函数实现而只需分析函数签名。
上面的错误信息提示：函数的返回了借用的值，但没有值可借用，需要使用 'static 静态生命周期申明。
修复代码，修改函数签名，返回拥有所有权的 vector。
```rust
impl Vec2 {
    fn new() -> Vec2 {
        let v = Vec2 {
            data: Box::new([0]),
            length: 0,
            capacity: 1
        };
        return v;
    }
}
```
需要注意的是，容量问题不会被编译器捕捉到，它是一个逻辑错误，由程序员来识别。
也就是说，如果我们不修复这个bug，那么错误至少会是一个显式的越界数组错误，而不是访问越界内存的段错误。
接下来，我们实现push方法:
```rust
fn push(&mut self, n: isize) {
    if self.length == self.capacity {
        let new_capacity = self.capacity * 2;
        let mut new_data = unsafe {
            let ptr = Heap::default()
                .alloc(Layout::array::<isize>(new_capacity).unwrap())
                .unwrap() as *mut isize;
            Box::from_raw(slice::from_raw_parts_mut(ptr, new_capacity))
        };

        for i in 0..self.length {
            new_data[i] = self.data[i];
        }

        self.data = new_data;
        self.capacity = new_capacity;
    }

    self.data[self.length] = n;
    self.length += 1;
}
```
上述方法可以正确编译和工作，它不包含显式的 `free(self.data)`，因为 rust 会自动释放 `self.data` 的旧值。
实现自动释放是基于 rust 的生命周期，它确定旧数组的生命周期在在变量重新分配时结束。
由于程序员不必显式地释放分配的内存，这消除了相关的内存泄露和两次 free 的问题。

上述的内存分配方式在 rust 中是非惯用的。基本上，内存分配要么通过声明一个值隐式地分配在栈上，
要么使用 Box 或从它派生的任何指针类型显式地分配在堆上。通过这些接口，rust 自动分配适当大小和对齐的内容。
例如：
```rust
struct Point { x: f32, y: f32 }
let p: Box<Point> = Box::new(Point{ x: 0.1, y: 0.2 });
```
Rust 决定 Point 的大小，在后台执行 malloc(sizeof(Point))。

