# Rust测试

## 单元测试

### 单元测试框架

如果只用Rust标准库所内置的单元测试框架，功能还是比较有限的，实践中很容易造成大量重复测试代码，维护成本提高。

这里推荐Rust工程默认引入[rstest](https://crates.io/crates/rstest)作为单元测试框架。它所支持的功能很多，其中最常用和重要的基本功能如下：
- 数据驱动测试（Parameterize）：非常有用！
- 上下文和初始化（Fixture）
- 超时检测（Timeout）

### 浮点数的比较

众所周知，浮点数是不适合直接做相等判断的。单元测试里如果需要用到浮点数相等的断言，建议使用[float_cmp](https://crates.io/crates/float-cmp)。

常规需求用这个库所提供的`assert_approx_eq!`宏就好，它的用法和`assert_eq`很相似，可以根据数据类型自动选择对应的浮点数相等判别的默认规则。如果需要使用自定义的比较精度，也可通过传入ulps和epsilon参数来实现。

`assert_approx_eq!`宏也支持浮点集合（数组、vec和slice）的比较，具体用法可参考下面示例代码：

``` rs

    #[test]
    fn test_sequence_harmonic_f32()
    {
        let expected = vec!
        [
            1.0/1.0,    1.0/2.0,    1.0/3.0,    1.0/4.0,    1.0/5.0, 
            1.0/6.0,    1.0/7.0,    1.0/8.0,    1.0/9.0,    1.0/10.0, 
        ];
        let n = expected.len();
        let actual = Sequence::Harmonic::<f32> { init: 1.0, difference: 1.0 }.vec(n);
        assert_approx_eq!(&[f32], &expected, &actual);
    }

```

要稍加注意的是，和`assert_eq!`不一样，`assert_approx_eq!`并不支持自定义断言失败提示信息。

### 带泛型参数的单元测试

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

为了得到Rust单元测试的代码覆盖率，我们通常会组合以下几个工具：
- [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov)：通过LLVM Instrumentation获得代码覆盖率的原始数据
- [grcov](https://github.com/mozilla/grcov)：加工数据生成人类阅读友好的代码覆盖率报告
- [codecov.io](https://codecov.io/)：在CI/CD发布流程中把代码覆盖率报告上传到相关网站做展示

## 集成测试

TODO

## 性能测试

官方的benchmark工具只在unstable上可用，而且功能限制比较多。推荐用以下组合代替：
- [criterion.rs](https://crates.io/crates/criterion)：性能测试框架，生成统计数据
- [gnuplots]()：把性能测试的结果渲染成丰富样式的图表，`criterion.rs`在ubuntu上运行时会自动集成

