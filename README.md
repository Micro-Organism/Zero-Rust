# Zero-Rust
Rust

## 教程
    Rust 语言是一种高效、可靠的通用高级语言。其高效不仅限于开发效率，它的执行效率也是令人称赞的，是一种少有的兼顾开发效率和执行效率的语言。
    Rust 语言由 Mozilla 开发，最早发布于 2014 年 9 月。Rust 的编译器是在 MIT License 和 Apache License 2.0 双重协议声明下的免费开源软件。
    截至目前( 2020 年 1 月)最新的编译器版本是 1.41.0。
    Rust 官方在线工具: https://play.rust-lang.org/。

## Rust语言的特点
- 高性能 - Rust 速度惊人且内存利用率极高。由于没有运行时和垃圾回收，它能够胜任对性能要求特别高的服务，可以在嵌入式设备上运行，还能轻松和其他语言集成。
- 可靠性 - Rust 丰富的类型系统和所有权模型保证了内存安全和线程安全，让您在编译期就能够消除各种各样的错误。
- 生产力 - Rust 拥有出色的文档、友好的编译器和清晰的错误提示信息， 还集成了一流的工具 —— 包管理器和构建工具， 智能地自动补全和类型检验的多编辑器支持， 以及自动格式化代码等等。

## Rust的应用
Rust 语言可以用于开发：
- 传统命令行程序 - Rust 编译器可以直接生成目标可执行程序，不需要任何解释程序。
- Web 应用 - Rust 可以被编译成 WebAssembly，WebAssembly 是一种 JavaScript 的高效替代品。
- 网络服务器 - Rust 用极低的资源消耗做到安全高效，且具备很强的大规模并发处理能力，十分适合开发普通或极端的服务器程序。
- 嵌入式设备 - Rust 同时具有JavaScript 一般的高效开发语法和 C 语言的执行效率，支持底层平台的开发。

## 环境搭建
    Rust 支持很多的集成开发环境（IDE）或开发专用的文本编辑器。
    官方网站公布支持的工具如下（https://www.rust-lang.org/zh-CN/tools）：
### 搭建 Visual Studio Code 开发环境

### 安装 Rust 编译工具
    在终端输入命令：
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    安装完成后，输入命令：
    rustc --version
    输出：
    rustc 1.41.0 (9b2f00ed2 2019-09-03)

### 安装 Cargo 包管理工具
    在终端输入命令：
    curl https://sh.rustup.rs -sSf | sh
    安装完成后，输入命令：
    cargo --version
    输出：
    cargo 1.41.0 (9b2f00ed 2019-09-03)

## Cargo教程
### Cargo 是什么
- Cargo 是 Rust 的构建系统和包管理器。
- Cargo 负责管理 Rust 项目的构建、依赖管理和代码分发。
- Cargo 的设计目标是简化 Rust 项目的管理和构建过程，使开发者能够更专注于编写代码而不必担心构建系统的细节。
### Cargo 主要特性和功能：
- 依赖管理：Cargo 通过 Cargo.toml 文件管理项目的依赖，这个文件列出了项目所需的所有外部库以及它们的版本。
- 构建系统：Cargo 使用 Rust 编译器（rustc）来构建项目，它会自动处理依赖的编译和链接。
- 包注册表：Cargo 与 crates.io 这个 Rust 社区的包注册表交互，允许开发者搜索、添加和管理第三方库。
- 构建配置：通过 Cargo.toml 和 Cargo.lock 文件，Cargo 允许开发者配置构建选项，如编译器选项、特性（features）和目标平台。
- 项目模板：Cargo 提供了创建新项目的模板，可以通过 cargo new 命令快速启动新项目。
- 测试：Cargo 提供了一个简单的命令 cargo test 来运行项目的单元测试。
- 基准测试：Cargo 支持使用 cargo bench 命令进行基准测试。
- 发布：通过 cargo publish 命令，开发者可以将他们的库发布到 crates.io 上，供其他开发者使用。
- 自定义构建脚本：Cargo 允许使用自定义的构建脚本来处理更复杂的构建需求。
- 多目标项目：Cargo 支持在一个项目中定义多个目标，如可执行文件、库、测试和基准测试。
- 跨平台构建：Cargo 支持跨多个平台构建 Rust 程序，包括 Windows、macOS、Linux 以及各种嵌入式系统。
- 构建缓存：为了加快构建速度，Cargo 使用构建缓存来存储编译后的依赖。
- 离线工作：Cargo 支持在没有互联网连接的情况下工作，它会自动使用本地缓存的依赖。
- 插件系统：Cargo 允许开发者编写插件来扩展其功能。
- 环境变量：Cargo 支持通过环境变量来覆盖默认的构建和运行行为。

### Cargo 功能
    Cargo 除了创建工程以外还具备构建（build）工程、运行（run）工程等一系列功能，构建和运行分别对应以下命令：
    cargo build：构建工程。
    cargo run：运行工程。


## 参考链接
- Rust 官方网站：https://www.rust-lang.org/zh-CN
- Rust 官方文档：https://doc.rust-lang.org/
- Rust Play：https://play.rust-lang.org/
- Visual Studio Code：https://code.visualstudio.com/
- Cargo 官方文档：https://doc.rust-lang.org/cargo/
- 菜鸟教程：https://www.runoob.com/rust/rust-tutorial.html