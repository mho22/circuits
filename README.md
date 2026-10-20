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

- Pick a part in the toolbar, or press its key: `W` wire, `G` ground, `S` switch, `R` resistor, `C` capacitor, `L` inductor, `B` battery. The toolbar groups the parts by role: wiring, passive parts and sources. The arrow next to a group lists its parts.
- Drag to draw a part. A part runs at 0°, 45° or 90°. Click to place a ground.
- Double-click a resistor, a battery, a capacitor or an inductor to change its value. Write `4.7k`, `220`, `9 V`, `10u`.
- Double-click a switch to open or close it.
- Right-click a part to delete it.
- Press `Ctrl Z` (`⌘ Z` on a Mac) to undo, and `Ctrl Shift Z` or `Ctrl Y` (`⌘ Shift Z`) to redo.
- Hover a part to read its voltage and current.

Two parts connect where their ends meet, or where an end touches a wire. Two wires that cross do not connect. Every ground is the same 0 V node.

A part drawn along a wire replaces the wire under it. The wire keeps its pieces on each side, and they connect to the ends of the part.

The color of a wire shows its voltage: green is positive, red is negative, gray is 0 V. The yellow dots show the current.

Time runs at 10 ms of circuit time per second. A capacitor starts empty and an inductor starts without current. Each time step is 10 µs.

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
