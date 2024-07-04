// Rust 基础语法

// 入口函数
pub(crate) fn run() {

    //变量
    let variable = 1;
    println!("The value of variable is: {}", variable);

    //可变变量
    let variable_mut = 1;
    println!("The value of variable_mut is: {}", variable_mut);

    //常量与不可变变量的区别
    const CONSTANT: i32 = 1;
    println!("The value of constant is: {}", CONSTANT);

    //重影（Shadowing）
    //重影的概念与其他面向对象语言里的"重写"（Override）或"重载"（Overload）是不一样的
    //重影就是指变量的名称可以被重新使用的机制
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    //The value of x is: 12
    println!("The value of x is: {}", x);

    //重影与可变变量的赋值不是一个概念，重影是指用同一个名字重新代表另一个变量实体，其类型、可变属性和值都可以变化。但可变变量赋值仅能发生值的变化
    let mut s = "123";
    // s = s.len();    //这段程序会出错：不能给字符串变量赋整型值。
    println!("The value of s is: {}", s);

}