#![cfg(test)]

// ANCHOR: imports
use futures::{
    future::{BoxFuture, FutureExt},
    task::{waker_ref, ArcWake},
};
use std::{
    future::Future,
    sync::mpsc::{sync_channel, Receiver, SyncSender},
    sync::{Arc, Mutex},
    task::Context,
    time::Duration,
};
// 我们在上一节里写的计时器
use timer_future::TimerFuture;
// ANCHOR_END: imports

// ANCHOR: executor_decl
/// 从通道接收任务并执行之的任务执行器。
struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}

/// `Spawner` 会将新的期物生成到任务通道中。
#[derive(Clone)]
struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}

/// 可以重新调度自身，以便由`Executor`轮询的期物。
struct Task {
    /// 正在执行中的期物，应当被推动到完成。
    /// 
    /// 使用 `Mutex` 与否并不影响正确性，因为我们一次只在一个线程上执行任务。
    /// 然而，Rust 并不足够智能，无法知道 `future` 仅在一个线程中被修改，
    /// 所以我们需要使用 `Mutex` 来证明线程安全性。在生产环境中的执行器不需要这样做，
    /// 可以使用 `UnsafeCell` 代替。
    future: Mutex<Option<BoxFuture<'static, ()>>>,

    /// 将任务本身放回任务队列的句柄。
    task_sender: SyncSender<Arc<Task>>,
}

fn new_executor_and_spawner() -> (Executor, Spawner) {
    // 最大允许在通道中排队的任务数。这只是为了让 `sync_channel` 满意，
    // 并不会出现在实际的执行器中。
    const MAX_QUEUED_TASKS: usize = 10_000;
    let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
    (Executor { ready_queue }, Spawner { task_sender })
}
// ANCHOR_END: executor_decl

// ANCHOR: spawn_fn
impl Spawner {
    fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        self.task_sender.send(task).expect("too many tasks queued");
    }
}
// ANCHOR_END: spawn_fn

// ANCHOR: arcwake_for_task
impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        // 实现 `wake`，将此任务重新发送到任务通道，以便执行器可以再次对其进行轮询。
        let cloned = arc_self.clone();
        arc_self
            .task_sender
            .send(cloned)
            .expect("too many tasks queued");
    }
}
// ANCHOR_END: arcwake_for_task

// ANCHOR: executor_run
impl Executor {
    fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            // 将期物取出，并且如果它尚未完成（仍然是Some），则对其进行轮询以尝试完成它。
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                // 从任务自身创建一个`LocalWaker`
                let waker = waker_ref(&task);
                let context = &mut Context::from_waker(&waker);
                // `BoxFuture<T>` 是 `Pin<Box<dyn Future<Output = T> + Send + 'static>>` 的类型别名。
                // 我们可以通过调用 `Pin::as_mut` 方法从中获取 `Pin<&mut dyn Future + Send + 'static>`。
                if future.as_mut().poll(context).is_pending() {
                    // 我们还没有处理完这个期物，所以将它放回任务中，以便将来再次运行。
                    *future_slot = Some(future);
                }
            }
        }
    }
}
// ANCHOR_END: executor_run

// ANCHOR: main
fn main() {
    let (executor, spawner) = new_executor_and_spawner();

    // 生成一个任务以在等待定时器之前和之后打印内容。
    spawner.spawn(async {
        println!("howdy!");
        // 等待我们的计时器期物在两秒后完成
        TimerFuture::new(Duration::new(2, 0)).await;
        println!("done!");
    });

    // 将生成器丢弃，这样我们的执行器就知道它已经完成，不会再接收到需要运行的新任务。
    drop(spawner);

    // 运行执行器直到任务队列为空。  
    // 这将先打印“howdy!”，暂停片刻，然后打印“done!”。
    executor.run();
}
// ANCHOR_END: main

#[test]
fn run_main() {
    main()
}
