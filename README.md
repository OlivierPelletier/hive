# Hive Game Engine

A game engine for the board game Hive made with Rust.

## Hive

Hive is a bug-themed tabletop game, designed by John Yianni and published in 2001 by Gen42 Games.
The object of Hive is to capture the opponent's queen bee by completely surrounding it, while
avoiding the capture of one's own queen ([Wikipedia](https://en.wikipedia.org/wiki/Hive_(game))).

## Purpose

The goal of this project is to create a reliable game engine for the game Hive
using [Rust Programming Language](https://rust-lang.org).

## Setup

* Rust 1.82.0 (https://www.rust-lang.org/tools/install)

## Usage

### As a library

Add hive-engine to your dependencies:

```
hive-engine = "0"
```

And then get started in your `main.rs`:

```rust
// Initialize game
let mut game: Game = Game::tournament();

//List actions for a player
let actions: Vec<Action> = game.list_actions_for_player( & game.players[game.current_player_index]);

// Play an action
let chosen_action: Action = actions[0];
game.play_action(chosen_action);

// Check if there's a winner
let winner: Option<PieceColor> = game.winner();
```

### As a command line prototype

```
cargo run
```

```
...
   __      __      __      __      __      __      __      __      __      __
  -2,-6   -1,-6   0,-6    1,-6    2,-6    3,-6    4,-6    5,-6    6,-6    7,-6
       __      __      __      __      __      __    W BETL    __    W SANT    __
      -2,-5   -1,-5   0,-5    1,-5    2,-5    3,-5    4,-5    5,-5    6,-5    7,-5
           __      __      __      __      __    W SANT  W GRHP  W GRHP    __      __
          -2,-4   -1,-4   0,-4    1,-4    2,-4    3,-4    4,-4    5,-4    6,-4    7,-4
               __      __      __      __      __    W SPDR    __      __      __      __
              -2,-3   -1,-3   0,-3    1,-3    2,-3    3,-3    4,-3    5,-3    6,-3    7,-3
                   __      __      __      __      __    B QBEE    __      __      __      __
                  -2,-2   -1,-2   0,-2    1,-2    2,-2    3,-2    4,-2    5,-2    6,-2    7,-2
                       __      __    B GRHP  B SPDR  B SPDR    __      __      __      __      __
                      -2,-1   -1,-1   0,-1    1,-1    2,-1    3,-1    4,-1    5,-1    6,-1    7,-1
                           __      __      __      __    W QBEE    __      __      __      __      __
                          -2,0    -1,0     0,0     1,0     2,0     3,0     4,0     5,0     6,0     7,0
                               __      __      __      __    W SPDR  B SANT    __      __      __      __
                              -2,1    -1,1     0,1     1,1     2,1     3,1     4,1     5,1     6,1     7,1
                                   __      __      __    W BETL    __      __      __      __      __      __
                                  -2,2    -1,2     0,2     1,2     2,2     3,2     4,2     5,2     6,2     7,2

[(0): B GRHP from:   0,-1  , to:   3,-1  ][(1): B SANT from:    3,1  , to:    2,2  ][(2): B SANT from:    3,1  , to:    1,3  ]...

Choose an action:

...

B wins!
```

## Development

### Run tests

```
cargo test
```

## TODO

* Add Pillbug piece type
