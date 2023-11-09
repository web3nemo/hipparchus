# README

The 1st version of codes in geodesic module are forked from [geographiclib-rs](https://github.com/georust/geographiclib-rs/), 
which is a pure Rust implementation on subset of [geographiclib](https://geographiclib.sourceforge.io/)
with direct and the inverse geodesic calculations.

The reasons why we are not choosing any [Rust binding of Karney's C++ implementation](https://github.com/savage13/geographiclib) are:
- Risks in unsafe codes in original C/C++ implementation 
- Code repository is inactive for N years (N>5)
- Crate is not widely use (daily downloads < 100)
- Not interested in other pieces code in geographiclib

The reasons why we are forking [geographiclib-rs](https://github.com/georust/geographiclib-rs/) and make some modification are:
- Leverage Modern Rust for better maintainance
- Be friendly for refactoring with good test coverage as foundation
- Small & deserted community and no much dev effort to improve lib iteratively and quickly
- Passion to explore it freely

Many thanks to [geographiclib-rs](https://github.com/georust/geographiclib-rs/) and 
