# Circuits

Build electronic circuits from scratch, in the browser. Start with a wire. Add physics.

<br>

## Run

```bash
npm install
npm run dev
```

Requires Rust through `rustup`. `rust-toolchain.toml` pins the version and the `wasm32-unknown-unknown` target.

<br>

## Use

- Drag to draw a wire. A wire runs at 0°, 45° or 90°.
- Right-click a wire to delete it.

<br>

## Structure

- `engines/simulator` — the circuit document and its operations, in pure Rust with no OS calls.
- `bindings/web` — the `wasm-bindgen` wrapper the browser loads.
- `src` — the canvas UI.

<br>

## Test

```bash
npm test
```
