# 通过`Waker`唤醒任务

通常情况下，期物（future）在第一次被`poll`时无法完成任务。当这种情况发生时，期物需要确保在准备好继续推进时再次被`poll`。这是通过`Waker`类型来实现的。

每次期物被`poll`时，都是作为一个“任务”的一部分进行的。任务是被提交给执行器的顶层期物。

`Waker` 提供了一个 `wake()` 方法，可以用来通知执行器相关的任务应该被唤醒。当调用 `wake()` 时，执行器就知道与该 `Waker` 关联的任务已经准备好进行下一步操作，其期物应再次进行轮询。

`Waker` 还实现了 `clone()`，因此它可以被复制和存储。

让我们尝试使用 `Waker` 实现一个简单的定时器期物。

## 应用：构建一个定时器

为了方便举例，当计时器创建时，我们将启动一个新的线程，休眠指定的时间，然后在时间窗口结束时向计时器期物发出信号。

首先，使用 `cargo new --lib timer_future` 启动一个新项目，并在 `src/lib.rs` 中添加我们需要的导入内容：

```rust
{{#include ../../examples/02_03_timer/src/lib.rs:imports}}
```

首先，我们来定义期物类型。我们的期物需要一种方式让线程能够传达定时器已到期，并且期物应该完成的消息。我们将使用一个共享的 `Arc<Mutex<..>>` 值来在线程和期物之间进行通信。

```rust,ignore
{{#include ../../examples/02_03_timer/src/lib.rs:timer_decl}}
```

现在，让我们实际编写 `Future` 的实现吧！

```rust,ignore
{{#include ../../examples/02_03_timer/src/lib.rs:future_for_timer}}
```

很简单，对吧？如果线程已经将 `shared_state.completed` 设置为真，那么这个期物就完成了！否则，我们会克隆当前任务的 `Waker` 并将其传递给 `shared_state.waker`，这样线程就可以唤醒该任务。

重要的是，每次期物被轮询时，我们都必须更新 `Waker`，因为期物可能已经移动到具有不同 `Waker` 的另一个任务中。这种情况会在期物被轮询后在任务之间传递时发生。

最后，我们需要API来实际构建计时器并启动线程：

```rust,ignore
{{#include ../../examples/02_03_timer/src/lib.rs:timer_new}}
```

太棒了！这就是我们构建一个简单的计时期物所需的一切。现在，如果我们有一个执行器来运行这个期物就好了……
