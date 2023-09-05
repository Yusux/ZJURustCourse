# HW2

## Exercise 1

### 简介

在这个练习中，需要实现自己的 `Buffer<T>`，其中只有一个成员 `Vec<T>`，且至少实现一个方法 `sum` 来返回所有成员的和。

目录树如下所示：

```shell
buffer/
├── Cargo.lock
├── Cargo.toml
└── src
    ├── buffer.rs   # 实现 Buffer<T> 的代码
    ├── main.rs     # 测试 Buffer<T> 的代码
    └── point.rs    # 结构体 Point 的代码
```

其中 Buffer<T> 具有

- `new` 方法，用于创建一个新的 `Buffer<T>`，并将其初始化为空。
- `sum` 方法，用于返回 `Buffer<T>` 中所有元素的和。
- `push` 方法，用于向 `Buffer<T>` 中添加一个元素。
- `remove` 方法，用于移除 `Buffer<T>` 中的指定位置的元素。

### 测试

为了测试 `Buffer<T>` 的正确性，我使用了 `i32` `i64` `f32` `f64` 以及结构体 `Point` 作为泛型参数，多次插入删除数值，多次调用 `sum` 分别测试了 `sum` 方法的正确性。测试代码见 `buffer/src/main.rs` 中单元测试部分。可以使用 `cargo test` 进行测试。

```shell
> cargo test 
    Finished test [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/main.rs (target/debug/deps/buffer-edcb1f95d96101de)

running 6 tests
test tests::test_empty ... ok
test tests::test_f32 ... ok
test tests::test_f64 ... ok
test tests::test_i32 ... ok
test tests::test_point ... ok
test tests::test_i64 ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Exercise 2

### 简介

这里我们需要实现一个函数 `fn compareString(x: &str, y: &str) -> bool`，用于比较两个字符串之间的字典序大小，`x` 大于 `y` 返回 `true`，否则返回 `false`。

目录树如下所示：

```shell
cmpstr/
├── Cargo.lock
├── Cargo.toml
└── src
    ├── cmpstr.rs   # 实现 compareString 的代码
    └── main.rs     # 测试 compareString 的代码
```

实现思路为利用 `String.chars()` 方法将字符串转换为字符迭代器，然后逐个比较同位置的字符，如果：

- `x` 的字符和 `y` 的字符相等，继续比较下一个字符。
- `x` 的字符比 `y` 的字符大，返回 `true`；
- `x` 的字符比 `y` 的字符小，返回 `false`；
- `x` 尚有字符，而 `y` 已经没有字符，返回 `true`；
- `x` 已经没有字符，而 `y` 尚有字符，返回 `false`。

### 测试

为了测试 `compareString` 的正确性，我根据长度相同、长度不同、有空字符串、有英文以外字符等情况进行测试，测试代码见 `cmpstr/src/main.rs` 中单元测试部分。可以使用 `cargo test` 进行测试。

```shell
> cargo test 
    Finished test [unoptimized + debuginfo] target(s) in 0.18s
     Running unittests src/main.rs (target/debug/deps/cmpstr-2c84a80a47346346)

running 9 tests
test tests::test_back_longer ... ok
test tests::test_empty_back ... ok
test tests::test_empty ... ok
test tests::test_empty_front ... ok
test tests::test_front_longer ... ok
test tests::test_same_length_back_bigger ... ok
test tests::test_same_length_front_bigger ... ok
test tests::test_same_length_same_order ... ok
test tests::test_unicode ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Exercise 3

### 简介

在这个练习中，需要使用闭包和迭代器从原先的 `Vec<char>` 中生成一个新的 `Vec<char>`，内容变化为 `[a, b, c, d, e]` 到 `[b, c, d, e, f]`。

目录树如下所示：

```shell
iterclosure/
├── Cargo.lock
├── Cargo.toml
└── src
    └── main.rs # 实现迭代器和闭包及测试的代码
```

实现思路为获得 `Vec<char>` 的迭代器，然后使用 `map` 方法来给每个字符套用转换的闭包且获得一个新的迭代器，最后使用 `collect` 方法将迭代器转换为 `Vec<char>`。转换的闭包为 `|c| char::from_u32(*c as u32 + 1).unwrap()`，将字符的 Unicode 编码作为 `u32` 加 1 后转换为字符。

### 测试

由于题目明确说明将 `[a, b, c, d, e]` 转换为 `[b, c, d, e, f]`，因此我只测试了这一种情况，测试代码见 `iterclosure/src/main.rs` 中单元测试部分。可以使用 `cargo test` 进行测试。

```shell
> cargo test
    Finished test [unoptimized + debuginfo] target(s) in 0.03s
     Running unittests src/main.rs (target/debug/deps/iterclosure-963850296202b4c0)

running 1 test
test tests::test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
