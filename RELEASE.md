# Release v0.1.2

## What's New?

### hipparchus
- new entry crate to re-exports core modules in all sub hipparchus-* crates

### hipparchus-az
- Mod: Euclidean, Symmetry & Inverted Symmetry
- Degree & Radians: Norm
- DMS: Angle Conversion
- Azimuth: Angle Conversion, Operations, Norm

### hipparchus-geo
- Coord & LatLon
    - Latitude & Longitude with norm
    - Extract orientation, elimate zone, time zone & hemisphere
    - Combined LatLon and fmt with NMEA0183/ISO6709 standard
- Ellipsoid
    - 9 worldwide models, 13 regional models & 4 sphere models
    - 3 flattening, 5 eccentricity & 4 eccentricity square 
    - 6 radius, volume & surface area
- Geodesic
    - haversine
    - vencenty (may deprecate in future)
    - graphicslib migration (experimental, to be continued)

### hipparchus-mean
- Bench

### hipparchus-metrics
- Minor Refactors

### hipparchus-seq
- Minor Refactors
- Bench

## What's Changed?

N/A

## (Critical) Bug fixes

N/A

# Release v0.1

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
    - point: Canberra, Chebysheve, Euclidean, Manhattan, Gower
    - vector: Dot Product, Cosine
    - distribution (divergence or distance): Cross Entropy, KL divergence, JS divergence, Hellinger
    - sampling: Bray Curtis, Sorensen Dice, Jaccard
    - lat/lon: Haversine, vincenty
    - text: Hamming, Levenshtein

- engineering fundementals for project framing
    - workspace (reexports), project/package, folders, modules
    - unit tests & test coverage tools
    - CI pipeline (Github action), status badges
    - cargo docs (basic), package metadata, license & readme files

## What's Changed?

N/A

## Bug fixes

N/A
