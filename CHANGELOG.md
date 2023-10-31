# History

## v0.2 Release Note

### Engineering Fundementals

- workspace
    - enable workspace Cargo.toml with multiple crates.
    - folder structure alignment & refinement
    - reorg README.md with new isolated CHANGELOG.md, BACKLOG.md
    - add reexports in lib.rs

- tests
    - enable code coverage
        - instumentation, xtask + llvm-cov + grcov
        - reporting, coverage gutters in vscode
    - Add more unit tests
        - to improve code coverage to above 90%

### What's New?

- new OPs in hipparchus-mean
    - 4 mean: arithmetic mean, geometric mean, harmonic mean, quadratic mean
    - 4 moving average: sma, cma, wma, ema
    - 5 norm: L0/L1/L2/Lp/Lp(Inf) norm
    - 14 sequences
        - arithmetic(natural/odd/even), geometric
        - triangular, square, cubic, harmonic
        - fibonacci, lucas, padova
        - catalan
        - look and say

- new OPs in hipparchus-metrics
    - 1 point: gower distance
    - 1 distribution: hellinger distance
    - 1 sampling: bray curtis distance
    - 3 set: kumar_hassebrook(PCE), jaccard, sorensen(dice)

### What's Changed?

- deprecate mahalanobis distance (crate not ready)

### Bug fixes
N/A

## v0.1 Release Note

### Engineering Fundementals

Initialize project framing
- project, packages, folders, modules
- unit tests

### What's New?

Implement 14 distance calculation (basic) in 5 categories
- Minkowski-family distances: canberra, chebysheve, euclidean, manhattan, mahalanobis
- Simularity: cosine, dotproduct
- Distribution divergence or distance: cross entropy, KL divergence, JS divergence
- Sphere distance (between lat/lon coords): haversine, vincenty
- Text distance: hamming, levenshtein

### What's Changed?
N/A

### Bug fixes
N/A
