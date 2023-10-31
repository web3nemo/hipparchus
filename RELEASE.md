# Release v0.2 (in progress)

## What's New?

TODO

## What's Changed?

N/A

## Bug fixes

N/A

# Release v0.1 (locked)

## What's New?

- 27 new OPs in hipparchus-mean (stabled)
    - 4 mean: arithmetic mean, geometric mean, harmonic mean, quadratic mean
    - 4 moving average: sma, cma, wma, ema
    - 5 norm: L0/L1/L2/Lp/Lp(Inf) norm
    - 14 sequences with 3 derivations (recursive/map/fold)
        - arithmetic(natural/odd/even), geometric
        - triangular, square, cubic, harmonic
        - fibonacci, lucas, padova
        - catalan, look and say

- 20 new OPs in hipparchus-metrics (alpha, in development)
    - points: canberra, chebysheve, euclidean, manhattan, mahalanobis, gower
    - vectors: cosine, dotproduct
    - geo/sphere distance (for lat/lon): haversine, vincenty
    - distribution (divergence or distance): cross entropy, KL divergence, JS divergence, hellinger
    - sampling: bray curtis
    - set: jaccard, sorensen(dice), kumar_hassebrook(PCE)
    - text distance: hamming, levenshtein

- engineering fundementals for project framing
    - workspace (reexports), project/package, folders, modules
    - unit tests & test coverage tools
    - CI pipeline (Github action), status badges
    - cargo docs (basic), package metadata, license & readme files

## What's Changed?

N/A

## Bug fixes

N/A
