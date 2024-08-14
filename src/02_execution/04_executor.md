# 应用：构建一个执行器

Rust的期物（future）是惰性的：除非被主动驱动完成，否则它们不会执行任何操作。将期物在`async`函数内部`.await`是驱动期物完成的一种方法，但这只是将问题推到更高一级：谁来运行由顶层的`async`函数返回的期物呢？答案是我们需要一个期物执行器。

期物执行器会接收一组顶层的期物，并通过在期物可以取得进展时调用`poll`来将它们运行至完成。通常，执行器会先对期物进行一次`poll`以开始期物的执行。当期物通过调用`wake()`表示它们已准备好取得进展时，它们会被重新放回队列中，并再次调用`poll`。这一过程会重复，直到期物完成。

在本节中，我们将编写一个简单的执行器，它能够并发地运行大量顶层的期物，直到完成。

在这个示例中，我们依赖于`futures`板条箱中的`ArcWake`特征，它提供了一种简便的方法来构建`Waker`。请编辑`Cargo.toml`文件以添加一个新的依赖项：

```toml
[package]
name = "timer_future"
version = "0.1.0"
authors = ["XYZ Author"]
edition = "2021"

[dependencies]
futures = "0.3"
```

接下来，我们需要在 `src/main.rs` 的顶部添加以下导入内容：

```rust,ignore
{{#include ../../examples/02_04_executor/src/lib.rs:imports}}
```

我们的执行器通过把任务发送到一个通道上来工作。执行器将从通道中取出事件并运行它们。当一个任务准备好继续工作（被唤醒）时，它可以通过将自己重新放回通道来让自己再次被轮询。

在这个设计中，执行器本身只需要任务通道的接收端。用户将获得发送端，以便他们可以生成新的期物。任务本身就是可以让自己被调度回队列的期物，因此我们将任务们存储为“一个期物和一个发送端”的二元配对，任务可以使用其持有的发送端来让自己重新回到队列。

```rust,ignore
{{#include ../../examples/02_04_executor/src/lib.rs:executor_decl}}
```

让我们也为 `spawner` 添加一个方法，以使得生成新的期物变简单。此方法将接收一个期物类型，将其装箱，并创建一个包含它的新 `Arc<Task>`，这样它就可以被放入执行器的队列中：

```rust,ignore
{{#include ../../examples/02_04_executor/src/lib.rs:spawn_fn}}
```

要轮询期物，我们需要创建一个`Waker`。如在[任务唤醒部分]中讨论的那样，`Waker`负责在`wake`被调用后调度任务，使其再次被轮询。请记住，`Waker`会告诉执行器究竟是哪一个任务已准备就绪，从而允许执行器仅轮询那些准备好推进的期物。创建一个新的`Waker`最简单的方法是实现`ArcWake`特征，然后使用`waker_ref`或`.into_waker()`函数将一个`Arc<impl ArcWake>`转换为`Waker`。让我们为我们的任务实现`ArcWake`，以便它们可以被转换为`Waker`并被唤醒：

```rust,ignore
{{#include ../../examples/02_04_executor/src/lib.rs:arcwake_for_task}}
```

当从`Arc<Task>`创建一个`Waker`时，调用`wake()`会导致一个`Arc`的副本被发送到任务通道上。然后，我们的执行器需要拾取该任务并对其进行轮询。让我们来实现这一点：

```rust,ignore
{{#include ../../examples/02_04_executor/src/lib.rs:executor_run}}
```

恭喜！我们现在有了一个可用的期物执行器。我们甚至可以使用它来运行 `async/.await` 代码和自定义期物，例如我们之前编写的 `TimerFuture`。

```rust,edition2018,ignore
{{#include ../../examples/02_04_executor/src/lib.rs:main}}
```

[任务唤醒部分]: ./03_wakeups.md
