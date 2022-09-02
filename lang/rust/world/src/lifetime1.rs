// rust 中每一个引用都有其生命周期，也就是引用保持有效的作用域。
// 大部分时候生命周期是隐含并可以推断的，正如大部分时候类型可以推断一样
// 生命周期的主要目的是避免悬垂引用。
// rust 编译器使用借用检查器来检查生命周期是否有效。

fn fake_main() {
    //// error
    //let r;                       //---------------------------+------------'a
    //{                            //                           +
    //    let x = 5;               //-------+------'b           |
    //    r = &x;                  //       |                   |
    //}                            //-------+                   |
    // r 指向的引用已经被释放掉了，r 变为了悬垂引用                    |
    //println!("r = {}", r);       //                           |
    //                             //                           |
    //println!("Hello, world!");   //                           |
    //                             //---------------------------+
    //
    let r;                       //---------------------------+------------'a
                                 //                           +
    let x = 5;                   //-------+------'b           |
    r = &x;                      //       |                   |
                                 //       |                   |
    println!("r = {}", r);       //       |                   |
                                 //       |                   |
    println!("Hello, world!");   //       |                   |
}                                //---------------------------+
