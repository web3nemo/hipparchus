# hipparchus

A Rust lib to implement various algorithm to calaculate distances.

Its folder structure is listed below: 

``` bash
.
├── .cargo                  # alias configuration of cargo
├── .vscode                 # vscode customization on launch & settings json files
├── crates
│   ├── hipparchus-mean     # LpNorm, mean & moving average
│   ├── hipparchus-space    # point, vector & geo distance
│   ├── hipparchus-stats    # distribution, sampling & set distance
│   └── hipparchus-text     # distribution, sampling & set distance
├── xtask                   # cargo xtaskops
└── Cargo.toml              # workspace
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

See in [CHANGELOG](./changelog.md).

## Roadmap

See in [BACKLOG](./backlog.md).

