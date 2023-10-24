# hipparchus

A Rust lib to implement various algorithm to calaculate distances.

Its folder structure is listed below: 

``` bash
.
├── .cargo                  # alias configuration of cargo
├── .vscode                 # vscode customization on launch & settings json files
├── benches                 # benchmark tests
├── examples                # examples
├── hipparchus              # core engine of hipparchus
│   ├── src                     # source codes
│   │   ├── discrete            # text distance on strings (discrete value based) 
│   │   ├── distribution        # divergence of distributions
│   │   ├── simularity          # simularity on f32/f64 vectors
│   │   ├── space               # space distance on f32/f64 vectors
│   │   ├── sphere              # sphere distance on lat/lon coords
│   │   └── lib.rs              # module root
│   └── Cargo.toml              # Cargo project 
├── tests                   # integratio  tests
└── xtask                   # cargo xtaskops
├── Cargo.toml              # workspace
```

## Enlistment

### Prequisites

Install Rust and VSCode. Run below commands in terminal window:
``` bash
$ cargo install cargo-binutils
$ cargo install grcov
$ rustup component add llvm-tools-preview
$ code --install-extension ryanluker.vscode-coverage-gutters
```

### Build project

To build the project, please run below commands in terminal window:

``` bash
$ cargo build
```

### Run Tests

To execute the unit tests, please run below commands in terminal window:
``` bash
$ cargo test
```

To generate or update test coverage report, please run below commands in terminal window:
``` bash
$ cargo xtask coverage
```

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