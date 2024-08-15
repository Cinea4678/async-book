# `Send` 近似

有些 `async fn` 状态机可以安全地在不同线程之间传递，有些则不能。一个 `async fn` 的 `Future` 是否实现了 `Send` 特征（trait），取决于其是否在 `.await` 处持有了一个非 `Send` 类型。编译器尽其所能地近似分析值是否可能在 `.await` 处被持有，但目前在许多情况下，分析结果表现得过于保守。

例如，考虑一个简单的非 `Send` 类型，它可能包含了 `Rc` ：

```rust
use std::rc::Rc;

#[derive(Default)]
struct NotSend(Rc<()>);
```

即使在 `async fn` 中返回的 `Future` 类型必须是 `Send` 的情况下，类型为 `NotSend` 的变量也可以在 `async fn` 中暂时作为临时变量出现。

```rust,edition2018
# use std::rc::Rc;
# #[derive(Default)]
# struct NotSend(Rc<()>);
async fn bar() {}
async fn foo() {
    NotSend::default();
    bar().await;
}

fn require_send(_: impl Send) {}

fn main() {
    require_send(foo());
}
```

然而，如果我们将 `foo` 修改为 “在一个变量中存储 `NotSend`”，那么这个例子将无法再编译：

```rust,edition2018
# use std::rc::Rc;
# #[derive(Default)]
# struct NotSend(Rc<()>);
# async fn bar() {}
async fn foo() {
    let x = NotSend::default();
    bar().await;
}
# fn require_send(_: impl Send) {}
# fn main() {
#    require_send(foo());
# }
```

```
error[E0277]: `std::rc::Rc<()>` cannot be sent between threads safely
  --> src/main.rs:15:5
   |
15 |     require_send(foo());
   |     ^^^^^^^^^^^^ `std::rc::Rc<()>` cannot be sent between threads safely
   |
   = help: within `impl std::future::Future`, the trait `std::marker::Send` is not implemented for `std::rc::Rc<()>`
   = note: required because it appears within the type `NotSend`
   = note: required because it appears within the type `{NotSend, impl std::future::Future, ()}`
   = note: required because it appears within the type `[static generator@src/main.rs:7:16: 10:2 {NotSend, impl std::future::Future, ()}]`
   = note: required because it appears within the type `std::future::GenFuture<[static generator@src/main.rs:7:16: 10:2 {NotSend, impl std::future::Future, ()}]>`
   = note: required because it appears within the type `impl std::future::Future`
   = note: required because it appears within the type `impl std::future::Future`
note: required by `require_send`
  --> src/main.rs:12:1
   |
12 | fn require_send(_: impl Send) {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
```

这个错误是符合预期的。如果我们将 `x` 存储到一个变量中，那么它在 `.await` 之前将不会被释放，而此时 `async fn` 可能正在运行到不同的线程上。鉴于 `Rc` 不是 `Send` 的，允许它在线程间传递并不安全。一个简单的解决方案是在 `.await` 之前 `drop` 掉 `Rc`，但遗憾的是，这种做法在当前并不会消除编译错误。

为了成功绕过这个问题，你可能需要引入一个块作用域来封装所有非 `Send` 的变量。这使得编译器更容易理解这些变量在 `.await` 处并未存活。

```rust,edition2018
# use std::rc::Rc;
# #[derive(Default)]
# struct NotSend(Rc<()>);
# async fn bar() {}
async fn foo() {
    {
        let x = NotSend::default();
    }
    bar().await;
}
# fn require_send(_: impl Send) {}
# fn main() {
#    require_send(foo());
# }
```
