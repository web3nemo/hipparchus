# hipparchus

Rust crates to implement various mathmatics calculation.

The folder structure is listed below: 

``` bash
.
├── .archive                # backup legacy codes as references
├── .cargo                  # alias configuration of cargo
├── .vscode                 # vscode customization on launch & settings json files
├── crates
│   ├── hipparchus-mean     # various mathematics operations: lp-norm, mean & moving average
│   └── hipparchus-metrics  # various distance metrics
├── xtask                   # cargo xtaskops
└── Cargo.toml              # workspace
```

The detailed development guide is still under construction.

## Enlistment

### Prequisites

Install Rust and VSCode on you dev machine. 

Run below commands to setup further tools in terminal window:
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
$ cargo xtask coverage --dev
```

## Change History

See details in [CHANGELOG](./CHANGELOG.md).
