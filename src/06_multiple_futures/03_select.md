# `select!`

`futures::select`宏可以同时运行多个期物（future），并允许用户在任意一个期物完成时立即做出响应。

```rust,edition2018
{{#include ../../examples/06_03_select/src/lib.rs:example}}
```

上面的函数将同时运行 `t1` 和 `t2`。当 `t1` 或 `t2` 中的任意一个完成时，相应的处理程序将调用 `println!`，并且函数将在未完成剩余任务的情况下结束。

`select` 的基本语法是 `<模式> = <表达式> => <代码>`，可根据需要重复多次，以便在多个期物上进行 `select` 操作。

## `default => ...` 和 `complete => ...`

`select` 还支持 `default` 和 `complete` 分支。

如果没有任何正在被 `select` 的期物已经完成，那么 `default` 分支将会运行。因此，带有 `default` 分支的 `select` 将总是立即返回，因为如果其他任何期物都没有准备好，那么它可以运行 `default`。

`complete` 分支可以用来处理所有被 `select` 的期物已经完成并且将不再取得进展的情况。这在循环中使用 `select!` 时通常非常有用。

```rust,edition2018
{{#include ../../examples/06_03_select/src/lib.rs:default_and_complete}}
```

## 与 `Unpin` 和 `FusedFuture` 的交互

你可能已经注意到，在上面的第一个例子中，我们不得不对两个 `async fn` 返回的期物调用 `.fuse()`，并使用 `pin_mut` 对它们进行固定。这两个函数调用都是必要的，因为在 `select` 中使用的期物必须同时实现 `Unpin` 特征（trait）和 `FusedFuture` 特征。

`Unpin` 是必要的，因为在 `select` 中使用的期物并不是通过值传递的，而是通过可变引用传递的。由于不需要获取期物的所有权，未完成的期物在调用 `select` 之后可以再次使用。

同样，`FusedFuture` 特征也是必需的，因为 `select` 不能在期物完成后再次对其进行轮询。实现了 `FusedFuture` 的期物能够被跟踪是否已经完成。这使得可以在循环中使用 `select`，并仅对尚未完成的期物进行轮询。这一点可以在上面的例子中看到，当循环第二次运行时，`a_fut` 或 `b_fut` 可能已经完成。由于 `future::ready` 返回的期物实现了 `FusedFuture`，因此它能够告诉 `select` 不要再次轮询它。

请注意，流有一个对应的 `FusedStream` 特征。实现了该特征的流或通过 `.fuse()` 包装后的流，将从它们的 `.next()` / `.try_next()` 组合器中生成 `FusedFuture` 期物。

```rust,edition2018
{{#include ../../examples/06_03_select/src/lib.rs:fused_stream}}
```

## 在 `select` 循环中使用 `Fuse` 和 `FuturesUnordered` 实现并发任务

有一个不太容易发现但很有用的函数 `Fuse::terminated()`，它允许构造一个已经终止的空期物，并可以在之后填充需要运行的期物。

当有一个需要在 `select` 循环中运行的任务，而该任务是在 `select` 循环内部创建的，那么这种方法会非常方便。

请注意 `.select_next_some()` 函数的使用。它可以与 `select` 一起使用，仅对从流中返回的 `Some(_)` 值运行分支，并忽略 `None` 值。

```rust,edition2018
{{#include ../../examples/06_03_select/src/lib.rs:fuse_terminated}}
```

当需要同时运行多个相同的期物副本时，可以使用 `FuturesUnordered` 类型。以下示例与上面的示例类似，但会运行每个 `run_on_new_num_fut` 的副本直到完成，而不是在创建新的副本时中止它们。它还会打印出 `run_on_new_num_fut` 返回的值。

```rust,edition2018
{{#include ../../examples/06_03_select/src/lib.rs:futures_unordered}}
```
