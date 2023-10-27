# Roadmap

## Milestones

- v0.1, 熟悉Rust设计开发流程，实现一个最小可用的工程原型（10+种以上度量计算，不承诺接口兼容性）
- v0.5, 优化设计，正式开源并发布测试版，收集用户反馈（30+种以上度量计算，不承诺接口兼容性）
- v0.9, 针对用户反馈持续优化，持续完善所支持的度量算法，发布预览版（40+种以上度量计算，不承诺接口兼容性）
- v1.0, 丰富Github社区的工程化实践，进一步优化实现的内存占用和性能，完善文档与例程，发布正式版（基于semver承诺接口向下兼容）
- v1.x, 修复缺陷并持续维护，视社区活跃度而定，保持平均每2-6个月发布一个新的稳定版（基于semver承诺接口向下兼容）

## Planed

### Engineering Fundementals

- Integrate with CI/CD pipelines
- Official write-ups for devevelop guide
- Switch to public git repo in OSS manner
- Publish crates to public repository
- automated codefmt style (infra not ready)

### New Features

  - hipparchus-mean
    - various mean with customized weights
    - various moving average

  - hipparchus-space, hipparchus-text & hipparchus-stats
    - new metrics
        - kulczynski distance
        - lorentzian distance, intersection & non-intersection distance, refer to https://github.com/drostlab/philentropy
        - wave hedges distance & vicis wave hedges distance, refer to https://github.com/aziele/statistical-distance 
        - moid distance (minimum orbit intersection distance)
        - AMOVA distance, unifrac distance, ladder distance
        - ngd distance (normalized google distance)
        - Nei’s genetic distance, conditional genetic distance
        - wasserstein distance

## More Exploring

- More distances algorithm
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