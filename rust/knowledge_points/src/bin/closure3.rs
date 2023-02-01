// 闭包捕获环境变量
// 闭包可以通过三种方式捕获其环境，它们对应函数的三种获取参数的方式，
// 分别是获取所有权、可变借用、不可变借用。
// 这三种捕获的方式被编码为如下三个 Fn trait：
// （1）FnOnce 消费从周围作用域捕获的变量，闭包周围的作用域被称为其环境。 
// 为了消费捕获到的变量，闭包必须获取其所有权并在定义闭包时将其移进闭包。
// 其名称的 Once 部分代表了闭包不能多次获取相同变量的所有权。
// （2）FnMut 获取可变的借用值，所以可以改变其环境。
// （3）Fn 从其环境获取不可变的借用值
//
// 当创建一个闭包时，rust 会根据其如何使用环境中的变量来推断我们希望如何引用环境。 
// 由于所有闭包都可以被调用至少一次，因此所有闭包都实现了 FnOnce。
// 没有移动被捕获变量的所有权到闭包的闭包也实现了 FnMut。
// 不需要对捕获的变量进行可变访问的闭包实现了 Fn。

fn main() {
    // let x = 3;
    // let equal_to_x = |n| {n == x};
    // ley y = 3;
    // assert!(equal_to_x(y));

    let x = vec![1, 2, 3];
    let equal_to_x = move |n| {n == x};  // 移动所有权
    println!("x == {:?}", x);  // error，所有权已经转移

    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
}