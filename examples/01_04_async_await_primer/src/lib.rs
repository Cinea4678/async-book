#![cfg(test)]

use futures::executor::block_on;

mod first {
// ANCHOR: hello_world
// `block_on` 会阻塞当前线程，直到提供的期物（future）运行完毕。其他执行器则提供了更复杂的行为，
// 例如将多个期物调度到同一个线程上。
use futures::executor::block_on;

async fn hello_world() {
    println!("hello, world!");
}

fn main() {
    let future = hello_world(); // 没有进行输出
    block_on(future); // `future` 被运行， "hello, world!" 被输出
}
// ANCHOR_END: hello_world

#[test]
fn run_main() { main() }
}

struct Song;
async fn learn_song() -> Song { Song }
async fn sing_song(_: Song) {}
async fn dance() {}

mod second {
use super::*;
// ANCHOR: block_on_each
fn main() {
    let song = block_on(learn_song());
    block_on(sing_song(song));
    block_on(dance());
}
// ANCHOR_END: block_on_each

#[test]
fn run_main() { main() }
}

mod third {
use super::*;
// ANCHOR: block_on_main
async fn learn_and_sing() {
    // 在学会这首歌之前，先不把它唱出来。我们在这里使用 `.await` 而不是 `block_on`，
    // 以避免阻塞线程，这样就可以在“唱歌”的同时“跳舞”。
    let song = learn_song().await;
    sing_song(song).await;
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();

    // `join!` 类似于 `.await`，但可以同时等待多个期物。如果我们在 `learn_and_sing` 
    // 期物中暂时被阻塞，那么 `dance` 期物将接管当前线程。如果 `dance` 被阻塞，
    // `learn_and_sing` 可以重新接管。如果两个期物都被阻塞，那么 `async_main` 也会
    // 被阻塞，并将控制权交还给执行器。
    futures::join!(f1, f2);
}

fn main() {
    block_on(async_main());
}
// ANCHOR_END: block_on_main

#[test]
fn run_main() { main() }
}
