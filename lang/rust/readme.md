## 介绍
Rust 由 Graydon Hoare 在 2008 年私人研发，2009年得到 Mozilla 赞助，2010年首次发布 0.1.0 版本，用于Servo 引擎的研发，2015 年 rust 发布 1.0 版本。 
* 2015-2018 工具、文档、编译器更加智能  
* 2018-2021 异步生态完善  

一门赋予每个人构建可靠且高效软件能力的语言，Rust 原则：
* 可靠性（Realiable）如果它能够编译，它就可以工作。
* 高性能（performant）既高效执行有使用最少内存。
* 便捷性（Supportive）语言、工具和社区随时为用户提供帮助。
* 生产力（Porductive）让工作事半功倍。
* 透明性（Transparent）让用户可以预测和控制底层细节。
* 多样性（Versatile）用 Rust 做任何事。


### rust 基础
通用编程概念  
所有权  
结构体  
枚举与模式匹配  
vector、string、hashmap  
包、crate、模块  
测试  

### rust 进阶
错误处理  
泛型  
trait  
生命周期  
迭代器  
闭包  
只能指针  
线程  
面向对象  
高级特征  

## 安装
`curl https://sh.rustup.rs -sSf | sh`  
`source $HOME/.cargo/env`  
或手动添加环境变量  
`export PATH="$HOME/.cargo/bin:$PATH"`  

## 更新版本
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
生成文档：`cargo doc --open`

## 参考
[anonymousGiga/learn_rust](https://github.com/anonymousGiga/learn_rust)
