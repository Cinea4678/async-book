# 迭代和并发

与同步的 `Iterator` 类似，有许多不同的方法可以遍历和处理 `Stream` 中的值。你可以使用组合器风格的函数，比如 `map`、`filter` 和 `fold`，以及它们的遇错提前退出的变体 `try_map`、`try_filter` 和 `try_fold`。

遗憾的是，`for` 循环不能用于 `Stream`，但是对于命令式风格的代码，可以使用 `while let` 和 `next`/`try_next` 函数：

```rust,edition2018,ignore
{{#include ../../examples/05_02_iteration_and_concurrency/src/lib.rs:nexts}}
```

然而，如果我们一次只处理一个元素，那么我们可能会错失并发处理元素的机会。毕竟，编写异步代码的初衷正是为了利用并发。要同时处理来自流的多个项目，可以使用 `for_each_concurrent` 和 `try_for_each_concurrent` 方法：

```rust,edition2018,ignore
{{#include ../../examples/05_02_iteration_and_concurrency/src/lib.rs:try_for_each_concurrent}}
```
