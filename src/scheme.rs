//! Material Design 3-ish Color Scheme Generation (ARGB output)

use crate::palette::CorePalette;

/// A complete color scheme (ARGB values).
#[derive(Debug, Clone)]
pub struct MaterialColorScheme {
    // Primary
    pub primary: u32,
    pub on_primary: u32,
    pub primary_container: u32,
    pub on_primary_container: u32,

    // Secondary
    pub secondary: u32,
    pub on_secondary: u32,
    pub secondary_container: u32,
    pub on_secondary_container: u32,

    // Tertiary
    pub tertiary: u32,
    pub on_tertiary: u32,
    pub tertiary_container: u32,
    pub on_tertiary_container: u32,

    // Error
    pub error: u32,
    pub on_error: u32,
    pub error_container: u32,
    pub on_error_container: u32,

    // Surface
    pub surface: u32,
    pub surface_bright: u32,
    pub surface_dim: u32,
    pub on_surface: u32,
    pub on_surface_variant: u32,

    // Surface Containers
    pub surface_container_lowest: u32,
    pub surface_container_low: u32,
    pub surface_container: u32,
    pub surface_container_high: u32,
    pub surface_container_highest: u32,

    // Outline
    pub outline: u32,
    pub outline_variant: u32,

    // Inverse
    pub inverse_surface: u32,
    pub inverse_on_surface: u32,
    pub inverse_primary: u32,

    // Fixed Accent
    pub primary_fixed: u32,
    pub primary_fixed_dim: u32,
    pub on_primary_fixed: u32,
    pub on_primary_fixed_variant: u32,
    pub secondary_fixed: u32,
    pub secondary_fixed_dim: u32,
    pub on_secondary_fixed: u32,
    pub on_secondary_fixed_variant: u32,
    pub tertiary_fixed: u32,
    pub tertiary_fixed_dim: u32,
    pub on_tertiary_fixed: u32,
    pub on_tertiary_fixed_variant: u32,

    // Utility
    pub scrim: u32,
    pub shadow: u32,
}

impl Default for MaterialColorScheme {
    fn default() -> Self {
        Self::dark_from_argb(0xFF6750A4)
    }
}

impl MaterialColorScheme {
    pub fn dark_from_argb(seed: u32) -> Self {
        let mut palette = CorePalette::from_argb(seed);
        palette.cache_all();
        Self::dark_from_palette(&mut palette)
    }

    pub fn light_from_argb(seed: u32) -> Self {
        let mut palette = CorePalette::from_argb(seed);
        palette.cache_all();
        Self::light_from_palette(&mut palette)
    }

    pub fn dark_from_palette(p: &mut CorePalette) -> Self {
        Self {
            primary: p.primary.tone(80),
            on_primary: p.primary.tone(20),
            primary_container: p.primary.tone(30),
            on_primary_container: p.primary.tone(90),

            secondary: p.secondary.tone(80),
            on_secondary: p.secondary.tone(20),
            secondary_container: p.secondary.tone(30),
            on_secondary_container: p.secondary.tone(90),

            tertiary: p.tertiary.tone(80),
            on_tertiary: p.tertiary.tone(20),
            tertiary_container: p.tertiary.tone(30),
            on_tertiary_container: p.tertiary.tone(90),

            error: p.error.tone(80),
            on_error: p.error.tone(20),
            error_container: p.error.tone(30),
            on_error_container: p.error.tone(90),

            surface: p.neutral.tone(6),
            surface_bright: p.neutral.tone(24),
            surface_dim: p.neutral.tone(6),
            on_surface: p.neutral.tone(90),
            on_surface_variant: p.neutral_variant.tone(80),

            surface_container_lowest: p.neutral.tone(4),
            surface_container_low: p.neutral.tone(10),
            surface_container: p.neutral.tone(12),
            surface_container_high: p.neutral.tone(17),
            surface_container_highest: p.neutral.tone(22),

            outline: p.neutral_variant.tone(60),
            outline_variant: p.neutral_variant.tone(30),

            inverse_surface: p.neutral.tone(90),
            inverse_on_surface: p.neutral.tone(20),
            inverse_primary: p.primary.tone(40),

            primary_fixed: p.primary.tone(90),
            primary_fixed_dim: p.primary.tone(80),
            on_primary_fixed: p.primary.tone(10),
            on_primary_fixed_variant: p.primary.tone(30),
            secondary_fixed: p.secondary.tone(90),
            secondary_fixed_dim: p.secondary.tone(80),
            on_secondary_fixed: p.secondary.tone(10),
            on_secondary_fixed_variant: p.secondary.tone(30),
            tertiary_fixed: p.tertiary.tone(90),
            tertiary_fixed_dim: p.tertiary.tone(80),
            on_tertiary_fixed: p.tertiary.tone(10),
            on_tertiary_fixed_variant: p.tertiary.tone(30),

            scrim: 0xFF000000,
            shadow: 0xFF000000,
        }
    }

    pub fn light_from_palette(p: &mut CorePalette) -> Self {
        Self {
            primary: p.primary.tone(40),
            on_primary: p.primary.tone(100),
            primary_container: p.primary.tone(90),
            on_primary_container: p.primary.tone(10),

            secondary: p.secondary.tone(40),
            on_secondary: p.secondary.tone(100),
            secondary_container: p.secondary.tone(90),
            on_secondary_container: p.secondary.tone(10),

            tertiary: p.tertiary.tone(40),
            on_tertiary: p.tertiary.tone(100),
            tertiary_container: p.tertiary.tone(90),
            on_tertiary_container: p.tertiary.tone(10),

            error: p.error.tone(40),
            on_error: p.error.tone(100),
            error_container: p.error.tone(90),
            on_error_container: p.error.tone(10),

            surface: p.neutral.tone(98),
            surface_bright: p.neutral.tone(98),
            surface_dim: p.neutral.tone(87),
            on_surface: p.neutral.tone(10),
            on_surface_variant: p.neutral_variant.tone(30),

            surface_container_lowest: p.neutral.tone(100),
            surface_container_low: p.neutral.tone(96),
            surface_container: p.neutral.tone(94),
            surface_container_high: p.neutral.tone(92),
            surface_container_highest: p.neutral.tone(90),

            outline: p.neutral_variant.tone(50),
            outline_variant: p.neutral_variant.tone(80),

            inverse_surface: p.neutral.tone(20),
            inverse_on_surface: p.neutral.tone(95),
            inverse_primary: p.primary.tone(80),

            primary_fixed: p.primary.tone(90),
            primary_fixed_dim: p.primary.tone(80),
            on_primary_fixed: p.primary.tone(10),
            on_primary_fixed_variant: p.primary.tone(30),
            secondary_fixed: p.secondary.tone(90),
            secondary_fixed_dim: p.secondary.tone(80),
            on_secondary_fixed: p.secondary.tone(10),
            on_secondary_fixed_variant: p.secondary.tone(30),
            tertiary_fixed: p.tertiary.tone(90),
            tertiary_fixed_dim: p.tertiary.tone(80),
            on_tertiary_fixed: p.tertiary.tone(10),
            on_tertiary_fixed_variant: p.tertiary.tone(30),

            scrim: 0xFF000000,
            shadow: 0xFF000000,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn luminance_from_argb(argb: u32) -> f32 {
        let r = ((argb >> 16) & 0xFF) as f32 / 255.0;
        let g = ((argb >> 8) & 0xFF) as f32 / 255.0;
        let b = (argb & 0xFF) as f32 / 255.0;
        0.299 * r + 0.587 * g + 0.114 * b
    }

    #[test]
    fn test_dark_scheme_generation() {
        let scheme = MaterialColorScheme::dark_from_argb(0xFF6750A4);
        assert!(luminance_from_argb(scheme.primary) > luminance_from_argb(scheme.on_primary));
    }

    #[test]
    fn test_light_scheme_generation() {
        let scheme = MaterialColorScheme::light_from_argb(0xFF6750A4);
        assert!(luminance_from_argb(scheme.on_primary) > luminance_from_argb(scheme.primary));
    }

    #[test]
    fn test_surface_hierarchy_dark() {
        let scheme = MaterialColorScheme::dark_from_argb(0xFF6750A4);
        assert!(
            luminance_from_argb(scheme.surface_container_lowest)
                < luminance_from_argb(scheme.surface_container_low)
        );
        assert!(
            luminance_from_argb(scheme.surface_container_low)
                < luminance_from_argb(scheme.surface_container)
        );
        assert!(
            luminance_from_argb(scheme.surface_container)
                < luminance_from_argb(scheme.surface_container_high)
        );
        assert!(
            luminance_from_argb(scheme.surface_container_high)
                < luminance_from_argb(scheme.surface_container_highest)
        );
    }

    #[test]
    fn test_error_colors() {
        let scheme = MaterialColorScheme::dark_from_argb(0xFF6750A4);
        let r = (scheme.error >> 16) & 0xFF;
        let b = scheme.error & 0xFF;
        assert!(r > b);
    }
}
