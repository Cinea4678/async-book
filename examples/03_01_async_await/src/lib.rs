#![allow(unused)]
#![cfg(test)]

mod async_fn_and_block_examples {
use std::future::Future;
// ANCHOR: async_fn_and_block_examples

// `foo()` 返回一个实现了 `Future<Output = u8>` 的类型。
// `foo().await` 将会得到一个 `u8` 类型的值。
async fn foo() -> u8 { 5 }

fn bar() -> impl Future<Output = u8> {
    // 这个`async`块会生成一个实现了` Future<Output = u8>` 的类型
    async {
        let x: u8 = foo().await;
        x + 5
    }
}
// ANCHOR_END: async_fn_and_block_examples
}

mod async_lifetimes_examples {
use std::future::Future;
// ANCHOR: lifetimes_expanded
// 这个函数:
async fn foo(x: &u8) -> u8 { *x }

// 等同于这个函数:
fn foo_expanded<'a>(x: &'a u8) -> impl Future<Output = u8> + 'a {
    async move { *x }
}
// ANCHOR_END: lifetimes_expanded

async fn borrow_x(x: &u8) -> u8 { *x }

#[cfg(feature = "never_compiled")]
// ANCHOR: static_future_with_borrow
fn bad() -> impl Future<Output = u8> {
    let x = 5;
    borrow_x(&x) // ERROR: `x` does not live long enough
}

fn good() -> impl Future<Output = u8> {
    async {
        let x = 5;
        borrow_x(&x).await
    }
}
// ANCHOR_END: static_future_with_borrow
}

mod async_move_examples {
use std::future::Future;
// ANCHOR: async_move_examples
/// `async` 块:
///
/// 多个不同的 `async` 代码块可以访问同一个局部变量，只要它们是在该变量的作用域内执行的。
async fn blocks() {
    let my_string = "foo".to_string();

    let future_one = async {
        // ...
        println!("{my_string}");
    };

    let future_two = async {
        // ...
        println!("{my_string}");
    };

    // 将两个期物运行至完成，输出了两次“foo”:
    let ((), ()) = futures::join!(future_one, future_two);
}

/// `async move` 块:
///
/// 由于捕获的变量会被移动到由 `async move` 块生成的期物中，因此只有一个 `async move` 
/// 块可以访问被捕获的变量。然而，这使得期物能够超出变量的原始作用域而存活。
fn move_block() -> impl Future<Output = ()> {
    let my_string = "foo".to_string();
    async move {
        // ...
        println!("{my_string}");
    }
}
// ANCHOR_END: async_move_examples
}
