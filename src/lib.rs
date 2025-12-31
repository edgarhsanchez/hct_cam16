//! `hct-cam16` — CAM16 + HCT perceptual color math.
//!
//! This crate provides a small, dependency-free implementation of:
//! - CAM16 hue/chroma extraction
//! - HCT (Hue–Chroma–Tone) color space
//! - sRGB/Linear RGB/XYZ/L* conversions
//! - An iterative gamut-mapping solver (Material Design 3 style)
//!
//! The public API is designed for deterministic `u32` ARGB input/output.

mod hct;
mod math;
mod palette;
mod scheme;

pub use hct::{Hct, ViewingConditions};
pub use palette::{CorePalette, TonalPalette, STANDARD_TONES};
pub use scheme::MaterialColorScheme;
