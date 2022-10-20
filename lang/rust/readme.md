## 介绍
Rust 由 Graydon Hoare 在 2008 年私人研发，2009年得到 Mozilla 赞助，2010 年首次发布 0.1.0 版本，用于 Servo 引擎的研发。2015年5月15号 年 Rust 发布 1.0 版本。2021年2⽉9号，Rust 基⾦会宣布成⽴，以致⼒于在全球范围内推⼴和发展 Rust 语⾔。
* 2015-2018 工具、文档、编译器更加智能  
* 2018-2021 异步生态完善  

一门赋予每个人构建可靠且高效软件能力的语言，Rust 原则：
* 可靠性（Realiable）如果它能够编译，它就可以工作。
* 高性能（performant）既高效执行又使用最少内存。
* 生产力（Porductive）让工作事半功倍。
* 便捷性（Supportive）语言、工具和社区随时为用户提供帮助。
* 透明性（Transparent）让用户可以预测和控制底层细节。
* 多样性（Versatile）用 Rust 做任何事。

总的来说，Rust 有三大优势：
1. ⾼性能，Rust 执行速度快且内存利用率高，没有运行时和垃圾回收，能够用于对性能要求高的服务，且可以在嵌入式设备商运行，还能轻松与其他语言集成。
2. 可靠性，Rust 保证了内存安全和线程安全，在编译时就能够避免大多数运行时可能发生的错误。
3. 生产力，Rust 拥有出色的文档、强大的编译器和清晰的错误提示，还提供了方便的包管理工具和构建工具。

## Rust 学习

### 基础
**通用编程概念**  **所有权**  **结构体**  **枚举与模式匹配**
**vector, string, hashmap**  **包，crate, 模块**
**测试**

### 进阶
**错误处理**  **泛型**  **trait**  **生命周期**
**迭代器**  **闭包**  **只能指针**  **线程**  **面向对象**
**高级特征**

### 学习路线
1. [The Rust Programming Language](https://doc.rust-lang.org/book/)
2. [Rust By Example](https://doc.rust-lang.org/rust-by-example/)
3. [Asynchronous Programming in Rust](https://rust-lang.github.io/async-book/)
5. [The Rustonomicon](https://doc.rust-lang.org/nomicon/)
6. 其他：网络编程、实现链表

***

## 安装
`curl https://sh.rustup.rs -sSf | sh`  
`source $HOME/.cargo/env`  
或手动添加环境变量  
`export PATH="$HOME/.cargo/bin:$PATH"`  

## 更新
rustup update

## 卸载
rustup self uninstall

## 显示版本
rustc --version

## cargo 使用
新建项目：`cargo new hello`  
只编译检查，不生成可执行文件：`cargo check`  
调试模式构建：`cargo build`  
发布模式构建：`cargo build --release`  
创建 lib：`cargo new --lib mylib`  
运行：`cargo run`  
打印堆栈：`RUST_BACKTRACE=1 cargo run`  
生成文档（在浏览器中打开当前项目用到的库的文件）：`cargo doc --open`

***

## 参考
[The Rust Programming Language](https://doc.rust-lang.org/book/)  
[Rust By Example](https://doc.rust-lang.org/rust-by-example/)  
[The Cargo Book](https://doc.rust-lang.org/cargo/guide/)
[Asynchronous Programming in Rust](https://rust-lang.github.io/async-book/)    
[The Rustonomicon](https://doc.rust-lang.org/nomicon/)  

[The Rust Programming Language source code](https://nostarch.com/Rust2018)  
[anonymousGiga/learn_rust](https://github.com/anonymousGiga/learn_rust)  
