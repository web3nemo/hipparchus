# 路线图

## 项目发布策略

视缺陷数量、功能需求及社区活跃度而定，平均每2~6个月发布一个新版本。

- 主版本
  - v0，原型版或实验版，发布更频繁，不承诺接口向下兼容
  - v1+，按[semver](https://semver.org/)语义，承诺接口向下兼容（尽可能）

- 次版本
  - 偶数：奇数版本发布以增加新功能为主（0.x原型版或实验版与约定相反）
  - 奇数：偶数版本发布以完善既有功能为主（0.x原型版或实验版与约定相反）

## 里程碑

- [Done] v0.1, 熟悉Rust语言及项目开发流程，实现并发布一个最小可用的工程原型
- [In Progress] v0.2, 补充性能测试，完善开发文档，丰富开源协作体验
- [Not Started] v0.9, 针对用户反馈持续优化，并完善所支持的度量算法，发布最终测试版（至少40+种以上度量计算）
- [Not Started] v1.0, 发布正式版（基于semver承诺接口向下兼容）
- [Not Started] v1.x, 修复缺陷并持续维护，视社区活跃度而定，保持平均每2-6个月发布一个新的稳定版（基于semver承诺接口向下兼容）

## vNEXT：执行计划

> [!NOTE]
> - （定期或不定期）从vBLUE远期规划里按优先级挑选一些合适的创意和任务放入vNEXT执行计划，或将需暂时搁置的任务从vNEXT转回vBLUE远期规划
> - 平时用vNEXT来追踪当前待发布版本主要任务目前的开发状态
> - 对外发布新版本时从vNEXT执行计划中提取相关信息，编写相应的版本发行说明

### 工程基础

- 开源支持
  - 自述文件与项目描述信息
    - [x] 下载数徽章
    - [x] 警告信息
    - [x] 独立的ENLISTMENT文档
  - 标签和里程碑
    - [x] 标签：Severity#Bug|Perf|Feature|Doc|Question
    - [x] 里程碑：H|He|...
  - 问题描述模板
    - [x] 缺陷报告
    - [x] 功能建议

- 开发文档
  - [x] 路线图
  - [ ] 软件架构图

### 发布 hipparchus-mean v0.2

  - 彻底解决浮点数比较问题
    - [ ] 提供对&[T]或Iterator<T>的浮点比较能力（废弃可能已疏于维护的float-cmp）
    - [ ] 增加harmonic sequence的单元测试
    - [ ] 用新assert断言宏改写既有单元测试

  - 性能测试
    - [ ] 搭建性能测试框架
    - [ ] 尝试分析并提高性能
    - [ ] 发布性能测试报告

### 开发 hipparchus-metrics （当前工作版本暂不计划对外发布）

  - 重构现有代码
    - [ ] 利用hipparchus-mean的Fp浮点数定义，提供更易于管理的统一值类型定义
    - [ ] 重新组织代码结构，合并同类metrics计算，精简设计

## vBLUE：远期规划

> [!NOTE]
> - （定期或不定期）从vBLUE远期规划里按优先级挑选一些合适的创意和任务放入vNEXT执行计划，或将需暂时搁置的任务从vNEXT转回vBLUE远期规划
> -  平时用vBLUE收集新需求、功能创意和需长期关注领域的重要问题等

### 工程基础

- 入口crate
  - 参考num-traits，提供入口crate对多crate做re-exports

- 改善测试覆盖率
  - 如何在覆盖率报告里忽略测试代码
  - 调研llvm-cov代码覆盖率统计不准确的问题
  - 集成到CI流水线中

- 代码风格统一管理
  - 启用codefmt
  - 配置合适的规则

- 开源支持
  - 自述文件和项目描述信息
    - 项目Logo
    - 设计目标
    - SECURITY文档
    - CREDIT文档
  - 项目介绍
    - Features
    - News
  - PR请求模板

### hipparchus-mean

  - 计算阶乘和双阶乘（实现基于prime swing或recursive split的快速算法）
  - 计算排列组合（基于快速阶乘算法）
  - 杨辉三角形中数阵：simplex(n维三角数列), binomial expansion（二项式分布）

### hipparchus-metrics

- 支持更多距离度量算法
  - kulczynski distance
  - lorentzian distance, intersection & non-intersection distance, refer to https://github.com/drostlab/philentropy
  - wave hedges distance & vicis wave hedges distance, refer to https://github.com/aziele/statistical-distance 
  - moid distance (minimum orbit intersection distance)
  - AMOVA distance, unifrac distance, ladder distance
  - ngd distance (normalized google distance)
  - Nei’s genetic distance, conditional genetic distance
  - wasserstein distance

## 更多探索

- 学习和调研更多距离度量算法
  - motyka
  - tanimoto
  - ruzicka
  - harmonic_mean
  - fidelity
  - bhattacharyya
  - matusita
  - squared_chord
  - squared_euclidean
  - pearson
  - neyman
  - squared_chisq
  - prob_symm
  - divergence
  - clark
  - additive_symm
  - kullback-leibler
  - jeffreys
  - k_divergence
  - topsoe
  - jensen_difference
  - taneja
  - kumar-johnson
  - avg
  - acc
  - add_chisq
  - marylandbridge
  - max_symmetric_chisq
  - neyman_chisq
  - pearson_chisq
  - penroseshape
  - vicis_symmetric_chisq
