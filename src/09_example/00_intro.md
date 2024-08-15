# 最终项目：使用异步Rust构建并发Web服务器

在本章中，我们将使用异步Rust来修改《Rust编程语言》中的[单线程Web服务器](https://doc.rust-lang.org/book/ch20-01-single-threaded.html)，使其能够并发地处理请求。

## 回顾
以下是那节课结束时代码的样子。

`src/main.rs`:
```rust
{{#include ../../examples/09_01_sync_tcp_server/src/main.rs}}
```

`hello.html`:
```html
{{#include ../../examples/09_01_sync_tcp_server/hello.html}}
```

`404.html`:
```html
{{#include ../../examples/09_01_sync_tcp_server/404.html}}
```

如果你使用 `cargo run` 运行服务器，并在浏览器中访问 `127.0.0.1:7878`，你会看到Ferris向你问好的友好消息！