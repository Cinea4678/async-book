# 测试TCP服务器

接下来，我们来测试`handle_connection`函数。

首先，我们需要一个`TcpStream`来进行测试。在端到端或集成测试中，我们可能需要建立一个真实的TCP连接来测试我们的代码。实现这一目标的一种策略是在`localhost`的0号端口上启动监听器。0号端口虽然不是一个有效的UNIX端口，但在测试中它是可行的，操作系统会为我们选择一个可用的TCP端口。

不过，在这个例子中，我们将为连接处理器编写一个单元测试，以检查针对不同输入返回的响应是否正确。为了保持我们的单元测试独立且具有确定性，我们将用一个模拟（mock）来替换`TcpStream`。

首先，我们将更改 `handle_connection` 的签名，以便更容易进行测试。`handle_connection` 实际上并不需要一个 `async_std::net::TcpStream`；它只需要任何实现了 `async_std::io::Read`、`async_std::io::Write` 和 `marker::Unpin` 的结构体。根据这一点来更改其类型签名，可以让我们传递一个用于测试的模拟（mock）。

```rust,ignore
use async_std::io::{Read, Write};

async fn handle_connection(mut stream: impl Read + Write + Unpin) {
```

接下来，让我们构建一个模拟的 `TcpStream`，并实现这些特征（trait）。首先，让我们实现 `Read` 特征，它包含一个 `poll_read` 方法。我们的模拟 `TcpStream` 包含一些数据，它们将被复制到读取缓冲区内，然后返回 `Poll::Ready`，以示读取已完成。

```rust,ignore
{{#include ../../examples/09_05_final_tcp_server/src/main.rs:mock_read}}
```

我们的 `Write` 实现非常相似，不过我们需要编写三个方法：`poll_write`、`poll_flush` 和 `poll_close`。`poll_write` 将会把任何输入数据复制到模拟的 `TcpStream` 中，并在完成时返回 `Poll::Ready`。对于模拟 `TcpStream` 的刷新或关闭，无需进行任何操作，因此 `poll_flush` 和 `poll_close` 只需要返回 `Poll::Ready` 即可。

```rust,ignore
{{#include ../../examples/09_05_final_tcp_server/src/main.rs:mock_write}}
```

最后，我们的模拟需要实现 `Unpin`，以表示它在内存中的位置可以被安全地移动。有关固定和 `Unpin` 特征的更多信息，请参见[固定部分](../04_pinning/01_chapter.md)。

```rust,ignore
{{#include ../../examples/09_05_final_tcp_server/src/main.rs:unpin}}
```

现在我们准备测试 `handle_connection` 函数了。

在设置好包含一些初始数据的 `MockTcpStream` 之后，我们可以使用属性 `#[async_std::test]` 来运行 `handle_connection`，这类似于我们使用 `#[async_std::main]` 的方式。

为了确保 `handle_connection` 按预期工作，我们将检查根据其初始内容写入到 `MockTcpStream` 的数据是否正确。

```rust,ignore
{{#include ../../examples/09_05_final_tcp_server/src/main.rs:test}}
```
