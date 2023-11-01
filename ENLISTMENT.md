# Enlistment

> [!NOTE]
> The detailed development guide is still under construction.

The folder structure of 'hipparchus' project is listed below: 

``` bash
.
├── .cargo                  # alias configuration of cargo
├── .github                 # github workflow
├── .vscode                 # vscode customization on launch & settings json files
├── doc                     # dev internal doc
│   └── .archive            # backup legacy codes as references
├── crates
│   ├── hipparchus-mean     # various mathematics operations: lp-norm, mean & moving average
│   └── hipparchus-metrics  # various distance metrics
├── xtask                   # cargo xtaskops 
└── Cargo.toml              # workspace
```

## Prequisites

Install Rust and VSCode on you dev machine. 

Run below commands to setup further tools in terminal window:
``` bash
$ cargo install cargo-binutils
$ cargo install grcov
$ rustup component add llvm-tools-preview
$ code --install-extension ryanluker.vscode-coverage-gutters
```

## Build

To build the project, please run below commands in terminal window:

``` bash
$ cargo build
```

## Run Tests

To execute the unit tests, please run below commands in terminal window:
``` bash
$ cargo test
```

To generate or update test coverage report, please run below commands in terminal window:
``` bash
$ cargo xtask coverage --dev
```

