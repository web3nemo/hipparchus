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
- Implement 15 distance calculation in 6 categories
    - space distances: canberra, chebysheve, euclidean(rogers), gower, manhattan(czekanowski), mahalanobis
    - sphere distance: haversine, vincenty
    - vector simularity: cosine, dotproduct
    - text distance: hamming, levenshtein
    - statistics distance
        - distribution divergence: cross entropy, KL divergence, JS divergence, hellinger
        - set distance: kumar_hassebrook(PCE), jaccard, sorensen(dice)
        - sampling distance: bray curtis

## Backlog

### Functionality
- LP norm
- kulczynski distance
- lorentzian distance, intersection & non-intersection distance, refer to https://github.com/drostlab/philentropy
- wave hedges distance & vicis wave hedges distance, refer to https://github.com/aziele/statistical-distance 
- moid distance (minimum orbit intersection distance)
- AMOVA distance, unifrac distance, ladder distance
- ngd distance (normalized google distance)
- Nei’s Genetic Distance, Conditional Genetic Distance

motyka
tanimoto
ruzicka
harmonic_mean
fidelity
bhattacharyya
matusita
squared_chord
squared_euclidean
pearson
neyman
squared_chisq
prob_symm
divergence
clark
additive_symm
kullback-leibler
jeffreys
k_divergence
topsoe
jensen_difference
taneja
kumar-johnson
avg
acc
add_chisq
marylandbridge
max_symmetric_chisq
neyman_chisq
pearson_chisq
penroseshape
vicis_symmetric_chisq

### EE Fundementals

- Rewrite the official README markdowns
- Integrate with CI/CD pipelines
- Official write-ups for devevelop guide
- Switch to public git repo in OSS manner
- Publish crates to public repository
