# My Runtime

## 简介

使用 Rust 实现一个简单的异步运行时，基本要求为实现课程内的单线程 `Runtime`，进阶要求为增加多线程支持（基本实现）和实现任务调度（未完成）。

> 个人能力有限，实现不足之处还请指出

目录树如下所示：

```shell
my_runtime/
├── Cargo.lock
├── Cargo.toml
├── examples
│   ├── calculate.rs
│   └── demo.rs
├── Readme.md
└── src
    ├── executor.rs  # 新建 Executor 时可以传入 usize 类型的参数，表示线程池中线程的数量
    ├── lib.rs
    └── waker.rs
```

## 测试

项目目录下使用 `cargo test` 进行测试，可以对 `src/lib.rs` 中的简单异步程序进行测试。

## 示例

在 `examples` 目录下有两个示例程序，分别为 `demo.rs` 和 `calculate.rs`。

`demo.rs` 包含对 spawn task、多逻辑核占用、任务能否在不 spin 状态下被外部唤醒的测试，可以通过 `cargo run --example demo` 运行。

`calculate.rs` 包含对多线程运行时相较于单线程运行时的性能提升的测试，默认线程池中线程数量为 4，可以通过 `cargo run --example calculate` 运行。