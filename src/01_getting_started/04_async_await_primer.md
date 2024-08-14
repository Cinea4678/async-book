# `async`/`.await` 入门

`async`/`.await` 是Rust内置的工具，用于编写看起来像同步代码的异步函数。`async` 会将一段代码转换为一个实现了名为 `Future` 的特征（trait）的状态机。而在同步方法中调用阻塞函数会阻塞整个线程，被阻塞的 `Future` 会让出线程的控制权，从而允许其他 `Future` 运行。

让我们向 `Cargo.toml` 文件中添加一些依赖:

```toml
{{#include ../../examples/01_04_async_await_primer/Cargo.toml:9:10}}
```

要创建一个异步函数，你可以使用 `async fn` 语法：

```rust,edition2018
async fn do_something() { /* ... */ }
```

`async fn` 返回的值是一个 `Future`。要使其生效，必须在执行器上运行该 `Future`。

```rust,edition2018
{{#include ../../examples/01_04_async_await_primer/src/lib.rs:hello_world}}
```

在一个 `async fn` 中，你可以使用 `.await` 来等待另一个实现了 `Future` 特征的类型完成，例如另一个 `async fn` 的输出。与 `block_on` 不同，`.await` 并不会阻塞当前线程，而是异步地等待期物（future）完成，如果期物当前无法取得进展，它将允许其他任务运行。

例如，假设我们有三个 `async fn`：`learn_song`、`sing_song` 和 `dance`：

```rust,ignore
async fn learn_song() -> Song { /* ... */ }
async fn sing_song(song: Song) { /* ... */ }
async fn dance() { /* ... */ }
```

一种学习、唱歌和跳舞的方法是单独阻塞每一个任务：

```rust,ignore
{{#include ../../examples/01_04_async_await_primer/src/lib.rs:block_on_each}}
```

然而，以这种方式我们并没有发挥出最佳性能——我们每次只做了一件事！显然，我们必须先学会歌曲才能演唱，但我们可以在学习和演唱歌曲的同时跳舞。为此，我们可以创建两个可以并发运行的 `async fn`：

```rust,ignore
{{#include ../../examples/01_04_async_await_primer/src/lib.rs:block_on_main}}
```

在这个例子中，学习歌曲必须在唱歌之前发生，但学习和唱歌可以与跳舞同时进行。如果在`learn_and_sing`中使用`block_on(learn_song())`而不是`learn_song().await`，线程在`learn_song`运行时将无法执行其他操作。这将使得在学习歌曲的同时跳舞变得不可能。通过对`learn_song`期物进行`.await`，我们允许其他任务在`learn_song`阻塞时接管当前线程。这使得可以在同一线程上同时并发地完成多个期物。
