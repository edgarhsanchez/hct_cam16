//! Tonal Palette for perceptual color roles.

use crate::hct::Hct;
use std::collections::HashMap;

/// Standard tones used in Material Design 3
pub const STANDARD_TONES: &[u8] = &[
    0, 4, 6, 10, 12, 17, 20, 22, 24, 30, 40, 50, 60, 70, 80, 87, 90, 92, 94, 95, 96, 98, 99, 100,
];

/// A tonal palette generates colors at different tones (lightness levels)
/// while maintaining the same hue and chroma.
#[derive(Debug, Clone)]
pub struct TonalPalette {
    /// Base hue angle (0-360)
    hue: f64,
    /// Base chroma value
    chroma: f64,
    /// Cache of generated colors
    cache: HashMap<u8, u32>,
}

impl TonalPalette {
    /// Create a new tonal palette from hue and chroma
    pub fn new(hue: f64, chroma: f64) -> Self {
        Self {
            hue,
            chroma,
            cache: HashMap::new(),
        }
    }

    /// Create a tonal palette from an HCT color
    pub fn from_hct(hct: &Hct) -> Self {
        Self::new(hct.hue(), hct.chroma())
    }

    /// Create a tonal palette from an ARGB color
    pub fn from_argb(argb: u32) -> Self {
        Self::from_hct(&Hct::from_argb(argb))
    }

    /// Hue angle
    pub fn hue(&self) -> f64 {
        self.hue
    }

    /// Chroma
    pub fn chroma(&self) -> f64 {
        self.chroma
    }

    /// Get a color at the specified tone (0-100)
    pub fn tone(&mut self, tone: u8) -> u32 {
        let tone = tone.min(100);

        if let Some(&cached) = self.cache.get(&tone) {
            return cached;
        }

        let hct = Hct::new(self.hue, self.chroma, tone as f64);
        let argb = hct.to_argb();
        self.cache.insert(tone, argb);
        argb
    }

    /// Get an HCT color at the specified tone
    pub fn tone_hct(&self, tone: u8) -> Hct {
        Hct::new(self.hue, self.chroma, tone.min(100) as f64)
    }

    /// Pre-cache all standard tones
    pub fn cache_standard_tones(&mut self) {
        for &tone in STANDARD_TONES {
            self.tone(tone);
        }
    }
}

/// Core tonal palettes for a Material Design 3 style color scheme
#[derive(Debug, Clone)]
pub struct CorePalette {
    pub primary: TonalPalette,
    pub secondary: TonalPalette,
    pub tertiary: TonalPalette,
    pub neutral: TonalPalette,
    pub neutral_variant: TonalPalette,
    pub error: TonalPalette,
}

impl CorePalette {
    pub fn from_argb(seed: u32) -> Self {
        let hct = Hct::from_argb(seed);
        Self::from_hct(&hct)
    }

    pub fn from_hct(seed: &Hct) -> Self {
        let hue = seed.hue();
        let chroma = seed.chroma();

        Self {
            primary: TonalPalette::new(hue, chroma.max(48.0)),
            secondary: TonalPalette::new(hue, 16.0),
            tertiary: TonalPalette::new((hue + 60.0) % 360.0, 24.0),
            neutral: TonalPalette::new(hue, 4.0),
            neutral_variant: TonalPalette::new(hue, 8.0),
            error: TonalPalette::new(25.0, 84.0),
        }
    }

    pub fn cache_all(&mut self) {
        self.primary.cache_standard_tones();
        self.secondary.cache_standard_tones();
        self.tertiary.cache_standard_tones();
        self.neutral.cache_standard_tones();
        self.neutral_variant.cache_standard_tones();
        self.error.cache_standard_tones();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tonal_palette_creation() {
        let palette = TonalPalette::new(270.0, 50.0);
        assert!((palette.hue() - 270.0).abs() < 0.001);
        assert!((palette.chroma() - 50.0).abs() < 0.001);
    }

    #[test]
    fn test_tonal_palette_tones() {
        let mut palette = TonalPalette::new(270.0, 50.0);

        let tone_0 = palette.tone(0);
        let r = (tone_0 >> 16) & 0xFF;
        let g = (tone_0 >> 8) & 0xFF;
        let b = tone_0 & 0xFF;
        assert!(r < 20 && g < 20 && b < 20);

        let tone_100 = palette.tone(100);
        let r = (tone_100 >> 16) & 0xFF;
        let g = (tone_100 >> 8) & 0xFF;
        let b = tone_100 & 0xFF;
        assert!(r > 240 && g > 240 && b > 240);
    }

    #[test]
    fn test_core_palette() {
        let palette = CorePalette::from_argb(0xFF6750A4);

        assert!((palette.primary.chroma() - 48.0).abs() < 0.001);
        assert!((palette.secondary.chroma() - 16.0).abs() < 0.001);
        assert!(palette.neutral.chroma() <= 8.0);
        assert!(palette.error.hue() < 50.0 || palette.error.hue() > 330.0);
    }

    #[test]
    fn test_palette_caching() {
        let mut palette = TonalPalette::new(200.0, 40.0);
        let first = palette.tone(50);
        let second = palette.tone(50);
        assert_eq!(first, second);
    }
}
