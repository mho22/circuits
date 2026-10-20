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

- Pick a part in the toolbar, or press its key: `W` wire, `S` switch, `R` resistor, `C` capacitor, `L` inductor, `D` diode, `N` NPN transistor, `P` PNP transistor, `M` NMOS transistor, `F` PMOS transistor, `B` battery, `V` VCC, `G` ground. The toolbar groups the parts by role: wiring, passive parts, semiconductors and sources. The arrow next to a group lists its parts.
- Drag to draw a part. A part runs at 0° or 90°. Click to place a VCC or a ground.
- Double-click a resistor, a battery, a VCC, a capacitor or an inductor to change its value. Write a number in the unit at the end of the input: `Ω`, `V`, `µF` or `mH`. A comma or a dot separates the decimals: `4700` Ω, `4,7` µF.
- Double-click a switch to open or close it.
- Right-click a wire to delete it.
- Right-click any other part to open its menu: `Move`, `Rotate`, `Mirror horizontally`, `Mirror vertically` and `Delete`. A ground and a VCC have only `Move` and `Delete`. After `Move`, the part follows the cursor: click to drop it, or press `Escape` to cancel. `Rotate` turns the part a quarter clockwise. A mirrored transistor swaps the sides of its collector and its emitter, or of its drain and its source.
- Press `Ctrl Z` (`⌘ Z` on a Mac) to undo, and `Ctrl Shift Z` or `Ctrl Y` (`⌘ Shift Z`) to redo.
- Hover a part to read its voltage and current.
- Move the view with a two-finger swipe or the mouse wheel, by dragging with the middle button, or by holding `Space` and dragging.
- Click the target button in the bottom right to bring the view back to the start.
- Click `Save` or press `Ctrl S` (`⌘ S`) to save the drawing as a `.circuit` file. Chrome and Edge ask for the name and the folder. The other browsers download the file. Click `Open` or press `Ctrl O` (`⌘ O`) to replace the drawing with a file. Undo brings the previous drawing back.

Two parts connect where their ends meet, or where an end touches a wire. Two wires that cross do not connect. Every ground is the same 0 V node. A VCC holds its point at its voltage above that node, 5 V by default.

A part drawn, moved or turned along a wire replaces the wire under it. The wire keeps its pieces on each side, and they connect to the ends of the part.

A `.circuit` file is JSON. It lists each part with its kind, its value, and the grid points of its two ends:

```json
{
	"version" : 1,
	"parts" : [
		{ "kind" : "resistor", "ohms" : 1000, "from" : [ 0, 0 ], "to" : [ 4, 0 ] },
		{ "kind" : "switch", "closed" : true, "from" : [ 4, 0 ], "to" : [ 4, 2 ] }
	]
}
```

The kinds are `wire`, `ground`, `switch`, `resistor`, `capacitor`, `inductor`, `battery`, `diode`, `npn`, `pnp`, `nmos`, `pmos` and `vcc`. The values are `ohms`, `farads`, `henries` and `volts`, in base units. A mirrored transistor has `"mirrored" : true`.

A diode lets the current pass from the start of the drag to its end, where the bar is drawn. It blocks the current the other way. A diode that conducts drops about 0.7 V.

A transistor is drawn from its base to its bar. The collector sits one step past the bar on the left of the drag, and the emitter on the right. Its gain is 100: the collector current is 100 times the base current until the transistor saturates.

A MOSFET is drawn from its gate to its channel, like a bipolar transistor. The drain sits on the left of the drag, and the source on the right. The gate takes no current. An NMOS conducts when its gate is more than 1.5 V above its source. Its drain current then follows the square law: 10 mA/V² times the square of the voltage above 1.5 V. A PMOS does the same with every voltage reversed.

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
