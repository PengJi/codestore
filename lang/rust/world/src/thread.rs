// 线程

use std::thread;
use std::time::Duration;

fn fake_main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("num {} in spawn", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // handle.join().unwrap();  // 等待线程执行完成才执行下面的代码

    for i in 1..5 {
        println!("num {} in main", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();  // 等待线程执行完成程序才结束
}
