// Rust 数据类型

//! 数据类型
pub(crate) fn run() {
    //整数型（Integer）
    let integer = 1;
    println!("The value of integer is: {}", integer);

    //浮点数型（Float）
    let float = 2.5;
    println!("The value of float is: {}", float);

    //布尔型（Boolean）
    let boolean = true;
    println!("The value of boolean is: {}", boolean);

    //字符型（Char）
    let character = 'a';
    println!("The value of character is: {}", character);

    //复合型（Compound）
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // tup.0 等于 500
    // tup.1 等于 6.4
    // tup.2 等于 1
    let (x, y, z) = tup;
    // y 等于 6.4

    //数组型（Array）

    //元组型（Tuple）

    //指针型（Pointer）

    //函数型（Function）

    //其他类型（Other）
    //数组用一对 [ ] 包括的同类型数据
    let mut a = [1, 2, 3, 4, 5]; // a 是一个长度为 5 的整型数组
    let b = ["January", "February", "March"]; // b 是一个长度为 3 的字符串数组
    let c: [i32; 5] = [1, 2, 3, 4, 5]; // c 是一个长度为 5 的 i32 数组
    let d = [3; 5]; // 等同于 let d = [3, 3, 3, 3, 3];
    let first = a[0];
    let second = a[1];
    // 数组访问
    a[0] = 123; // 错误：数组 a 不可变
    let mut a = [1, 2, 3];
    a[0] = 4; // 正确


    //空类型（Void）

    //单元类型（Unit）

    //动态类型（Dynamic）

    //错误类型（Error）

    //特殊类型（Special）

    //引用类型（Reference）

    //可变引用类型（Mutable Reference）


    //数学运算
    let sum = 5 + 10; // 加
    let difference = 95.5 - 4.3; // 减
    let product = 4 * 30; // 乘
    let quotient = 56.7 / 32.2; // 除
    let remainder = 43 % 5; // 求余
}