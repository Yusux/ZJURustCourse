# HW6

## 课堂练习

### 简介

使用 [reqwest](https://github.com/seanmonstar/reqwest) 实现了一个比较简单的 HTTP 信息获取程序，可以打印出指定 URL 的 HTML 内容。

目录树如下：

``` bash
use_reqwest/
├── Cargo.lock
├── Cargo.toml
└── src
    └── main.rs
```

### 使用

进入 `use_reqwest` 项目目录，通过 `cargo run <input uri>` 运行程序，其中 `<input uri>` 为输入的 URI。例如：

``` bash
> cargo run http://baidu.com
HTTP/1.1 200 OK
date: Wed, 13 Sep 2023 08:25:46 GMT
server: Apache
last-modified: Tue, 12 Jan 2010 13:48:00 GMT
etag: "51-47cf7e6ee8400"
accept-ranges: bytes
content-length: 81
cache-control: max-age=86400
expires: Thu, 14 Sep 2023 08:25:46 GMT
connection: Keep-Alive
content-type: text/html

<html>
<meta http-equiv="refresh" content="0;url=http://www.baidu.com/">
</html>
```

## 简易 HTTP Server

在上一份作业的基础上，利用 [axum](https://github.com/tokio-rs/axum) 实现了一个比较简单的 HTTP Server，通过接受带有相关参数的 POST 请求，对 Redis 进行操作，并返回结果。提交的参数包括：

- `submit`: 如通过 `reqwest` 等工具提交的请求，必须设置为 `true`。
- `key`: 为 Redis 的 `key` 或是订阅的 `channel`。
- `value`: 为 Redis 的 `value` 或是发布的 `message`。
- `expire`: 为 Redis 的 `expire time`。

前一次实验中的中间件过滤器仍然有效，也有 axum 的中间件用来过滤一些不符合条件的请求。

目录树如下：

``` bash
mini_http_server/
├── Cargo.lock
├── Cargo.toml
├── examples
│   └── demo.rs                 # 示例程序
├── Readme.md
├── src
│   ├── main.rs                 # HTTP Server 程序
│   └── utils.rs                # 工具函数
├── src
│   ├── del.html
└── static                      # 静态 HTML 文件
    └── html
        ├── del.html
        ├── get.html
        ├── index.html
        ├── ping.html
        ├── publish.html
        ├── set.html
        └── subscribe.html
simple_mini_redis/
├── Cargo.lock
├── Cargo.toml
├── idl
│   └── volo_example.thrift     # Thrift IDL 文件
├── rust-toolchain.toml
├── src
│   ├── bin
│   │   └── server.rs           # 服务端程序
│   ├── client_fns.rs           # 客户端使用的函数
│   └── lib.rs                  # 服务端实现
└── volo-gen
    ├── build.rs
    ├── Cargo.toml
    ├── src
    │   └── lib.rs
    └── volo.yml
```

## 运行

### 启动 Redis

首先进入 `simple_mini_redis` 目录，通过 `cargo run --bin server` 启动 Redis 服务端程序，该程序会监听 `[::]:6379` 地址。

### 启动 HTTP Server

然后进入 `mini_http_server` 目录，通过 `cargo run` 启动 HTTP Server 程序，该程序会监听 `[::]:3000` 地址。

### 测试

可以通过访问服务开启的 `http://<server ip>:3000` 地址来访问 HTTP Server 的静态页面进行相关操作。

在 `mini_http_server/src/examples` 目录下有一个 `demo.rs` 示例程序，可以进入 `mini_http_server` 通过 `cargo run --example demo` 来运行该程序，该程序会向 HTTP Server 发送一些请求，测试 HTTP Server 的功能，示例程序的输出如下：

``` bash
> cargo run --example demo
    Finished dev [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/demo`
All tests passed!
```
