# Tile Puzzle Wasm


## Description

A [15-puzzle](https://en.wikipedia.org/wiki/15_puzzle) game made in rust with the [Yew framework](https://yew.rs/) as a frontend.
It's possible to use dimensions that aren't 4x4, but I haven't added the images to support that, and the function that checks if a board state is solvable is written for 4x4 boards.

## Installing

* Clone the repo
* Install WebAssembly target with rustup to be able to compile to Wasm
* Install Trunk to run a dev server
```
git clone https://github.com/blink1415/tile_puzzle_wasm
cd tile_puzzle_wasm
rustup target add wasm32-unknown-unknown
cargo install trunk
```

Run the dev server

```
trunk serve
```

You can then find the app at http://localhost:8080/