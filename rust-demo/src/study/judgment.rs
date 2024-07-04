//Rust 条件语句

//Rust 中的条件语句有：if，if let，while，loop，for，match。
pub(crate) fn run() {
    let number = 3;
    // 判断条件 首先，条件表达式 number < 5 不需要用小括号包括（注意，不需要不是不允许）；
    // 但是 Rust 中的 if 不存在单语句不用加 {} 的规则，不允许使用一个语句代替一个块。尽管如此，Rust 还是支持传统 else-if 语法的
    if number < 5 {
        println!("条件为 true");
    } else {
        println!("条件为 false");
    }
}

fn demo01() {
    let a = 12;
    let b;
    if a > 0 {
        b = 1;
    }
    else if a < 0 {
        b = -1;
    }
    else {
        b = 0;
    }
    println!("b is {}", b);
}

fn demo02() {
    let number = 3;
    // if number {   // 报错，expected `bool`, found integerrustc(E0308)
    //     println!("Yes");
    // }
}

fn demo03() {
    let a = 12;
    // if <condition> { block 1 } else { block 2 }
    // Rust 中我们可以使用 if-else 结构实现类似于三元条件运算表达式 (A ? B : C) 的效果：
    let b = if a > 0 { 1 } else { -1 };
    println!("b is {}", b);
}