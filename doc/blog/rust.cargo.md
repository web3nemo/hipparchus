# Rust项目组织

## Workspace, Package, Crate, Module

- Workspace：类似monorepo思想，源码中多个package/crate共享环境依赖，为开发管理便利而定义的统一的工作区
- Package: 可独立发布的包，可包含最多一个库crate和任意数量的二进制crate，通过`cargo.toml`来定义
- Crate：同一个独立发布的package下的一个模块，可以是lib，也可以是可执行的二进制文件
    - 库crate通过package根目录下的`lib.rs`定义，最多只能有1个
    - 二进制crate可以有任意多个，默认用package根目录下的`main.rs`定义，也可以在`bin`子目录下扩展定义更多的独立binary
- Module: 同一个crate下在不同源文件中可树型嵌套定义的子模块，类似于其它高级语言中的namespace
    - 源码目录和源文件，自动以文件名和目录名为namespace，构造成独立的子模块
    - 不同子模块之间通过self、super、crate来相互引用和访问

> 几个容易造成误解的地方：
> - crates.io实际上发布和管理的最小单位并不是crate，而是crates，也就是package
- - cargo管理项目依赖时本来应该指定的是package下的lib crate，但由于约定一个package下最多只能有一个lib crate，所以指定package及其版本就足够了
> - cargo实际上可以通过`cargo run --bin`或`cargo install --bin`来指定一个package下的特定crate

下面是一个Rust工程的源码结构：
```sh
.                                                   # Rool of workspace    
├── Cargo.toml                                      # Cargo.toml for Workspace 
├── Cargo.lock
├── crates
│   ├── pkg1                                        # Root of package
│   │   ├── Cargo.toml                              # Cargo.toml for package
│   │   ├── Cargo.lock
│   │   ├── src
│   │   │   ├── mod1                                
│   │   │   │   ├── mod.rs                          # Entry mod.rs for a module (folder)    
│   │   │   │   ├── ...
│   │   │   │   └── sub-mod.rs                      
│   │   │   ├── ...
│   │   │   ├── mod2.rs                             # Module defined as source file
│   │   │   ├── main.rs                             # Default binary crate
│   │   │   ├── lib.rs                              # Default lib crate
│   │   │   └── bin
│   │   │       ├── main1.rs                        # More binary crate
│   │   │       └── ...
│   │   ├── tests
│   │   │   ├── some_integration_tests.rs           # Integration tests
│   │   │   └── ...
│   │   ├── benches
│   │   │   ├── simple_bench.rs                     # Benckmark tests    
│   │   │   └── ...
│   │   └── examples
│   │       ├── simple_example.rs                   # Example codes
│   │       └── ...
│   ├── pkg2
│   └── ...

```

## 引用Module

TODO

## 发布Package

TODO: dry-run

## 自建crates仓库

TODO
