// ANCHOR: simple_future
trait SimpleFuture {
    type Output;
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
}

enum Poll<T> {
    Ready(T),
    Pending,
}
// ANCHOR_END: simple_future

struct Socket;
impl Socket {
    fn has_data_to_read(&self) -> bool {
        // check if the socket is currently readable
        true
    }
    fn read_buf(&self) -> Vec<u8> {
        // Read data in from the socket
        vec![]
    }
    fn set_readable_callback(&self, _wake: fn()) {
        // register `_wake` with something that will call it
        // once the socket becomes readable, such as an
        // `epoll`-based event loop.
    }
}

// ANCHOR: socket_read
pub struct SocketRead<'a> {
    socket: &'a Socket,
}

impl SimpleFuture for SocketRead<'_> {
    type Output = Vec<u8>;

    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        if self.socket.has_data_to_read() {
            // 套接字有数据——将它读取到一个缓冲区内，并返回。
            Poll::Ready(self.socket.read_buf())
        } else {
            // 套接字暂时还没有数据。
            // 
            // 安排`wake`在一旦有数据时被调用。当数据可用时，`wake`将被调用，
            // 此时使用此期物的用户将知道再次调用`poll`并接收数据。
            self.socket.set_readable_callback(wake);
            Poll::Pending
        }
    }
}
// ANCHOR_END: socket_read

// ANCHOR: join
/// 一个 `SimpleFuture`，它可以并发地运行另外两个期物直到完成。
///
/// 并发性是通过对每个期物的 `poll` 调用可以交替进行来实现的，这允许每个期物以自己
/// 的节奏推进。
pub struct Join<FutureA, FutureB> {
    // 每个字段可能包含一个需要运行至完成的期物。如果该期物已经完成，字段将被设置为
    // `None`。这可以防止我们在期物完成后继续轮询它，这种轮询违反`Future`特征的约定。 
    a: Option<FutureA>,
    b: Option<FutureB>,
}

impl<FutureA, FutureB> SimpleFuture for Join<FutureA, FutureB>
where
    FutureA: SimpleFuture<Output = ()>,
    FutureB: SimpleFuture<Output = ()>,
{
    type Output = ();
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        // 尝试将期物`a`推进至完成。
        if let Some(a) = &mut self.a {
            if let Poll::Ready(()) = a.poll(wake) {
                self.a.take();
            }
        }

        // 尝试将期物`b`推进至完成。
        if let Some(b) = &mut self.b {
            if let Poll::Ready(()) = b.poll(wake) {
                self.b.take();
            }
        }

        if self.a.is_none() && self.b.is_none() {
            // 两个期物都已经完成——我们可以成功返回了
            Poll::Ready(())
        } else {
            // 一个或多个期物返回了`Poll::Pending`并仍然有工作没有完成。它们将在可以继续
            // 推进时调用`wake()`函数。
            Poll::Pending
        }
    }
}
// ANCHOR_END: join

// ANCHOR: and_then
/// 一个 `SimpleFuture`，它依次运行两个期物，直到它们全部完成。
//
// 注意：在这个简单的示例中，`AndThenFut` 假设第一个和第二个期物在其创建时都已经可用。而实际
// 的 `AndThen` 组合器允许根据第一个期物的输出创建第二个期物，例如 `get_breakfast.and_then(|food| eat(food))`。
pub struct AndThenFut<FutureA, FutureB> {
    first: Option<FutureA>,
    second: FutureB,
}

impl<FutureA, FutureB> SimpleFuture for AndThenFut<FutureA, FutureB>
where
    FutureA: SimpleFuture<Output = ()>,
    FutureB: SimpleFuture<Output = ()>,
{
    type Output = ();
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        if let Some(first) = &mut self.first {
            match first.poll(wake) {
                // 我们已经完成第一个期物了——将它移除并开始第二个期物！
                Poll::Ready(()) => self.first.take(),
                // 我们还没有完成第一个期物。
                Poll::Pending => return Poll::Pending,
            };
        }
        // 现在第一个期物已经完成了，尝试完成第二个。
        self.second.poll(wake)
    }
}
// ANCHOR_END: and_then

mod real_future {
use std::{
    future::Future as RealFuture,
    pin::Pin,
    task::{Context, Poll},
};

// ANCHOR: real_future
trait Future {
    type Output;
    fn poll(
        // 注意 `&mut self` 变为了 `Pin<&mut Self>`:
        self: Pin<&mut Self>,
        // 并且 `wake: fn()` 变为了 `cx: &mut Context<'_>`:
        cx: &mut Context<'_>,
    ) -> Poll<Self::Output>;
}
// ANCHOR_END: real_future

// ensure that `Future` matches `RealFuture`:
impl<O> Future for dyn RealFuture<Output = O> {
    type Output = O;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        RealFuture::poll(self, cx)
    }
}
}
