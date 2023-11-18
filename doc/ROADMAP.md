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

- 开源支持
  - 自述文件和项目描述信息
    - [ ] 项目Logo
    - [ ] 设计目标
    - [ ] SECURITY和CREDIT文档
  - 项目介绍
    - [ ] Features
    - [ ] News

### hipparchus-az v0.1.3

- DMS
  - [ ] Conversion from/to text and Debug/Display trait
  - [ ] Add/Sub/Mul/Div trait
  - [ ] Trianglular function and trait
- Azimuth
  - [ ] Trianglular function and trait
  - [ ] Conversion from/to text and Debug/Display trait
- Angle
  - [ ] Define wrapped f64/f32 degrees
  - [ ] Define wrapped f64/f32 radians
  - [ ] Add/Sub/Mul/Div trait
  - [ ] Trianglular function and trait
- LatLon
  - [ ] 经纬度格式的解析
  - [ ] WGS84坐标与GCJ02（火星坐标）的相互转换

### hipparchus-geo v0.1.3

- Geodedic
  - [ ] Geodesic Trait for direct & inverse problem
  - [ ] Auxiliary latitude 
  - [ ] Unit Test

### hipparchus-mean v0.1.3

N/A

### hipparchus-metrics v0.1.3

N/A

### hipparchus-seq v0.1.3

[ ] Refactor

## vBLUE：远期规划

> [!NOTE]
> - （定期或不定期）从vBLUE远期规划里按优先级挑选一些合适的创意和任务放入vNEXT执行计划，或将需暂时搁置的任务从vNEXT转回vBLUE远期规划
> -  平时用vBLUE收集新需求、功能创意和需长期关注领域的重要问题等

### 工程基础

- 单元测试
  - [ ] 在代码覆盖率报告里忽略测试代码

- 代码风格管理
  - [ ] 启用codefmt并配置合适的规则

- BORS机器人
  - [ ] 启用bors.toml

- 开源支持
  - PR请求模板
  - 软件架构图

### hipparchus-mean

- 计算阶乘和双阶乘（实现基于prime swing或recursive split的快速算法）
- 计算排列组合（基于快速阶乘算法）
- 杨辉三角形中数阵：simplex(n维三角数列), binomial expansion（二项式分布）

### hipparchus-geo

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
