# minesweeper

This is a minesweeper game developed in Rust using Bevy, a free and open source data-driven game engine. The development is based on [Bevy Minesweeper: Introduction - DEV Community](https://dev.to/qongzi/bevy-minesweeper-introduction-4l7f) but since the engine is very young and hence under very rapid iterations a lot of APIs have changed since the tutorial (it uses Bevy 0.8 while this repo is with Bevy 0.13). Some additional designs were also added in this repo.

## Prerequisites

Ensure you have Rust installed on your system. If not, you can download and install it from the official Rust website: [https://www.rust-lang.org/](https://www.rust-lang.org/)

## Usage
```bash
git clone https://github.com/WanruXX/minesweeper.git
cd minesweeper
cargo run
```

There is also a web version of the game compiled by Bevy WebAssembly (WASM). To play the game in the browser:
```bash
rustup target install wasm32-unknown-unknown
cargo install wasm-server-runner
cd .carge
cargo serve_release
```

## Demo
https://github.com/WanruXX/minesweeper/assets/48364194/4976de5c-7ad1-4c72-b627-32a7ddf18d97