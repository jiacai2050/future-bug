# Future bug demo


The cpufuture in this repo is [futures-cpupool 0.1.8](https://docs.rs/crate/futures-cpupool/0.1.8)，I vendor it here to add debug message.

## How to reproduce

```
cargo build --release
gdb ./target/release/demo
```

Then following errors will be thrown:

```
Got msg
Got msg done
Poll ok, NotReady
Got msg
Got msg done
Poll err:Canceled
thread 'main' panicked at /home/jiacai/grpc-rs-demo/cpufuture/src/lib.rs:332:14:
cannot poll CpuFuture twice: Canceled
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
Got msg
free(): double free detected in tcache 2
Aborted
```

When running in debug mode, the error will be gone!
