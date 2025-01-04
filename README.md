# Labyrinth Bevy Game

## Launch
`cargo run`

## Web Launch
`cargo run --example web`
Open under [http://localhost:8080](http://localhost:8080)

## Gameplay
- Use arrow keys to move.
- Press x key to attack.
- Avoid enemies.
- Try to find the green finish area.
- Collect coins to increase score.

## Build for web
[Unofficial docs](https://bevy-cheatbook.github.io/platforms/wasm/webpage.html)
```
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --no-typescript --target web \
    --out-dir ./wasm_out/ \
    --out-name "game" \
    ./target/wasm32-unknown-unknown/release/game.wasm
```

