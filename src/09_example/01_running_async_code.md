# 运行异步代码

一个HTTP服务器应该能够同时为多个客户端提供服务；也就是说，它不应该在处理当前请求之前等待前一个请求完成。《Rust编程语言》中[通过创建一个线程池来解决这个问题](https://doc.rust-lang.org/book/ch20-02-multithreaded.html#turning-our-single-threaded-server-into-a-multithreaded-server)，每个连接都在其自己的线程中处理。在这里，我们不会通过增加线程来提高吞吐量，而是使用异步代码来达到相同的效果。

让我们通过将 `handle_connection` 声明为一个 `async fn` 来使其返回一个期物（future）：

```rust,ignore
{{#include ../../examples/09_02_async_tcp_server/src/main.rs:handle_connection_async}}
```

将 `async` 添加到函数的声明中，会将其返回类型从单元类型 `()` 改变为实现了 `Future<Output=()>` 的类型。

如果我们尝试编译它，编译器会警告我们期物将不会工作：

```console
$ cargo check
    Checking async-rust v0.1.0 (file:///projects/async-rust)
warning: unused implementer of `std::future::Future` that must be used
  --> src/main.rs:12:9
   |
12 |         handle_connection(stream);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_must_use)]` on by default
   = note: futures do nothing unless you `.await` or poll them
```

因为我们没有对 `handle_connection` 的结果进行 `await` 或 `poll`，因此它将永远不会运行。如果你运行服务器并在浏览器中访问 `127.0.0.1:7878`，你会发现连接被拒绝了；我们的服务器没有处理请求。

我们无法在同步代码中直接 `await` 或 `poll` 期物。我们需要一个异步运行时来调度和运行期物直到完成。请参考[选择运行时的部分](../08_ecosystem/00_chapter.md)以获取有关异步运行时、执行器和反应器的更多信息。前文列出的所有运行时都适用于本项目。在下面的示例中，我们将选用 `async-std` 板条箱（crate）。

## 添加一个异步运行时

以下示例将演示如何将同步代码重构为使用异步运行时；在这里，我们使用 `async-std`。`async-std` 中的 `#[async_std::main]` 属性允许我们编写一个异步的主函数。要使用这个功能，请在 `Cargo.toml` 中启用 `async-std` 的 `attributes` 特性：

```toml
[dependencies.async-std]
version = "1.6"
features = ["attributes"]
```

首先，我们将把 `main` 函数切换为异步，并使用 `await` 来等待由异步版本的 `handle_connection` 返回的期物。然后，我们将测试服务器的响应情况。代码示例如下：

```rust
{{#include ../../examples/09_02_async_tcp_server/src/main.rs:main_func}}
```
现在，我们来测试一下服务器是否能够并发处理连接。仅仅将 `handle_connection` 设置为异步函数并不意味着服务器能够同时处理多个连接；原因我们马上就会知道。

为了说明这一点，我们来模拟一个慢速请求。当客户端向 `127.0.0.1:7878/sleep` 发出请求时，服务器将会休眠5秒钟：

```rust,ignore
{{#include ../../examples/09_03_slow_request/src/main.rs:handle_connection}}
```
这与《Rust编程语言》中[模拟一个慢请求](https://doc.rust-lang.org/book/ch20-02-multithreaded.html#simulating-a-slow-request-in-the-current-server-implementation)非常相似，但有一个重要的区别：我们使用了非阻塞函数 `async_std::task::sleep`，而不是阻塞函数 `std::thread::sleep`。重要的是要记住，即使一段代码在 `async fn` 中运行并被 `await`，它也仍然可能会阻塞。

如果你运行服务器，你会看到对 `127.0.0.1:7878/sleep` 的请求会阻塞任何其他进入的请求5秒钟！这是因为在我们 `await` `handle_connection` 的结果时，不能继续进行其他并发任务。在下一节中，我们将看到如何使用异步代码来并发处理连接。
