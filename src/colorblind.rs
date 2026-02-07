/// Color blindness simulation module
/// Based on algorithms from:
/// - Brettel, H., Viénot, F., & Mollon, J. D. (1997)
/// - Digital approximations for color vision deficiency simulation

#[derive(Debug, Clone, Copy)]
pub enum ColorBlindnessType {
    Normal,
    Protanopia,      // Red-blind
    Deuteranopia,    // Green-blind
    Tritanopia,      // Blue-blind
    Protanomaly,     // Red-weak
    Deuteranomaly,   // Green-weak
    Tritanomaly,     // Blue-weak
    Achromatopsia,   // Total color blindness
}

/// Simulate color blindness on a single RGB color
pub fn simulate_color_blindness(r: u8, g: u8, b: u8, cb_type: ColorBlindnessType) -> (u8, u8, u8) {
    match cb_type {
        ColorBlindnessType::Normal => (r, g, b),
        ColorBlindnessType::Protanopia => simulate_protanopia(r, g, b),
        ColorBlindnessType::Deuteranopia => simulate_deuteranopia(r, g, b),
        ColorBlindnessType::Tritanopia => simulate_tritanopia(r, g, b),
        ColorBlindnessType::Protanomaly => simulate_protanomaly(r, g, b),
        ColorBlindnessType::Deuteranomaly => simulate_deuteranomaly(r, g, b),
        ColorBlindnessType::Tritanomaly => simulate_tritanomaly(r, g, b),
        ColorBlindnessType::Achromatopsia => simulate_achromatopsia(r, g, b),
    }
}

/// Simulate color blindness on an entire image
pub fn simulate_image_colorblindness(
    img: &image::RgbaImage,
    cb_type: ColorBlindnessType,
) -> image::RgbaImage {
    let mut output = img.clone();
    
    for pixel in output.pixels_mut() {
        let (r, g, b) = simulate_color_blindness(pixel[0], pixel[1], pixel[2], cb_type);
        pixel[0] = r;
        pixel[1] = g;
        pixel[2] = b;
        // Keep alpha channel unchanged
    }
    
    output
}

// Protanopia (Red-blind) - uses simplified transformation matrix
fn simulate_protanopia(r: u8, g: u8, b: u8) -> (u8, u8, u8) {
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;

    let new_r = 0.567 * r + 0.433 * g;
    let new_g = 0.558 * r + 0.442 * g;
    let new_b = 0.242 * g + 0.758 * b;

    (
        (new_r * 255.0).clamp(0.0, 255.0) as u8,
        (new_g * 255.0).clamp(0.0, 255.0) as u8,
        (new_b * 255.0).clamp(0.0, 255.0) as u8,
    )
}

// Deuteranopia (Green-blind)
fn simulate_deuteranopia(r: u8, g: u8, b: u8) -> (u8, u8, u8) {
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;

    let new_r = 0.625 * r + 0.375 * g;
    let new_g = 0.7 * r + 0.3 * g;
    let new_b = 0.3 * g + 0.7 * b;

    (
        (new_r * 255.0).clamp(0.0, 255.0) as u8,
        (new_g * 255.0).clamp(0.0, 255.0) as u8,
        (new_b * 255.0).clamp(0.0, 255.0) as u8,
    )
}

// Tritanopia (Blue-blind)
fn simulate_tritanopia(r: u8, g: u8, b: u8) -> (u8, u8, u8) {
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;

    let new_r = 0.95 * r + 0.05 * g;
    let new_g = 0.433 * g + 0.567 * b;
    let new_b = 0.475 * g + 0.525 * b;

    (
        (new_r * 255.0).clamp(0.0, 255.0) as u8,
        (new_g * 255.0).clamp(0.0, 255.0) as u8,
        (new_b * 255.0).clamp(0.0, 255.0) as u8,
    )
}

// Protanomaly (Red-weak) - partial red deficiency
fn simulate_protanomaly(r: u8, g: u8, b: u8) -> (u8, u8, u8) {
    let (sim_r, sim_g, sim_b) = simulate_protanopia(r, g, b);
    blend_colors(r, g, b, sim_r, sim_g, sim_b, 0.6)
}

// Deuteranomaly (Green-weak) - partial green deficiency
fn simulate_deuteranomaly(r: u8, g: u8, b: u8) -> (u8, u8, u8) {
    let (sim_r, sim_g, sim_b) = simulate_deuteranopia(r, g, b);
    blend_colors(r, g, b, sim_r, sim_g, sim_b, 0.6)
}

// Tritanomaly (Blue-weak) - partial blue deficiency
fn simulate_tritanomaly(r: u8, g: u8, b: u8) -> (u8, u8, u8) {
    let (sim_r, sim_g, sim_b) = simulate_tritanopia(r, g, b);
    blend_colors(r, g, b, sim_r, sim_g, sim_b, 0.6)
}

// Achromatopsia (Total color blindness) - grayscale
fn simulate_achromatopsia(r: u8, g: u8, b: u8) -> (u8, u8, u8) {
    // Standard luminance formula
    let gray = (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) as u8;
    (gray, gray, gray)
}

// Helper function to blend normal and simulated colors
fn blend_colors(
    r1: u8, g1: u8, b1: u8,
    r2: u8, g2: u8, b2: u8,
    factor: f32,
) -> (u8, u8, u8) {
    let r = (r1 as f32 * (1.0 - factor) + r2 as f32 * factor) as u8;
    let g = (g1 as f32 * (1.0 - factor) + g2 as f32 * factor) as u8;
    let b = (b1 as f32 * (1.0 - factor) + b2 as f32 * factor) as u8;
    (r, g, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_vision() {
        let (r, g, b) = simulate_color_blindness(255, 128, 64, ColorBlindnessType::Normal);
        assert_eq!((r, g, b), (255, 128, 64));
    }

    #[test]
    fn test_achromatopsia() {
        let (r, g, b) = simulate_color_blindness(255, 0, 0, ColorBlindnessType::Achromatopsia);
        assert_eq!(r, g);
        assert_eq!(g, b);
    }
}
