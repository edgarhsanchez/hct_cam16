# hct-cam16

[![CI](https://github.com/edgarhsanchez/hct_cam16/actions/workflows/ci.yml/badge.svg)](https://github.com/edgarhsanchez/hct_cam16/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/hct-cam16.svg)](https://crates.io/crates/hct-cam16)
[![Docs.rs](https://docs.rs/hct-cam16/badge.svg)](https://docs.rs/hct-cam16)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

`hct-cam16` is a small Rust crate implementing CAM16 + HCT perceptual color math with sRGB/XYZ/L* conversions and a gamut-mapping solver. Designed for deterministic ARGB-in/ARGB-out use, with no UI/engine dependencies.

## Quick start

```rust
use hct_cam16::Hct;

let seed = Hct::from_hex("#6750A4").unwrap();
let color = Hct::new(seed.hue(), seed.chroma(), 50.0);
println!("{:08X}", color.to_argb());
```

## Docs

- HCT solver overview: docs/HCT_SOLVER.md
