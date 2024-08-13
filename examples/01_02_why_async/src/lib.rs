#![cfg(test)]

use futures::{executor::block_on, join};
use std::thread;

fn download(_url: &str) {
    // ...
}

#[test]
// ANCHOR: get_two_sites
fn get_two_sites() {
    // 生成两个线程来执行任务
    let thread_one = thread::spawn(|| download("https://www.foo.com"));
    let thread_two = thread::spawn(|| download("https://www.bar.com"));

    // 等待两个线程都结束
    thread_one.join().expect("thread one panicked");
    thread_two.join().expect("thread two panicked");
}
// ANCHOR_END: get_two_sites

async fn download_async(_url: &str) {
    // ...
}

// ANCHOR: get_two_sites_async
async fn get_two_sites_async() {
    // 创建两个不同的“期物”，这些期物在完成时将异步地下载网页。
    let future_one = download_async("https://www.foo.com");
    let future_two = download_async("https://www.bar.com");

    // 将两个期物同时运行到完成状态
    join!(future_one, future_two);
}
// ANCHOR_END: get_two_sites_async

#[test]
fn get_two_sites_async_test() {
    block_on(get_two_sites_async());
}
