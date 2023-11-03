# 路线图

## 项目发布策略

视缺陷数量、功能需求及社区活跃度而定，平均每2~6个月发布一个新版本。

- 主版本
  - v0，原型版或实验版，发布更频繁，不承诺接口向下兼容
  - v1+，按[semver](https://semver.org/)语义，承诺接口向下兼容（尽可能）

- 次版本
  - 偶数：奇数版本发布以增加新功能为主（0.x原型版或实验版与约定相反）
  - 奇数：偶数版本发布以完善既有功能为主（0.x原型版或实验版与约定相反）

目前，主要版本规划如下：
  - [x] v0.1, 熟悉Rust语言及项目开发流程，实现并发布一个最小可用的工程原型
  - [ ] v0.2, 补充性能测试，完善开发文档，丰富开源协作体验
  - [ ] v0.9, 针对用户反馈持续优化，并完善所支持的度量算法，发布最终测试版（至少40+种以上度量计算）
  - [ ] v1.0, 发布正式版（基于semver承诺接口向下兼容）
  - [ ] v1.x, 修复缺陷并持续维护，视社区活跃度而定，保持平均每2-6个月发布一个新的稳定版（基于semver承诺接口向下兼容）

## vNEXT：执行计划

> [!NOTE]
> - （定期或不定期）从vBLUE远期规划里按优先级挑选一些合适的创意和任务放入vNEXT执行计划，或将需暂时搁置的任务从vNEXT转回vBLUE远期规划
> - 平时用vNEXT来追踪当前待发布版本主要任务目前的开发状态
> - 对外发布新版本时从vNEXT执行计划中提取相关信息，编写相应的版本发行说明

### 工程基础

- 开源文档
  - 项目组织
    - [x] 创建入口crate，提供子crates的re-exports（参考num）
    - [x] 拆分独立的hipparchus-geo项目
  - 自述文件与项目描述信息
    - [x] 增加下载数徽章和codecov.info徽章
    - [x] 增加使用警告信息
    - [x] 独立ENLISTMENT文档

- Github集成
  - 工作流
    - [x] 配置工作流触发条件，支持手动触发和定期触发
    - [x] 集成代码覆盖率统计到Github工作流中
    - [x] 集成性能测试到Github工作流中
  - 问题描述模板
    - [x] 缺陷报告
    - [x] 功能建议
  - 标签和里程碑
    - [x] 定义标签和里程碑
    - [x] 为所有历史PR补齐标签和里程碑的Tag

- 单元测试
  - [x] 代码覆盖率报告：后端（llvm-cov + grcov）, 前端（Coverage Gutter）
  - [x] 数据驱动测试：rstest
  - [ ] 高性能测试容器：nextest

### 发布 hipparchus-mean v0.2

  - 文档注释
    - [x] 学习Github Copilot的使用
    - [x] 提供一个简单的crate自述文件
    - [x] 初步完成crate文档注释

  - 完善测试
    - [x] 搭建性能测试框架，并建立对应的Github工作流
    - [x] 增加harmonic sequence的单元测试（解决浮点数组比较问题）

### 开发 hipparchus-geo （当前工作版本暂不计划对外发布）

  - 新增功能
    - [x] 方位和坐标的定义
    - [x] 度分秒及其和对应坐标值的相互转换
    - [ ] 经纬度、度分秒与字符串的相互转换
    - [ ] 经纬度的加减法和比较
    - [ ] 度分秒的加减法和比较
    - [ ] WGS84椭球体参数定义
    - [ ] GCJ02椭球体参数定义

### 开发 hipparchus-metrics （当前工作版本暂不计划对外发布）

  - 摸索代码重构方法
    - [x] 统一Fp浮点定义，去掉不必要的包依赖
    - [x] 利用迭代器的map和fold操作，简化并统一高维矢量空间的求解计算代码
    - [x] 合并同类metrics计算，精简设计
    - [x] 增加测试以验证对称性，即交换x、y后距离不变
    - [x] 增加测试以验证重合性，即x和其自身的距离为0

  - 模块重构进展
    - [x] 空间两点的距离
    - [x] 两个矢量的相似度或距离
    - [ ] 地球两点的距离
    - [ ] 两个字符串的相似度或距离
    - [ ] 两个集合的相似度或距离
    - [ ] 两个统计分布的相似度或三都
    - [ ] 两个样本序列的相似度或距离

## vBLUE：远期规划

> [!NOTE]
> - （定期或不定期）从vBLUE远期规划里按优先级挑选一些合适的创意和任务放入vNEXT执行计划，或将需暂时搁置的任务从vNEXT转回vBLUE远期规划
> -  平时用vBLUE收集新需求、功能创意和需长期关注领域的重要问题等

### 工程基础

- 改善测试覆盖率
  - 在代码覆盖率报告里忽略测试代码
  - 调研llvm-cov代码覆盖率统计不精确的问题

- 代码风格统一管理
  - 启用codefmt并配置合适的规则

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
  - 软件架构图

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
