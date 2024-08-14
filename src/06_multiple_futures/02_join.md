# `join!`

`futures::join`宏使得在并发执行多个不同的期物（future）的同时，等待它们全部完成成为可能。

## `join!`

在执行多个异步操作时，人们往往会简单地按顺序使用`.await`来等待它们：

```rust,edition2018,ignore
{{#include ../../examples/06_02_join/src/lib.rs:naiive}}
```

然而，这种方式会比预期更慢，因为它不会在`get_book`完成之前开始尝试`get_music`。在某些其他语言中，期物是在环境中自动执行的，因此可以通过先调用每个`async fn`来启动期物，然后同时等待它们的完成，从而并发地运行两个操作：

```rust,edition2018,ignore
{{#include ../../examples/06_02_join/src/lib.rs:other_langs}}
```

但是，Rust的期物在被主动调用 `.await` 之前不会执行任何工作。这意味着上面的两个代码片段都会依次运行 `book_future` 和 `music_future`，而不是并发运行它们。要正确地并发运行这两个期物，请使用 `futures::join!`：

```rust,edition2018,ignore
{{#include ../../examples/06_02_join/src/lib.rs:join}}
```

`join!` 返回的值是一个元组，其中包含每个传入期物的输出。

## `try_join!`

对于返回 `Result` 的期物，建议使用 `try_join!` 而不是 `join!`。由于 `join!` 只有在所有子期物都完成后才会完成，因此即使其中一个子期物返回了 `Err`，它也会继续处理其他的期物。

与 `join!` 不同，`try_join!` 会在其中一个子期物返回错误时立即完成。

```rust,edition2018,ignore
{{#include ../../examples/06_02_join/src/lib.rs:try_join}}
```

注意，传递给 `try_join!` 的期物必须具有相同的错误类型。可以考虑使用 `futures::future::TryFutureExt` 中的 `.map_err(|e| ...)` 和 `.err_into()` 函数来统一错误类型：

```rust,edition2018,ignore
{{#include ../../examples/06_02_join/src/lib.rs:try_join_map_err}}
```
