# hipparchus

A Rust lib to implement various algorithm to calaculate distances.

Its folder structure is listed below: 

``` bash
.
├── Cargo.toml              # Rust
├── src                     # source code
│   ├── discrete            # text distance on strings (discrete value based) 
│   ├── distribution        # divergence of distributions
│   ├── simularity          # simularity on f32/f64 vectors
│   ├── space               # space distance on f32/f64 vectors
│   ├── sphere              # sphere distance on lat/lon coords
│   └── lib.rs              # 
├── tests                   # integratio  tests
├── benches                 # benchmark tests
└── examples                # examples
```

## Enlistment

TODO

## What's New

### v0.1
- Setup project framing: prject, packages, folders, modules, unit tests
- Implement 14 distance calculation (basic) in 5 categories
    - Minkowski-family distances: canberra, chebysheve, euclidean, manhattan, mahalanobis
    - Simularity: cosine, dotproduct
    - Distribution divergence or distance: cross entropy, KL divergence, JS divergence
    - Sphere distance (between lat/lon coords): haversine, vincenty
    - Text distance: hamming, levenshtein

## Backlog

TODO