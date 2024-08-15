# 特征中的`async`

> 译注：原文已经过时。`async fn` in traits 已在Rust 1.75版本中稳定，详见[Announcing Rust 1.75.0](https://blog.rust-lang.org/2023/12/28/Rust-1.75.0.html)。

目前，在Rust的稳定版本中，`async fn`不能在特征（trait）中使用。从2022年11月17日开始，`async-fn-in-trait`的一个MVP版本已在编译器工具链的nightly版本中可用，[详细信息请参见此处](https://blog.rust-lang.org/inside-rust/2022/11/17/async-fn-in-trait-nightly.html)。

同时，对于稳定的工具链，有一个解决方案是使用来自crates.io的[async-trait板条箱（crate）](https://github.com/dtolnay/async-trait)。

请注意，使用这些特征方法将导致每次函数调用时进行一次堆分配。对于绝大多数应用程序来说，这并不是一个显著的成本，但在决定是否在预期每秒调用数百万次的低级函数的公共API中使用此功能时，应该考虑这一点。

