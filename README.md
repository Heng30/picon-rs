<img src="./screenshot/picon-en.png" width="100"/>
<!-- ![screenshot](./screenshot/picon-en.png) -->

[中文文档](./README.zh-CN.md)

#### Introduction
It is a simple Android App for displaying crypto news. Based on `Rust` and `egui`.

#### Features
- [x] use [odaily.news](https://www.odaily.news/) provided `API` to fetch Chinese news
- [x] use [cryptocompare](https://min-api.cryptocompare.com/data/v2/news/?lang=EN) provided `API` to fetch English news

#### How to build?
- Install Android `sdk`, `ndk`, `jdk17`, and set environment variables
- Install `Rust` and `Cargo`
- Run `make`
- Refer to [Makefile](./Makefile) and [build.help](./build.help) for more information

#### Reference
- [egui](https://github.com/emilk/egui)
- [rust-android-examples](https://github.com/rust-mobile/rust-android-examples)
- [cross-platform-rust-http-request](https://logankeenan.com/posts/cross-platform-rust-http-request/)

