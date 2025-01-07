# Grpc-rs demo

本项目是一个使用 [grpc](https://github.com/tikv/grpc-rs) 进行 helloworld 通信的 demo，用于展示一个 future poll 两次的问题，这里面涉及到 future 0.1 和 0.3 的东西，由于旧系统升级有些代价，因此并不能简单的将 future 0.1 升级到 0.3。

代码仓库中的 cpufuture 为 [futures-cpupool 0.1.8](https://docs.rs/crate/futures-cpupool/0.1.8)，放到这里方便加 debug 信息。

## 如何执行

```
# 启动 release 的 server
cargo run --bin server --release
# 启动 client
cargo run --bin client
```

当启动 client 时，server 会出现下面的错误信息，当用 debug 模型启动 server 时，则没有这个错误。
目前可以确定问题出现的条件：
- release 编译
- rust 版本大于 nightly-2023-06-02

```
thread 'grpc-poll-0' panicked at 'cannot poll CpuFuture twice: Canceled', ~/demo/cpufuture/src/lib.rs:332:14
stack backtrace:
   0: rust_begin_unwind
             at /rustc/d59363ad0b6391b7fc5bbb02c9ccf9300eef3753/library/std/src/panicking.rs:593:5
   1: core::panicking::panic_fmt
             at /rustc/d59363ad0b6391b7fc5bbb02c9ccf9300eef3753/library/core/src/panicking.rs:67:14
   2: core::result::unwrap_failed
             at /rustc/d59363ad0b6391b7fc5bbb02c9ccf9300eef3753/library/core/src/result.rs:1651:5
   3: core::result::Result<T,E>::expect
             at /rustc/d59363ad0b6391b7fc5bbb02c9ccf9300eef3753/library/core/src/result.rs:1033:23
   4: <futures_cpupool::CpuFuture<T,E> as futures::future::Future>::poll
             at ./cpufuture/src/lib.rs:321:15
   5: core::ops::function::FnOnce::call_once
             at /rustc/d59363ad0b6391b7fc5bbb02c9ccf9300eef3753/library/core/src/ops/function.rs:250:5
   6: futures::task_impl::Spawn<T>::enter::{{closure}}
             at /home/db/.cargo/registry/src/mirrors.ustc.edu.cn-61ef6e0cd06fb9b8/futures-0.1.31/src/task_impl/mod.rs:399:27
   7: futures::task_impl::std::set
             at /home/db/.cargo/registry/src/mirrors.ustc.edu.cn-61ef6e0cd06fb9b8/futures-0.1.31/src/task_impl/std/mod.rs:86:13
   8: futures::task_impl::Spawn<T>::enter
             at /home/db/.cargo/registry/src/mirrors.ustc.edu.cn-61ef6e0cd06fb9b8/futures-0.1.31/src/task_impl/mod.rs:399:9
   9: futures::task_impl::Spawn<T>::poll_fn_notify
             at /home/db/.cargo/registry/src/mirrors.ustc.edu.cn-61ef6e0cd06fb9b8/futures-0.1.31/src/task_impl/mod.rs:291:9
  10: futures_util::compat::compat01as03::Compat01As03<T>::in_notify
             at /home/db/.cargo/registry/src/mirrors.ustc.edu.cn-61ef6e0cd06fb9b8/futures-util-0.3.31/src/compat/compat01as03.rs:39:9
  11: <futures_util::compat::compat01as03::Compat01As03<Fut> as core::future::future::Future>::poll
             at /home/db/.cargo/registry/src/mirrors.ustc.edu.cn-61ef6e0cd06fb9b8/futures-util-0.3.31/src/compat/compat01as03.rs:160:23
  12: <server::GreeterService as demo::helloworld_grpc::Greeter>::say_hello::{{closure}}
             at ./bin/server.rs:44:34
  13: grpcio::task::executor::poll
             at /home/db/.cargo/registry/src/mirrors.ustc.edu.cn-61ef6e0cd06fb9b8/grpcio-0.13.0/src/task/executor.rs:202:15
  14: grpcio::task::executor::resolve
             at /home/db/.cargo/registry/src/mirrors.ustc.edu.cn-61ef6e0cd06fb9b8/grpcio-0.13.0/src/task/executor.rs:142:5
  15: grpcio::task::CallTag::resolve
             at /home/db/.cargo/registry/src/mirrors.ustc.edu.cn-61ef6e0cd06fb9b8/grpcio-0.13.0/src/task/mod.rs:181:39
  16: grpcio::env::poll_queue
             at /home/db/.cargo/registry/src/mirrors.ustc.edu.cn-61ef6e0cd06fb9b8/grpcio-0.13.0/src/env.rs:30:9
  17: grpcio::env::EnvBuilder::build::{{closure}}
             at /home/db/.cargo/registry/src/mirrors.ustc.edu.cn-61ef6e0cd06fb9b8/grpcio-0.13.0/src/env.rs:107:21
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
Got msg
Segmentation fault
```
