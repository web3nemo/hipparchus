# Rust存在的问题

- [ ] 单例模式：不支持懒加载语法

``` rs

```

- 浮点常量初始化
  - [ ] 编译期计算函数无法定义为常量
  - [ ] 泛型特性里定义常量无法使用类型转换函数 [feature(generic_const_exprs)](https://github.com/rust-lang/rust/issues/76560)

``` rs
const M:Self = Self::from_i32(360).unwrap();
const N:Self = 360 as Self;
```


- [ ] 模板偏特化：不支持

