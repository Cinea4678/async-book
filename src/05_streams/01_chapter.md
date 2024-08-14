#  `Stream`特征

`Stream` 特征（trait）类似于 `Future`，但其在完成前可以产生多个值，类似于标准库中的 `Iterator` 特征：

```rust,ignore
{{#include ../../examples/05_01_streams/src/lib.rs:stream_trait}}
```

One common example of a `Stream` is the `Receiver` for the channel type from
the `futures` crate. It will yield `Some(val)` every time a value is sent
from the `Sender` end, and will yield `None` once the `Sender` has been
dropped and all pending messages have been received:

一个常见的 `Stream` 示例是来自 `futures` 板条箱（crate）的通道类型的 `Receiver`。每当从 `Sender` 端发送一个值时，它会生成 `Some(val)`；如果 `Sender` 被丢弃并且所有待处理的消息都已接收完毕，它会生成 `None`：

```rust,edition2018,ignore
{{#include ../../examples/05_01_streams/src/lib.rs:channels}}
```
