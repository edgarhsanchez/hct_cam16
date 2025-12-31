# hct-cam16

[![CI](https://github.com/edgarhsanchez/hct_cam16/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/edgarhsanchez/hct_cam16/actions/workflows/ci.yml)
[![CodeQL](https://github.com/edgarhsanchez/hct_cam16/actions/workflows/codeql.yml/badge.svg?branch=main)](https://github.com/edgarhsanchez/hct_cam16/actions/workflows/codeql.yml)
[![Crates.io](https://img.shields.io/crates/v/hct-cam16.svg)](https://crates.io/crates/hct-cam16)
[![Docs.rs](https://docs.rs/hct-cam16/badge.svg)](https://docs.rs/hct-cam16)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

`hct-cam16` is a small Rust crate implementing CAM16 + HCT perceptual color math with sRGB/XYZ/L* conversions and a gamut-mapping solver. Designed for deterministic ARGB-in/ARGB-out use, with no UI/engine dependencies.

## What are CAM16 and HCT?

**CAM16** is a color appearance model. Unlike simple RGB/HSV/HSL, it tries to model how humans *perceive* color (hue / colorfulness / lightness) under viewing conditions.

**HCT** (Hue–Chroma–Tone) is the Material Design 3 color space built on top of CAM16:
- **Hue**: the “angle” of the color (0–360°) in CAM16.
- **Chroma**: perceived colorfulness/saturation (how vivid it is).
- **Tone**: perceptual lightness (0–100), based on L*.

This crate lets you:
- Convert sRGB colors to/from perceptual HCT.
- Adjust hue/chroma/tone in a way that tracks human perception better than HSL.
- Generate Material-style tonal palettes (consistent hue/chroma across tones).
- Generate a full Material Design 3 color scheme from a seed color.

## Quick start

```rust
use hct_cam16::Hct;

let seed = Hct::from_hex("#6750A4").unwrap();
let color = Hct::new(seed.hue(), seed.chroma(), 50.0);
println!("{:08X}", color.to_argb());
```

## Examples

### 1) Convert a color to HCT and inspect it

```rust
use hct_cam16::Hct;

let hct = Hct::from_argb(0xFF6750A4);
println!("hue={:.1} chroma={:.1} tone={:.1}", hct.hue(), hct.chroma(), hct.tone());
```

### 2) Make a lighter/darker tone (great for contrast ramps)

```rust
use hct_cam16::Hct;

let base = Hct::from_hex("#6750A4").unwrap();

let lighter = base.with_tone(80.0);
let darker = base.with_tone(30.0);

println!("lighter={:08X} darker={:08X}", lighter.to_argb(), darker.to_argb());
```

### 3) Rotate hue while keeping tone (useful for accents)

```rust
use hct_cam16::Hct;

let base = Hct::from_argb(0xFF6750A4);
let rotated = base.with_hue((base.hue() + 60.0) % 360.0);

println!("{:08X}", rotated.to_argb());
```

### 4) Generate a tonal palette (Material “tones”)

```rust
use hct_cam16::{Hct, TonalPalette};

let seed = Hct::from_hex("#6750A4").unwrap();
let mut palette = TonalPalette::from_hct(&seed);

// Common tones: 0..100. Material uses a standard set like 40, 90, 100, etc.
let t40 = palette.tone(40);
let t90 = palette.tone(90);

println!("t40={:08X} t90={:08X}", t40, t90);
```

### 5) Generate a full Material Design 3 color scheme

```rust
use hct_cam16::MaterialColorScheme;

let scheme = MaterialColorScheme::from_seed(0xFF6750A4);

// All fields are ARGB (u32)
println!("primary={:08X} on_primary={:08X}", scheme.primary, scheme.on_primary);
```

## Docs

- HCT solver overview: docs/HCT_SOLVER.md
