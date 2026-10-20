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

- Pick a part in the toolbar, or press its key: `W` wire, `R` resistor, `B` battery, `G` ground.
- Drag to draw a wire, a resistor or a battery. A part runs at 0°, 45° or 90°. Click to place a ground.
- Double-click a resistor or a battery to change its value. Write `4.7k`, `220`, `9 V`.
- Right-click a part to delete it.
- Hover a part to read its voltage and current.

Two parts connect where their ends meet, or where an end touches a wire. Two wires that cross do not connect. Every ground is the same 0 V node.

The color of a wire shows its voltage: green is positive, red is negative, gray is 0 V. The yellow dots show the current.

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
