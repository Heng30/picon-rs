#!/bin/bash

build-evn=
run-evn=RUST_LOG=error,warn,info,debug,sqlx=off,reqwest=off,wgpu_core=off,naga=off,wgpu_hal=off,winit=off

all: build-android-lib build-android-app

build-android-lib:
	$(build-evn) cargo ndk -t arm64-v8a -o picon/app/src/main/jniLibs/  build --release

build-android-app: build-android-lib
	cd ./picon && ./gradlew build

build-desktop-app:
	$(build-evn) $(run-evn) cargo build --bin picon --features=desktop --release

install-debug: build-android-app
	cd ./picon && ./gradlew installDebug

install-release: build-android-app
	cd ./picon && ./gradlew installRelease

install-desktop:
	cp -f ./target/release/picon ~/bin

run-android-app:
	adb shell am start -n xyz.heng30.picon/android.app.NativeActivity

run:
	$(build-evn) $(run-evn) cargo run --bin picon --features=desktop

test:
	$(build-evn) $(run-evn) cargo test -- --nocapture

clippy:
	cargo clippy

clean-incremental:
	rm -rf ./target/debug/incremental/*

clean:
	cargo clean
