# 递归

在内部，`async fn` 会创建一个状态机类型，其中包含每个被 `.await` 的子期物（future）。这使得递归 `async fn` 变得有些棘手，因为生成的状态机类型必须包含它自己：

```rust,edition2018
# async fn step_one() { /* ... */ }
# async fn step_two() { /* ... */ }
# struct StepOne;
# struct StepTwo;
// 这个函数:
async fn foo() {
    step_one().await;
    step_two().await;
}
// 生成了一个类似这样的类型:
enum Foo {
    First(StepOne),
    Second(StepTwo),
}

// 因此这个函数:
async fn recursive() {
    recursive().await;
    recursive().await;
}

// 会生成一个类似这样的类型:
enum Recursive {
    First(Recursive),
    Second(Recursive),
}
```

这行不通——我们创建了一个大小无限的类型！编译器会抱怨：

```
error[E0733]: recursion in an `async fn` requires boxing
 --> src/lib.rs:1:22
  |
1 | async fn recursive() {
  |                      ^ an `async fn` cannot invoke itself directly
  |
  = note: a recursive `async fn` must be rewritten to return a boxed future.
```

为了实现递归，我们必须使用 `Box` 引入一个间接层。不幸的是，由于编译器的限制，仅仅将对 `recursive()` 的调用包裹在 `Box::pin` 中是不够的。为了使其正常工作，我们必须将 `recursive` 改为一个非 `async` 函数，该函数返回一个 `.boxed()` 的 `async` 代码块：

```rust,edition2018
{{#include ../../examples/07_05_recursion/src/lib.rs:example}}
```
