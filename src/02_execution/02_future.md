#  `Future` 特征

`Future` 特征（trait）是 Rust 中异步编程的核心。`Future` 是一个异步计算，它可以生成一个值（虽然这个值可能是空的，例如 `()`）。一个 *简化* 的期物（future）特征可能看起来像这样：

```rust
{{#include ../../examples/02_02_future_trait/src/lib.rs:simple_future}}
```

期物可以通过调用 `poll` 函数来推进，该函数将尽可能将期物向完成的方向驱动。如果期物完成，它会返回 `Poll::Ready(result)`。如果期物尚不能够完成，它会返回 `Poll::Pending`，并安排在期物可以继续推进时调用的`wake()` 函数。当 `wake()` 被调用时，驱动期物的执行器将再次调用 `poll`，以便期物可以继续推进。

如果没有 `wake()`，执行器将无法知道某个期物何时可以取得进展，并且将被迫不断地轮询每个期物。有了 `wake()`，执行器就可以确切地知道哪些期物已经准备好被 `poll` 了。

例如，考虑这样一种情况：我们想从一个可能已经有数据，也可能没有数据的套接字中读取数据。如果有数据，我们可以读取并返回 `Poll::Ready(data)`，但如果没有数据准备好，我们的期物将被阻塞，无法继续执行。当没有数据可用时，我们必须注册 `wake`，以便在数据准备好时调用它，通知执行器我们的期物已准备好继续执行。一个简单的 `SocketRead` 期物可能看起来像这样：

```rust,ignore
{{#include ../../examples/02_02_future_trait/src/lib.rs:socket_read}}
```

这种期物模型允许组合多个异步操作，而不需要中间的内存分配。通过类似于无分配状态机的方式，可以同时运行多个期物或将期物串联在一起，实现如下：

```rust,ignore
{{#include ../../examples/02_02_future_trait/src/lib.rs:join}}
```

这展示了如何同时运行多个期物，而无需单独分配资源，从而使异步程序更加高效。类似地，可以依次运行多个连续的期物，如下所示：

```rust,ignore
{{#include ../../examples/02_02_future_trait/src/lib.rs:and_then}}
```

这些示例展示了如何使用 `Future` 特征来表达异步控制流，而无需多个分配的对象和深度嵌套的回调。在掌握了基本的控制流之后，让我们来讨论真正的 `Future` 特征及其不同之处。

```rust,ignore
{{#include ../../examples/02_02_future_trait/src/lib.rs:real_future}}
```

首先，你会注意到我们的 `self` 类型不再是 `&mut Self`，而是变成了 `Pin<&mut Self>`。我们将在[后面的章节][pinning]中详细讨论固定（pinning），但现在你只需要知道，它允许我们创建不可移动的期物。不可移动的对象可以在其字段中存储指针，例如 `struct MyFut { a: i32, ptr_to_a: *const i32 }`。固定是实现 async/await 的必要条件。

其次，`wake: fn()` 被改为 `&mut Context<'_>`。在 `SimpleFuture` 中，我们使用了对函数指针 (`fn()`) 的调用来通知期物执行器“应该轮询当前的期物”。然而，由于 `fn()` 只是一个函数指针，它无法存储关于 *哪个* 期物调用了 `wake` 的任何数据。

在实际场景中，像Web服务器这样复杂的应用程序可能会有成千上万的不同连接，这些连接的唤醒都应该分别管理。`Context`类型通过提供一个`Waker`类型的值来解决这个问题，该值可以用于唤醒特定任务。

[pinning]: ../04_pinning/01_chapter.md
