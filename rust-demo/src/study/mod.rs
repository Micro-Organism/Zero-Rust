pub(crate) mod mysql;
pub(crate) mod env;
pub(crate) mod cargo;
pub(crate) mod command;
pub(crate) mod basic;
pub(crate) mod data_type;
pub(crate) mod comment;
pub(crate) mod function;
pub(crate) mod judgment;
pub(crate) mod circulate;
pub(crate) mod iterator;
pub(crate) mod closure;
pub(crate) mod ownership;
pub(crate) mod slice;
pub(crate) mod r#struct;
pub(crate) mod r#enum;
pub(crate) mod module;
pub(crate) mod error;
pub(crate) mod generic;
pub(crate) mod life;
pub(crate) mod io;
pub(crate) mod collection;
pub(crate) mod oop;
pub(crate) mod concurrent;
pub(crate) mod macros;
pub(crate) mod pointer;
pub(crate) mod asynchronous;

pub(crate) fn run() {

    // 调用 env 模块
    // env::run();

    // 调用 cargo 模块
    // cargo::run();

    // 调用 command 模块
    // command::run();

    // 调用 basic 模块
    // basic::run();

    // 调用 data_type 模块
    // data_type::run();

    // 调用 comment 模块
    // comment::run();

    // 调用 function 模块
    // function::run();

    // 调用 judgment 模块
    // judgment::run();

    // 调用 circulate 模块
    // circulate::run();

    // 调用 iterator 模块
    iterator::run();

    // 调用 closure 模块
    // closure::run();

    // 调用 ownership 模块
    // ownership::run();

    // 调用 slice 模块
    // slice::run();

    // 调用 r#struct 模块
    // r#struct::run();

    // 调用 r#enum 模块
    // r#enum::run();

    // 调用 module 模块
    // module::run();

    // 调用 error 模块
    // error::run();

    // 调用 generic 模块
    // generic::run();

    // 调用 life 模块
    // life::run();

    // 调用 io 模块
    // io::run();

    // 调用 collection 模块
    // collection::run();

    // 调用 oop 模块
    // oop::run();

    // 调用 concurrent 模块
    // concurrent::run();

    // 调用 macros 模块
    // macros::run();

    // 调用 pointer 模块
    // pointer::run();

    // 调用 asynchronous 模块
    // // 案例001
    // asynchronous::run01();
    // // 案例002
    // asynchronous::run01();

    // 调用 mysql 模块
    // mysql::run();
}