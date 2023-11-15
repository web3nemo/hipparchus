# Rust测试

## 单元测试

### 单元测试框架

如果只用Rust标准库所内置的单元测试框架，功能还是比较有限的，实践中很容易造成大量重复测试代码，维护成本提高。推荐Rust工程默认引入[rstest](https://crates.io/crates/rstest)作为单元测试框架，它所支持的一部分重要功能如下：
- 数据驱动测试（Parameterize）
- 上下文和初始化（Fixture）
- 超时检测（Timeout）

rstest的标准用法可在官方文档里查阅，在这里略补充一些为泛型写单元测试的技巧：

1. 使用impl trait作为case类型

``` rs

    #[rstest]
    #[case(-180i128, Remainder::Euclidean)]
    #[case(-180i64, Remainder::Euclidean)]
    #[case(-180i32, Remainder::Euclidean)]
    #[case(-180i16, Remainder::Euclidean)]
    #[should_panic]
    fn test_angle_norm_radians_panic(#[case] v: impl Angle, #[case] re: Remainder)
    {
        v.norm_radians(re);
    }

```

- 优点：简单好用
- 限制：很难用于需要泛型类型定义的复杂场景

2. 把测试函数定义为泛型测试，并设计相关的泛型参数约束

``` rs

    #[rstest]
    #[case(-std::f64::consts::PI, Remainder::Euclidean, std::f64::consts::PI)]
    #[case(-std::f64::consts::PI, Remainder::Symmetry, -std::f64::consts::PI)]
    #[case(-std::f64::consts::PI, Remainder::InvertedSymmetry, std::f64::consts::PI)]
    #[case(-std::f32::consts::PI, Remainder::Euclidean, std::f32::consts::PI)]
    #[case(-std::f32::consts::PI, Remainder::Symmetry, -std::f32::consts::PI)]
    #[case(-std::f32::consts::PI, Remainder::InvertedSymmetry, std::f32::consts::PI)]
    fn test_angle_norm_radians<T>(#[case] v: T, #[case] re: Remainder, #[case] expected: T) where
        T: Copy + Debug + Angle + ApproxEq
    {
        let actual = v.norm_radians(re);
        assert_approx_eq!(T, expected, actual);
    }

```

- 优点：几乎没有限制
- 限制：定义测试函数时语法略复杂，但比较规范

### 代码覆盖率

TODO

## 集成测试

TODO

## 性能测试

TODO
