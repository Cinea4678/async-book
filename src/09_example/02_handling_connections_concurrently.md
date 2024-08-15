# 并发处理连接

目前代码的问题在于 `listener.incoming()` 是一个阻塞的迭代器。在 `listener` 等待传入连接时，执行器不能运行其他期物（future），并且在处理完上一个连接之前，我们无法处理新的连接。

为了解决这个问题，我们将把 `listener.incoming()` 从一个阻塞的迭代器转换为一个非阻塞的流（Stream）。流与迭代器类似，但可以被异步消费。有关更多信息，请参阅[关于流的章节](../05_streams/01_chapter.md)。

接下来，我们将用非阻塞的 `async_std::net::TcpListener` 替换当前阻塞的 `std::net::TcpListener`，
并更新我们的连接处理器以接受 `async_std::net::TcpStream`：

```rust,ignore
{{#include ../../examples/09_04_concurrent_tcp_server/src/main.rs:handle_connection}}
```

`TcpListener`的异步版本为`listener.incoming()`实现了`Stream`特征，这一改变带来了两个好处：

首先，`listener.incoming()`不再阻塞执行器。当没有需要处理的传入TCP连接时，执行器现在可以让出时间片给其他待处理的期物。

其次，流中的元素可以使用Stream的`for_each_concurrent`方法来并发处理。在这里，我们将利用这个方法来并发处理每个传入的请求。我们需要从`futures`板条箱（crate）中导入`Stream`特征，因此现在我们的Cargo.toml看起来如下：

```diff
+[dependencies]
+futures = "0.3"

 [dependencies.async-std]
 version = "1.6"
 features = ["attributes"]
```

现在，我们可以通过闭包函数来并发地处理每个连接。这个闭包函数会取得每个`TcpStream`的所有权，并在新的`TcpStream`可用时立即运行。只要`handle_connection`不阻塞，一个慢请求就不会再阻止其他请求的完成。

```rust,ignore
{{#include ../../examples/09_04_concurrent_tcp_server/src/main.rs:main_func}}
```
# 并行处理请求

到目前为止，我们的例子主要将并发（使用异步代码）作为并行（使用线程）的替代方案来呈现。然而，异步代码和线程并不是互相排斥的。在我们的例子中，`for_each_concurrent` 并发、但在同一线程上地处理每个连接。除此以外，`async-std` 板条箱也允许我们将任务生成到独立的线程上。

由于 `handle_connection` 既具有 `Send` 特征，又是非阻塞的，因此可以安全地与 `async_std::task::spawn` 一起使用。以下是其示例代码：

```rust
{{#include ../../examples/09_05_final_tcp_server/src/main.rs:main_func}}
```
现在，我们同时使用了并发和并行来处理多个请求！请参阅[多线程执行器部分](../08_ecosystem/00_chapter.md#single-threading-vs-multithreading)来获取更多信息。