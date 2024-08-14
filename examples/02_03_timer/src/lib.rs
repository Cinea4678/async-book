// ANCHOR: imports
use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread,
    time::Duration,
};
// ANCHOR_END: imports

// ANCHOR: timer_decl
pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

/// 在期物和等待中的线程之间共享状态
struct SharedState {
    /// 睡眠的时间是否已到
    completed: bool,

    /// `TimerFuture`正在运行的任务的唤醒器。当线程将 `completed` 设置为 `true` 后，
    /// 可以使用该唤醒器通知 `TimerFuture` 的任务醒来，检查 `completed` 是否为 `true`，
    /// 然后继续执行。
    waker: Option<Waker>,
}
// ANCHOR_END: timer_decl

// ANCHOR: future_for_timer
impl Future for TimerFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // 查看共享状态，看看计时器是否已经完成。
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            Poll::Ready(())
        } else {
            // 设置唤醒器，以便当计时器完成时，线程可以唤醒当前任务，确保期物再次被轮询，
            // 并看到 `completed = true` 的状态。
            //
            // 虽然这看起来很不错：只设置一次唤醒器，而不是每次都重复克隆它。但是，`TimerFuture` 
            // 可能会在执行器上不同的任务之间移动，这可能导致一个过时的唤醒器指向错误的任务，
            // 进而阻止 `TimerFuture` 正确唤醒。
            //
            // 注意：可以使用 `Waker::will_wake` 函数来检查这种情况，但为了简化处理，
            // 我们在这里省略了这个步骤。
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}
// ANCHOR_END: future_for_timer

// ANCHOR: timer_new
impl TimerFuture {
    /// 创建一个新的 `TimerFuture`，它将在指定的超时之后完成
    pub fn new(duration: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));

        // 启动新线程
        let thread_shared_state = shared_state.clone();
        thread::spawn(move || {
            thread::sleep(duration);
            let mut shared_state = thread_shared_state.lock().unwrap();
            // 发出定时器已完成的信号，并唤醒上次轮询期物的任务（如果存在的话）。
            shared_state.completed = true;
            if let Some(waker) = shared_state.waker.take() {
                waker.wake()
            }
        });

        TimerFuture { shared_state }
    }
}
// ANCHOR_END: timer_new

#[test]
fn block_on_timer() {
    futures::executor::block_on(async {
        TimerFuture::new(Duration::from_secs(1)).await
    })
}
