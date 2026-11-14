# h3thievesguild

Statistics for Heroes of Might and Magic III: Horn of the Abyss games, read from save files.

This repository is the Rust successor to [homm3tools](https://github.com/oysteinmyrmo/homm3tools).
The plan is a Rust save parser compiled to WebAssembly, with a web frontend that loads a folder of
saves and charts each player's progress over the game.

## Status

The save parser, ported from the homm3tools C++ code with its full test suite, and the data set
that turns a game's saves into series for plotting.

- Supports **HotA 1.8** saves only.
- Finds the player, town and hero records by the shape of their data rather than by searching
  for hero and town names, so maps with renamed heroes and towns need no extra input.
- Reads the in-game date (month, week and day).
- Reads each player's resources, heroes on the map and towns.
- Builds a data set from the saves of one game: per-day series of each player's resources, army
  strength, heroes and towns, and each hero's level, experience and primary skills. Saves from
  different games, or two saves from the same day, are reported as errors naming the files.
- Creature, artifact and town name tables do not yet include the Factory and Bulwark additions.
  Unknown IDs are kept as raw values (for example `Creature(190)`); unknown creatures count as 0
  towards army strength.

## Layout

| Path | What it is |
| --- | --- |
| `crates/h3save` | The save parser library. |
| `crates/h3save/tests` | Integration tests: one file per homm3tools C++ test file, plus tests added since. |
| `crates/h3dataset` | Builds the per-day data set for plotting from a game's saves. |
| `crates/h3wasm` | WebAssembly bindings for the browser, with TypeScript types for the data set. |
| `test_data` | HotA 1.8 test saves and maps, copied unchanged from homm3tools. Each folder's `info.txt` describes what was done in game. |

## Tests

```sh
cargo test
```

The 68 save tests started as a one-to-one port of the homm3tools Catch2 tests, with the same expected
values. Each runs every check of its C++ original (16,125 in total) plus checks added since, such as
the save's date. To see the count per test:

```sh
cargo test -- --nocapture --test-threads=1
```

### WebAssembly

The `h3wasm` tests run in WebAssembly under Node.js. They need the `wasm-bindgen` command-line
tool at the same version as the `wasm-bindgen` crate in `Cargo.lock`:

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli --version <version from Cargo.lock>
CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER=wasm-bindgen-test-runner \
    cargo test -p h3wasm --target wasm32-unknown-unknown
```

To build the module and its JavaScript bindings for a web page:

```sh
cargo build --release -p h3wasm --target wasm32-unknown-unknown
wasm-bindgen --target web --out-dir pkg target/wasm32-unknown-unknown/release/h3wasm.wasm
```

In the page:

```js
import init, { DatasetBuilder } from "./pkg/h3wasm.js";

await init();
const builder = new DatasetBuilder();
for (const file of input.files) {
    builder.add(file.name, new Uint8Array(await file.arrayBuffer()));
}
const dataset = builder.finish(); // Throws a DatasetError naming the files at fault.
```

### Robustness

Save files come from users, so the parser must not panic on any input. The library denies the
Clippy lints for indexing, slicing, unwrapping and unchecked arithmetic, and
`crates/h3save/tests/robustness.rs` feeds it truncated, damaged and random saves.

## Usage

```rust,no_run
use h3save::{SaveFile, player::players};

let save = SaveFile::read("[hotseat] 111.GM2")?;
println!("{}", save.date); // Month 1, Week 1, Day 1
for player in players(&save).iter().filter(|p| p.alive()) {
    println!("{:?}: {} gold, army strength {}", player.color(), player.gold(), player.kingdom_army_strength());
}
# Ok::<(), h3save::Error>(())
```

