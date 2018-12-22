# 第14章 更多关于cargo和crates.io

## 1 采用发布配置的自定义构建

* cargo通常有两种配置：dev和release
* 可以在cargo.toml中的profile.dev和profile.release节中为dev和release增加配置，以覆盖默认配置
* opt-level配置项定义优化级别，可用取值范围为[0,3]，值越大则进行越多的优化
* cargo的命令行选项--vcs none表示在新建项目的时候不初始化git仓库
* cargo update [可选的包名]命令更新依赖信息，重写cargo.lock
* 两个配置文件
  * cargo.toml 由程序员维护，不仅仅包含依赖信息，还包含其他元数据
  * cargo.lock 包含精确的依赖信息，由cargo维护，不应该手动编辑。
    * 对于二进制程序，应该将cargo.lock包含在代码仓库中
    * 库项目不应该包含cargo.lock，应该将cargo.lock添加到.gitignore中

## 2 将crate发布到crates.io

### 2.1 文档注释

* 用```///```来在项目（如函数、模块）前增加文档注释；注释可以使用markdown格式
* 用cargo doc命令来生成文档；如果带--open参数，可生成文档后自动用浏览器打开文档
* 执行cargo test时，文档注释中的代码会作为文档测试(Doc-tests)执行，结果会显示在命令输出中
* 用```//!```来在项目（如函数、模块）中增加文档注释，通常用在包的根文件或者模块的根文件开始处

### 2.2 使用pub use导出合适的公有API

* 包组织复杂时，用户需要了解包的结构，然后使用多级导入，如```use art::kinds::PrimaryColor```
* 不仅使用不方便，而且用户需要了解包结构；此外，文档的首页也不会显示内层导出的类型
* 为解决上述问题，可以使用```pub use```进行重新导出，如在art包中增加```pub use kinds::PrimaryColor```
* 这样用户就可以使用```use art::PrimaryColor```了，而且文档首页会在Reexports节列出重新导出的定义，方便用户查阅

### 2.3 注册crates.io账号

* 当前支持用github.com账号注册和登录
* 登录后获取自己的API Token
* 然后用```cargo login <API Token>```命令来将API Token存储到本地配置文件中，供后续使用

### 2.4 准备发布

* 注意包名称，本地使用时任意，发布时包名称需要在整个crates.io网站唯一
* 必须指定license类型
* 最终的cargo.toml示例

```toml
[package]
name = "guessing_game"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
description = "A fun game where you guess what number the computer has chosen."
license = "MIT OR Apache-2.0"

[dependencies]
```

### 2.5 发布

* 使用cargo publish命令发布包到crates.io网站，发布是永久的，不能覆盖或者删除已经发布的版本
* 使用cargo yark --vers 版本号  来撤销某版本，这并不是从网站删除该版本的数据，只是阻止新的项目使用这个版本
* 使用cargo yark --vers 版本号 --undo 来撤销撤销操作

## 3 Cargo工作空间

* 工作空间cargo.toml不包含package节，而是包含workspace节，其members字段指示工作空间包含的package

```toml
[workspace]
members = [
  "adder",
  "add-one",
]
```

* 目录结构如下

```toml
├── Cargo.lock
├── Cargo.toml
├── add-one
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

* 执行cargo run命令时，需要用-p参数指示要运行哪个package
* 类似地，执行cargo test命令时，可以用-p参数指示要为哪个crate运行测试
* 必须单独为每个crate执行cargo publish命令，不能用一个命令发布工作空间中的所有crate
* 依赖的外部crate应该写在工作空间的cargo.toml中的dependencies节，而不是写在每个package的cargo.toml中

## 4 使用 cargo install 从 Crates.io 安装二进制文件

* cargo install命令从crates.io网站下载二进制目标文件，安装到Rust安装目录中的bin子目录中

## 5 Cargo自定义扩展命令

* 如果存在系统搜索路径(由PATH环境变量指示)中有cargo-something可执行文件，则可以用cargo something来执行这个文件
* 执行cargo --list可以列出所有自定义命令
