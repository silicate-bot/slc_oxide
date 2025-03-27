# slc replay format

A tiny and incredibly fast replay format for Geometry Dash.

Requires Rust nightly, at least version `1.87.0`.

## Documentation

For documentation, please refer to the [original slc repo](https://github.com/silicate-bot/slc).

## Example

```rust
struct ReplayMeta {
  pub seed: u64
}

let mut replay = Replay::<ReplayMeta>::new(
  240.0, 
  ReplayMeta { 
    seed: 1234 
  }
);

// OR

let mut replay = Replay::<()>::new(240.0, ()); // For no meta

// Set tps by directly changing the value
replay.tps = 480.0;

// Add inputs using the `add_input` function
replay.add_input(200, InputData::Player(PlayerData {
  button: 1,
  hold: true,
  player_2: false
}));

// Other input types
replay.add_input(400, InputData::Death);
replay.add_input(600, InputData::TPS(480.0));

// Save the replay
let file = File::open("replay.slc")?;
let bw = BufWriter::new(file); // RECOMMENDED!
replay.write(bw)?;
```
