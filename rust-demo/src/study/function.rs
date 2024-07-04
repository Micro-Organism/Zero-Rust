//Rust 函数

pub(crate) fn run() {
    println!("Hello, world!");
    another_function01();

    another_function02(5, 6);
}

// 没有返回值的函数
fn another_function01() {
    println!("Hello, runoob!");
}

// 带有返回值的函数
fn another_function02(x: i32, y: i32) {
    println!("x 的值为 : {}", x);
    println!("y 的值为 : {}", y);
}

// 函数返回一个 i32 类型的值
fn another_function03() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("x 的值为 : {}", x);
    println!("y 的值为 : {}", y);
}

// 嵌套函数
fn another_function04() {
    fn five() -> i32 {
        5
    }
    println!("five() 的值为: {}", five());
}

// 函数返回值
fn another_function05(a: i32, b: i32) -> i32 {
    return a + b;
}