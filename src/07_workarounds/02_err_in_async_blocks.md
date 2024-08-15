# `async` 块中的 `?`

就像在 `async fn` 中一样，在 `async` 块中使用 `?` 是很常见的。然而，`async` 块的返回类型一般不会被显式声明，这可能导致编译器无法推断出 `async` 块的错误类型。

例如，以下代码：

```rust,edition2018
# struct MyError;
# async fn foo() -> Result<(), MyError> { Ok(()) }
# async fn bar() -> Result<(), MyError> { Ok(()) }
let fut = async {
    foo().await?;
    bar().await?;
    Ok(())
};
```

会触发这个错误：

```
error[E0282]: type annotations needed
 --> src/main.rs:5:9
  |
4 |     let fut = async {
  |         --- consider giving `fut` a type
5 |         foo().await?;
  |         ^^^^^^^^^^^^ cannot infer type
```

不幸的是，目前还没有办法为 `fut` “指定类型”，也无法显式指定 `async` 块的返回类型。要解决这个问题，可以使用 `::<>` 操作符为 `async` 块提供成功和错误类型：

```rust,edition2018
# struct MyError;
# async fn foo() -> Result<(), MyError> { Ok(()) }
# async fn bar() -> Result<(), MyError> { Ok(()) }
let fut = async {
    foo().await?;
    bar().await?;
    Ok::<(), MyError>(()) // <- 注意这里的显式类型声明
};
```

