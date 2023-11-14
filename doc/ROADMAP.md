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

- 质量管理
  - 单元测试
    - [x] 代码覆盖率报告：后端（llvm-cov + grcov）, 前端（Coverage Gutter）
    - [x] 数据驱动测试：rstest

### hipparchus-mean v0.1.3

  - 新增功能
    - [x] 新增SignedMod计算

  - 文档注释
    - [x] 学习Github Copilot的使用
    - [x] 提供一个简单的crate自述文件
    - [x] 初步完成crate文档注释

  - 完善测试
    - [x] 搭建性能测试框架，并建立对应的Github工作流
    - [x] 增加harmonic sequence的单元测试（解决浮点数组比较问题）

### hipparchus-geo v0.1.3

- 新增功能
  - [x] 坐标轴（经度和纬度）及坐标值正则化
  - [x] 方位、角度单位和坐标值符号的定义
  - [x] 度分秒、经纬度的定义及与坐标值的相互转换
  - [x] 地理区域定义：地区倾角、寒温带和东西半球
  - [x] 把经纬度格式化为NMEA0183/ISO6709字符串
  - [x] WGS84椭球体参数定义，地球的半径、面积和体积的计算
  - [x] 计算球面（地球）两点间的半正矢距离（haversine）
  - [ ] 根据经度计算时区
  - [ ] 经纬度格式的解析

### hipparchus-metrics v0.1.3

  - 模块重构
    - [x] 空间中两点间的距离
    - [x] 空间中两个矢量的相似度或距离
    - [x] 两个统计分布的相似度或距离
    - [x] 两个统计样本的相似度或距离
    - [x] 地球上两点间的距离
    - [x] 两个字符串的相似度或距离

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
  - 自述文件和项目描述信息
    - 项目Logo
    - 设计目标
    - SECURITY和CREDIT文档
  - 项目介绍
    - Features
    - News
  - PR请求模板
  - 软件架构图

### hipparchus-mean

- 计算阶乘和双阶乘（实现基于prime swing或recursive split的快速算法）
- 计算排列组合（基于快速阶乘算法）
- 杨辉三角形中数阵：simplex(n维三角数列), binomial expansion（二项式分布）

### hipparchus-geo

- 改进测地线的正算和反算
  - [x] 引入geographiclib-rs作为测地线求解的Rust基础实现
  - [x] 利用bitflags重构caps和mask的实现
  - [x] 移除static_lazy定义
  - [x] 重新定义平方根和立方根trait
  - [x] 利用椭球体trait作为geodesic初始化
  - [x] 提取coeff系数为独立模块
  - [x] 把GEODESIC_ORDER改为usize类型，去掉多余的类型转换
  - [x] 把成员GEODESIC_ORDER和TINY改为常量
  - [ ] 定义完整椭圆体参数并替换冗余实现
  - [ ] 逆向测地线求解，两个坐标点之间的方位（和距离）
  - [ ] 正向测地线求解。相对于坐标点的特定方位与距离的目标点

- [ ] 计算扁球面（地球）两点间的测地线距离（geodesic）
- [ ] WGS84坐标与GCJ02（火星坐标）的相互转换

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
