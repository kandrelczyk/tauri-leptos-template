# Tauri + Leptos

This template should help get you started developing with Tauri and Leptos. 

It uses Tauri 2.0beta and Leptos 0.6.

It has examples of:

* command returning `Result` that can be used with `create_resource`, `Suspense` and `ErrorBoundary`.
* command accepting no arguments
* sending events to the front-end
* logging on both front-end and back-end
* Tailwind CSS integration

### Desktop app

Run `cargo tauri dev`

### Android app

To build the app for Android on Linux insert following into .bashrc:

```
export JAVA_HOME={java home}
export ANDROID_HOME={android home}
export ANDROID_NDK_HOME={android NDK home}

export TOOLCHAIN=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64
export TARGET=aarch64-linux-android
export API=33

export AR=$TOOLCHAIN/bin/llvm-ar
export CC=$TOOLCHAIN/bin/$TARGET$API-clang
export AS=$CC
export CXX=$TOOLCHAIN/bin/$TARGET$API-clang++
export LD=$TOOLCHAIN/bin/ld
export RANLIB=$TOOLCHAIN/bin/llvm-ranlib
export STRIP=$TOOLCHAIN/bin/llvm-strip

export PATH=$PATH:$ANDROID_HOME/cmdline-tools/latest/bin
export PATH=$PATH:$TOOLCHAIN/bin
```


Run `cargo tauri android init` and `cargo tauri android dev`
