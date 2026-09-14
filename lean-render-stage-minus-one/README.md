# Stage −1: the smallest serious renderer experiment

This repository is a deliberately tiny precursor to a browser-engine project scaffolded in Lean 4.
It asks a narrower question first:

> Can a small but semantically coupled rendering unit be specified executably in Lean, independently implemented in Rust, and mechanically compared in a harness that demonstrably detects realistic faults?

It is **not HTML or CSS**. The language is currently represented by two fixtures in both implementations so that the experiment measures layout semantics before spending effort on parsing syntax.

## Semantic surface

Nodes are boxes or text. Boxes have optional width/height, padding, foreground and background colours. Text uses a fixed-width font model. Layout is block flow only.

The deliberately important dependency is:

```
parent width
  -> inner width
  -> text wrapping
  -> text height
  -> following sibling Y position
  -> parent natural height
```

That is enough coupling to expose integration mistakes without importing browser complexity.

## Repository structure

- `StageMinusOne.lean` — executable Lean semantics.
- `Main.lean` — Lean command-line renderer for named fixtures.
- `rust/` — independent Rust implementation.
- `scripts/differential.py` — exact display-list comparison plus deliberate fault injection.
- `.github/workflows/ci.yml` — builds both implementations and runs the experiment when this project is moved to its own repository.

## Observation

The renderer stops at a deterministic display list:

```
rect 0 0 120 88 red
text 10 10 black |hello world|
text 10 26 black |hello world|
rect 10 42 80 20 blue
text 10 62 black |tail|
```

Rasterisation is intentionally outside the experiment.

## Deliberate faults

The Rust implementation can inject four faults through `STAGE_MINUS_ONE_FAULT`:

- `drop_text`
- `off_by_one_width`
- `wrong_sibling_y`
- `wrong_paint_order`

CI requires the ordinary Rust implementation to match Lean and requires every deliberate fault to be detected by at least one fixture. This is a minimal harness-sensitivity test: agreement alone is not sufficient evidence.

## Build

Requires Lean 4/Lake, Rust/Cargo and Python 3.

```sh
lake build
(cd rust && cargo test)
python3 scripts/differential.py
```

## What this does not claim

- No web-platform compatibility.
- No proof that the Rust implementation refines the Lean model yet.
- No generated interface yet.
- No parser yet.
- No claim that Lean is economically superior to conventional contracts.

Those are subsequent experimental steps, not assumptions.

## Next evidence gates

1. Make the fixture/document representation single-source rather than duplicated.
2. Add exhaustive bounded document generation and compare both implementations.
3. Add a conventional Rust-contract control implementation for matched-task cost comparison.
4. Add a generated Rust semantic interface from the Lean-side schema.
5. Prove a small functional-refinement theorem for one pure kernel (for example wrapping or block placement).
6. Only then add a tiny source parser and CSS-like selector/cascade layer.
