<img src="./screenshot/picon-cn.png" width="100"/>
<!-- ![screenshot](./screenshot/picon-cn.png) -->

[English Documentation](./README.md)

#### 简介
一个简单的安卓加密新闻软件。基于`Rust`和`egui`实现。

#### 功能
- [x] 使用[odaily.news](https://www.odaily.news/) 提供的`API`获取中文新闻
- [x] 使用[cryptocompare](https://min-api.cryptocompare.com/data/v2/news/?lang=EN) 提供的`API`获取英文新闻

#### 如何构建?
- 安装 Android `sdk`, `ndk` 和 `jdk17`，并配置相应的环境变量
- 安装 `Rust` 和 `Cargo`
- 运行 `make`
- 参考 [Makefile](./Makefile) 和 [build.help](./build.help) 了解更多信息

#### 参考
- [egui](https://github.com/emilk/egui)
- [rust-android-examples](https://github.com/rust-mobile/rust-android-examples)
- [cross-platform-rust-http-request](https://logankeenan.com/posts/cross-platform-rust-http-request/)
