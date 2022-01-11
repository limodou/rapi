# rapi
一个用来管理 API 的 WEB 工具，目的是为了学习 Rust。本项目刚开始还在不断优化。

## 依赖库

* toiko 异步框架
* poem 原来是使用 axum，但是感觉有些地方做起来不方便，就切换到了 poem。
* sqlx 数据库访问，不过的确有些原始。后面准备转向 SeaOrm。
* sqlx-cli 用来做数据迁移
* naiveadmin 使用 naive-ui 前端库

## 数据库初始化

参数 sqlx-cli 的安装说明，先安装 sqlx 的命令行工具。

```
cargo install sqlx-cli
```

执行：

```
sqlx database create --database-url mysql://username:password@localhost:port/rapi
```

注意替换上面的值。

## 后台服务启动

工程下载下来之后，在工程目录下创建 .env，里面填入：

```
DB_CONNECTION = mysql://username:password@localhost:port/rapi
JWT_SECRET = yoursecret
STATIC_DIR = ./rapi-web
PORT = 5000
```
根据需要对上面值进行修改

然后执行：

```
cargo run -p rapi-server
```

## 前台开发环境启动

初始化

```
cd rapi-web
yarn
```

因为使用了 Vue3, Vite2 所以建议 nodejs 版本为14+。

启动前端

```
yarn dev
```

## license

MIT