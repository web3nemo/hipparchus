# Issues@Rust

以下列表是开发过程中发现的Rust语言目前存在的一些限制和问题。

- 单例模式
  - [ ] 不支持懒加载初始化 [#![feature(lazy_cell)]](https://github.com/rust-lang/rust/issues/109736)

- 浮点常量的初始化
  - [ ] 编译期计算函数无法定义为常量 [#![feature(const_fn_floating_point_arithmetic)]](https://github.com/rust-lang/rust/issues/57241)
  - [ ] 泛型特性里定义常量无法使用类型转换函数 [#![feature(generic_const_exprs)]](https://github.com/rust-lang/rust/issues/76560)

- 泛型
  - [ ] 不支持偏特化 [#![feature(specialization)]](https://github.com/rust-lang/rust/issues/31844)

- 更多浮点类型
  - [ ] 不支持f16: IEEE-754 binary16
  - [ ] 不支持f128: IEEE-754 binary128)
  - [ ] 不支持x86扩展精度浮点数f80
  - [ ] 不支持bf16、tf32等其它机器学习领域常用的特殊浮点数格式
