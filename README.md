# h3thievesguild

Statistics for Heroes of Might and Magic III: Horn of the Abyss games, read from save files.

This repository is the Rust successor to [homm3tools](https://github.com/oysteinmyrmo/homm3tools).
The plan is a Rust save parser compiled to WebAssembly, with a web frontend that loads a folder of
saves and charts each player's progress over the game.

## Status

First iteration: the save parser, ported from the homm3tools C++ code, with its full test suite.

- Supports **HotA 1.8** saves only.
- Finds heroes and towns the same way the C++ code does: by searching for the first hero's name
  (Orrin) and for town names. Replacing this with a sequential parse is the next step.
- Creature, artifact and town name tables do not yet include the Factory and Bulwark additions.
  Unknown IDs are kept as raw values (for example `Creature(190)`); unknown creatures count as 0
  towards army strength.

## Layout

| Path | What it is |
| --- | --- |
| `crates/h3save` | The save parser library. |
| `crates/h3save/tests` | Integration tests, one file per homm3tools C++ test file. |
| `test_data` | HotA 1.8 test saves and maps, copied unchanged from homm3tools. Each folder's `info.txt` describes what was done in game. |

## Tests

```sh
cargo test
```

The 68 integration tests are a one-to-one port of the homm3tools Catch2 tests, with the same expected
values. Each test runs exactly as many checks as its C++ original, 16,125 in total. To see the count
per test:

```sh
cargo test -- --nocapture --test-threads=1
```

## Usage

```rust
use h3save::{Input, SaveFile, player::players};

let save = SaveFile::read(&Input::new("[hotseat] 111.GM2"))?;
for player in players(&save).iter().filter(|p| p.alive()) {
    println!("{:?}: {} gold, army strength {}", player.color(), player.gold(), player.kingdom_army_strength());
}
# Ok::<(), h3save::Error>(())
```

When a map renames towns, give red player's start town and the number of towns:

```rust,no_run
# use h3save::{Input, SaveFile};
let save = SaveFile::read(&Input::with_towns("[hotseat] 111.GM2", "RedsTown", 4))?;
# Ok::<(), h3save::Error>(())
```
