//! 迭代器

use std::iter::Peekable;

pub(crate) fn run() {
    //使用 iter() 方法创建可变引用的迭代器：
    let vec = vec![1, 2, 3, 4, 5];
    let iter = vec.iter();

    //使用 iter_mut() 方法创建可变引用的迭代器：
    let mut vec = vec![1, 2, 3, 4, 5];
    let iter_mut = vec.iter_mut();

    //使用 into_iter() 方法创建获取所有权的迭代器：
    let vec = vec![1, 2, 3, 4, 5];
    let into_iter = vec.into_iter();

    //使用 map() 方法对每个元素进行转换
    let vec = vec![1, 2, 3, 4, 5];
    let squared_vec: Vec<i32> = vec.iter().map(|x| x * x).collect();

    //使用 filter() 方法过滤元素
    let vec = vec![1, 2, 3, 4, 5];
    let filtered_vec: Vec<i32> = vec.iter().filter(|x| x % 2 == 0).collect();

    //使用 for 循环遍历迭代器
    let vec = vec![1, 2, 3, 4, 5];
    for &num in vec.iter() {
        println!("{}", num);
    }

    //消费迭代器
    let arr = vec![1, 2, 3];
    let mut iter = arr.into_iter();
    while let Some(val) = iter.next() {
        println!("{}", val);
    }

    //适配器
    let arr = [1, 2, 3, 4, 5];
    let even_numbers: Vec<_> = arr.into_iter().filter(|&x| x % 2 == 0).collect();

    //迭代器链
    let arr = [1, 2, 3, 4, 5];
    let mut iter = arr.into_iter().peekable();
    while let Some(val) = iter.next() {
        if val % 2 == 0 {
            continue;
        }
        println!("{}", val);
    }

    //收集器
    let arr = [1, 2, 3, 4, 5];
    let sum: i32 = arr.into_iter().sum();
}

fn demo01() {
    // 定义一个包含整数的数组
    let numbers = vec![1, 2, 3, 4, 5];

    // 使用迭代器对数组进行遍历，并输出每个元素
    println!("Iterating through the array:");
    for num in numbers.iter() {
        println!("{}", num);
    }

    // 使用迭代器的 map 方法对数组中的每个元素进行平方运算，并收集结果到一个新的数组中
    let squared_numbers: Vec<i32> = numbers.iter().map(|x| x * x).collect();

    // 输出平方后的数组
    println!("Squared numbers: {:?}", squared_numbers);
}

fn demo02() {
    // 定义一个包含整数的数组
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // 使用迭代器的 filter 方法对数组进行过滤，筛选出偶数
    let even_numbers: Vec<i32> = numbers.iter().filter(|&x| x % 2 == 0).cloned().collect();

    // 输出筛选后的结果
    println!("Even numbers: {:?}", even_numbers);
}