// 引入所需的依赖库
use std::error::Error;
use tokio::runtime::Runtime;
use reqwest::get;

// 异步函数，用于执行 HTTP GET 请求并返回响应结果
async fn fetch_url(url: &str) -> Result<String, Box<dyn Error>> {
    // 使用 reqwest 发起异步 HTTP GET 请求
    let response = get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

// 异步任务执行函数
async fn execute_async_task() -> Result<(), Box<dyn Error>> {
    // 发起异步 HTTP 请求
    let url = "https://jsonplaceholder.typicode.com/posts/1";
    let result = fetch_url(url).await?;
    // 输出响应结果
    println!("Response: {}", result);
    Ok(())
}

// 主函数
pub(crate) fn run() {
    // 创建异步运行时
    let rt = Runtime::new().unwrap();
    // 在异步运行时中执行异步任务
    let result = rt.block_on(execute_async_task());
    // 处理异步任务执行结果
    match result {
        Ok(_) => println!("Async task executed successfully!"),
        Err(e) => eprintln!("Error: {}", e),
    }
}