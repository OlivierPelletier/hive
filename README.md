# Hive Game Engine
A game engine for the board game Hive made with Rust.

## Hive
Hive is a bug-themed tabletop game, designed by John Yianni and published in 2001 by Gen42 Games. The object of Hive is to capture the opponent's queen bee by completely surrounding it, while avoiding the capture of one's own queen ([Wikipedia](https://en.wikipedia.org/wiki/Hive_(game))).

## Purpose
The goal of this project is to create a reliable game engine for the game Hive using [Rust Programming Language](https://rust-lang.org). The project focus on stability and compatibility with [Web Assembly](https://webassembly.org/) to provide a way to integrate the Hive Engine into a web application.

## Setup
* Rust 1.45.2 (https://www.rust-lang.org/tools/install)
* Wasm-pack (https://rustwasm.github.io/wasm-pack/installer/)

## Usage
### Run tests
```
cargo test
```

### Run usage example
```
cargo run
```

## Wasm Usage

### Build with `wasm-pack build`

```
wasm-pack build
```

### Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```

### Publish to NPM with `wasm-pack publish`

```
wasm-pack publish
```

## Batteries Included

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
* [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.


## TODO
* ~~*grid.rs* unit tests~~
* *moves.rs* unit tests
* *rules.rs* unit tests
* Create the game controller
* Look for a way to use the engine
* Complete *Usage* section
