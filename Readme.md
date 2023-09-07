# HW3

## Exercise 1

### 简介

使用宏进行 Hashmap 的初始化。

目录树如下所示：

```shell
macro_hashmap/
├── Cargo.lock
├── Cargo.toml
└── src
    └── main.rs # 包含宏定义和测试代码
```

### 测试

项目目录下使用 `cargo test` 进行测试，也可以使用 `cargo run` 运行示例程序。

## Exercise 2

### 简介

实现一个简单的 `Rc` 类型，实现引用计数，要求实现

- `new` 函数
- `clone` 函数
- `Deref` trait
- 自动回收内存

> 为了让多个结构体共享相同的内存地址，除了采用 unsafe 的方式使用指针来访问，我暂时还没有找到其他合适的方法。

目录树如下所示：

```shell
my_rc/
├── Cargo.lock
├── Cargo.toml
└── src
    ├── main.rs     # 测试代码
    └── my_rc.rs    # 实现 MyRc 的代码
```

### 测试

项目目录下使用 `cargo test` 进行测试。也可以使用 `cargo run` 运行示例程序，示例程序中有使用 `println!` 打印引用计数，可以观察到引用计数的变化。

## Exercise 3

### 简介

利用 `RefCell` 实现一个具有内部可变性的栈结构体，具有：

- `new` 函数
- `push` 函数
- `pop` 函数

目录树如下所示：

```shell
my_stack/
├── Cargo.lock
├── Cargo.toml
└── src
    ├── main.rs     # 测试代码
    └── my_stack.rs # 实现 MyStack 的代码
```

### 测试

项目目录下使用 `cargo test` 进行测试。也可以使用 `cargo run` 运行示例程序。
