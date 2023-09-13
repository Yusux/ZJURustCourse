# HW5

## 简介

在本次作业中，我使用 [Volo](https://github.com/cloudwego/volo) 框架实现一个简单的 Redis 服务，并支持了以下的命令：

- `PING`
- `SET(expire)`
- `GET`
- `DEL`
- `SUBSCRIBE`
- `PUBLISH`

同时编写了一个简单的中间件来过滤命令。为了演示，中间件会过滤包含 `关注嘉然谢谢喵` 的命令，阻止命令的执行并返回 Error。

目录树如下：

``` bash
simple_mini_redis/
├── Cargo.lock
├── Cargo.toml
├── idl
│   └── volo_example.thrift     # Thrift IDL 文件
├── Readme.md
├── rust-toolchain.toml
├── src
│   ├── bin
│   │   ├── client.rs           # 客户端程序
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

## 命令

### PING

``` bash
PING [message]
```

若不带参数，则返回 `PONG`；若带参数，则返回参数。

``` bash
redis> PING
PONG
redis> PING Hello
Hello
```

### SET

``` bash
SET key value [expire time]
```

将 `key` 的值设为 `value`。若 `key` 已存在，则覆盖原值。添加 `expire time` 参数后，`key` 的生存时间为 `expire time` 秒，超过生存时间后 `key` 会被自动删除。

``` bash
redis> SET demokey 1
OK
redis> GET demokey
1
redis> SET demokey 2
OK
redis> GET demokey
2
redis> SET demokey 3 10
OK
redis> GET demokey  # Within 10 seconds
3
redis> GET demokey  # After 10 seconds
(nil)
```

### GET

``` bash
GET key
```

返回 `key` 的值。若 `key` 不存在，则返回 `(nil)`。

``` bash
redis> SET demokey 2
OK
redis> GET demokey
2
redis> GET key_not_exist
(nil)
```

### DEL

``` bash
DEL key
```

删除 `key`。若 `key` 不存在，则返回 `0`，否则返回 `1`。

``` bash
redis> SET demokey 1
OK
redis> GET demokey
1
redis> DEL demokey
1
redis> GET demokey
(nil)
redis> DEL demokey
0
```

### SUBSCRIBE

``` bash
SUBSCRIBE channel
```

> 由于框架 TCP 短连接的特性，客户端每次订阅只能获得一条消息。

订阅 `channel`，等待下一条发送到 `channel` 的消息。

``` bash
redis> SUBSCRIBE channel
1) "subscribe"
2) "channel"
3) (integer) 1  # Another client publish a message "Info" to channel
1) "message"
2) "channel"
3) "Info"
```

### PUBLISH

``` bash
PUBLISH channel message
```

向 `channel` 发送 `message`，并返回客户端数量。

``` bash
redis> PUBLISH channel Info
1
```

### QUIT

``` bash
QUIT
```

退出客户端。

``` bash
redis> QUIT
OK
```

## 运行

项目目录下使用 `cargo run --bin server` 启动服务端，使用 `cargo run --bin client` 启动客户端。客户端可以使用 `--host <host>` 和 `--port <port>` 参数指定服务端的地址和端口。服务端和客户端默认地址为 IPv4 本地地址 `127.0.0.1`，端口为 `38080`。

> 亦可尝试使用 release 版本。

先启动服务端。客户端启动后进入交互模式：

``` bash
redis> 
```

可以输入 [上述命令](#命令) 进行交互。一个简短的示例如下：

``` bash
redis> PING
PONG
redis> PING Hello,World!
Hello,World!
redis> SET mykey 1
OK
redis> GET mykey
1
redis> GET mykey_not_exist
(nil)
redis> SET mykey Hello
OK
redis> GET mykey
Hello
redis> DEL mykey
1
redis> DEL mykey
0
redis> DEL mykey_not_exist
0
redis> GET mykey
(nil)
redis> QUIT
OK
```

包含 `SUBSCRIBE`、 `PUBLISH` 以及中间件过滤的综合演示如下：

![demo](.assets/demo.gif)
