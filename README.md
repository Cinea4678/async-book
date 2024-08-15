# async-book汉语翻译
Rust异步编程

## 翻译

本书是[《Asynchronous Programming in Rust》](https://rust-lang.github.io/async-book/index.html)的中文译本，使用和原著相同的开源协议发布。

部分汉语内暂无公认翻译或存在争议的词汇，在翻译时将采取下表所示的策略：

- Trait：翻译为特征。
- Future：翻译为期物。Future有“期货”含义，在语境内通常指“异步任务”或“未来值”，以上三种翻译都是合理的；这里选择接近直译的”期货“，并采用”期物“以避免读者和金融领域中的”期货“混淆。译名”期物“参考了欧长坤著[《现代C++教程》](https://changkun.de/modern-cpp/)中对`std::future`的翻译；在Bjarne Stroustrup著、pansz译的《C++之旅（第3版）》中，亦直译`std::future`为“期货”。
- Stream：翻译为流。
- Crate：翻译为板条箱。Rust的Crate类似Python、Node等语言/框架的“包”，但是将Crate译为“包”在涉及其他语言的语境中容易和Package产生混淆。因此，这里选择直译为“板条箱”。
- Pin：翻译为固定。

译本采用以上翻译策略并不代表其为最优选项，“统一”“清晰”和“不易混淆”才是采用它们的目的和原因。上述翻译词汇在每一章中首次出现时，都会以`特征（trait）`的形式备注其原词。此外，根据语境，这些词语可能指Rust标准库中的同名类型（例如`std::pin::Pin`和`std::future::Future`），此时将保留原词而不进行翻译。

为了保护读者的阅读体验，译本的正文中不会出现其对应原本的提交哈希。贡献者在确认译本对应原本的版本时，可以参考译本仓库的`source`分支，译本保证和`source`分支中存储的原本完全同步，并在更新译本时同步更新`source`分支。



## 要求
异步书是用[`mdbook`]构建的，你可以使用Cargo来安装它。

```
cargo install mdbook
cargo install mdbook-linkcheck
```

[`mdbook`]: https://github.com/rust-lang/mdBook

## 构建
要构建完整的书，运行 `mdbook build` 以在 `book/` 目录下生成它。
```
mdbook build
```

## 开发
在写作期间是可以轻松看到你的修改的，`mdbook serve` 将会启动一个本地的web服务器。
```
mdbook serve
```
