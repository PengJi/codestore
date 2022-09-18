// 调用不安全的函数或者方法

unsafe fn dangerously() {
    println!("do something...");
}

fn foo() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("*r1 = {}", *r1);
        println!("*r2 = {}", *r2);
    }
}

fn fake_main() {
    unsafe {
        dangerous();
    }
    // dangerous();  // error

    foo();
}