# `async`/`.await`

在[第一章]里，我们简要地介绍了 `async`/`.await`。本章将详细讨论 `async`/`.await`，解释其工作原理以及 `async` 代码和传统Rust程序的区别。

`async`/`.await` 是Rust语言中特殊的语法，它们使得代码可以让出当前线程的控制权，而不是进行阻塞，从而允许其他代码在等待某个操作完成时继续进行。

有两种主要方式可以使用 `async`：`async fn` 和 `async` 块。它们都会返回一个实现了 `Future` 特征（trait）的值：

```rust,edition2018,ignore
{{#include ../../examples/03_01_async_await/src/lib.rs:async_fn_and_block_examples}}
```

正如我们在第一章中看到的，`async` 块和其他期物都是惰性的：它们在被运行之前什么都不做。运行一个期物（future）最常见的方法是对其调用 `.await`。当对一个期物调用 `.await` 时，它会尝试将其运行到完成。如果期物被阻塞，它将让出当前线程的控制权。当可以取得更多进展时，期物将由执行器拾起并继续运行，从而使 `.await` 得到解决（resolve）。

## `async`生命周期

与传统函数不同，`async fn` 接受引用或其他非 `'static` 的参数时，返回的期物会受到这些参数的生命周期限制：

```rust,edition2018,ignore
{{#include ../../examples/03_01_async_await/src/lib.rs:lifetimes_expanded}}
```

这意味着从 `async fn` 返回的期物必须在其非 `'static` 参数仍然有效的情况下进行 `.await` 操作。在通常情况下，如在调用函数后立即对期物进行 `.await` 操作（如 `foo(&x).await`），这不会成为问题。然而，如果要存储期物或将其发送到另一个任务或线程，这可能会引发问题。

一种常见的解决方法是将带有引用作为参数的 `async fn` 转换为 `'static` 期物，即将参数与对 `async fn` 的调用捆绑在一个 `async` 块内：

```rust,edition2018,ignore
{{#include ../../examples/03_01_async_await/src/lib.rs:static_future_with_borrow}}
```

通过将参数移入 `async` 块中，我们将其生命周期延长至与从调用 `good` 返回的期物的生命周期相匹配。

## `async move`

`async` 块和闭包允许使用 `move` 关键字，就像普通的闭包一样。一个 `async move` 块将会获取它所引用变量的所有权，从而允许这些变量拥有超出当前作用域的生命周期，但同时也放弃了与其他代码共享这些变量的能力。

```rust,edition2018,ignore
{{#include ../../examples/03_01_async_await/src/lib.rs:async_move_examples}}
```

## 在多线程执行器上进行`.await`

请注意，当使用多线程的期物执行器时，一个期物可能会在线程之间移动，因此在`async`代码块中使用的任何变量都必须能够在线程之间传递，因为任何`.await`都可能导致切换到一个新线程。

这意味着使用`Rc`、`&RefCell`或任何其他不实现`Send`特征的类型（包括引用不实现`Sync`特征的类型）都是不安全的。

（注意：如果这些类型在调用`.await`时不在作用域内，那它们是可以使用的。）

类似地，在`.await`期间持有传统的非期物感知锁也不是一个好主意，因为这可能会导致线程池死锁：一个任务可能获取了锁，执行`.await`并将线程让出给执行器，允许另一个任务尝试获取该锁，进而导致死锁。为避免这种情况，请使用`futures::lock`中的`Mutex`，而不是`std::sync`中的那个。（译注：`Tokio`和`async-std`也提供了类似的`Mutex`。）

[第一章]: ../01_getting_started/04_async_await_primer.md
