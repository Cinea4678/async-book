# `生成`

生成（spawn）允许你在后台运行一个新的异步任务。这使我们能够在任务运行的同时继续执行其他代码。

假设我们有一个希望在不阻塞主线程的情况下接受连接的网络服务器。为此，我们可以使用 `async_std::task::spawn` 函数来创建并运行一个处理连接的新任务。该函数接收一个期物（future），并返回一个 `JoinHandle`，可以在任务完成后使用它来等待任务的结果。

```rust,edition2018
{{#include ../../examples/06_04_spawning/src/lib.rs:example}}
```

`spawn`函数返回的`JoinHandle`实现了`Future`特征（trait），所以我们可以通过`.await`来获取任务的结果。这将阻塞当前任务，直到被生成的任务完成。如果不对任务进行`.await`，程序将继续执行而不等待该任务，并在函数完成时将它取消。

```rust,edition2018
{{#include ../../examples/06_04_spawning/src/lib.rs:join_all}}
```

为了在主任务和生成的任务之间进行通信，我们可以使用异步运行时提供的通道。