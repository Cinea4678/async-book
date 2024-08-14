#![cfg(test)]
#![allow(dead_code)]

// ANCHOR: example
use async_std::{task, net::TcpListener, net::TcpStream};
use futures::AsyncWriteExt;

async fn process_request(stream: &mut TcpStream) -> Result<(), std::io::Error>{
    stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n").await?;
    stream.write_all(b"Hello World").await?;
    Ok(())
}

async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    loop {
        // 接受一个新连接
        let (mut stream, _) = listener.accept().await.unwrap();
        // 在不阻塞主循环的同时处理这个请求
        task::spawn(async move {process_request(&mut stream).await});
    }
}
// ANCHOR_END: example
use std::time::Duration;
async fn my_task(time: Duration) {
    println!("Hello from my_task with time {:?}", time);
    task::sleep(time).await;
    println!("Goodbye from my_task with time {:?}", time);
}
// ANCHOR: join_all
use futures::future::join_all;
async fn task_spawner(){
    let tasks = vec![
        task::spawn(my_task(Duration::from_secs(1))),
        task::spawn(my_task(Duration::from_secs(2))),
        task::spawn(my_task(Duration::from_secs(3))),
    ];
    // 如果我们不等待这些任务并让函数结束，那么它们将会被丢弃
    join_all(tasks).await;
}
// ANCHOR_END: join_all

#[test]
fn run_task_spawner() {
    futures::executor::block_on(task_spawner());
}