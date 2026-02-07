/// Image handling module
/// Handles image loading, screenshot capture, and format conversion

use anyhow::Result;
use image::{DynamicImage, RgbaImage};
use slint::{Image, SharedPixelBuffer, Rgb8Pixel};
use std::path::Path;

/// Load an image from a file path
pub fn load_image(path: &Path) -> Result<RgbaImage> {
    let img = image::open(path)?;
    Ok(img.to_rgba8())
}

/// Take a screenshot of the primary display
pub fn take_screenshot() -> Result<RgbaImage> {
    let screens = screenshots::Screen::all()?;
    
    if screens.is_empty() {
        return Err(anyhow::anyhow!("No screens found"));
    }
    
    // Capture the primary screen (first screen)
    let screen = &screens[0];
    let image = screen.capture()?;
    
    // Convert to RgbaImage
    let width = image.width();
    let height = image.height();
    let rgba = image.rgba();
    
    RgbaImage::from_raw(width, height, rgba.to_vec())
        .ok_or_else(|| anyhow::anyhow!("Failed to create image from screenshot"))
}

/// Convert an RgbaImage to a Slint Image
pub fn rgba_to_slint_image(img: &RgbaImage) -> Result<Image> {
    let width = img.width();
    let height = img.height();
    
    // Convert RGBA to RGB (Slint uses RGB8)
    let mut rgb_data = Vec::with_capacity((width * height * 3) as usize);
    
    for pixel in img.pixels() {
        rgb_data.push(pixel[0]); // R
        rgb_data.push(pixel[1]); // G
        rgb_data.push(pixel[2]); // B
        // Ignore alpha channel
    }
    
    let buffer = SharedPixelBuffer::<Rgb8Pixel>::clone_from_slice(
        &rgb_data,
        width,
        height,
    );
    
    Ok(Image::from_rgb8(buffer))
}

/// Resize image if it's too large (optional optimization)
pub fn resize_if_needed(img: RgbaImage, max_width: u32, max_height: u32) -> RgbaImage {
    let (width, height) = img.dimensions();
    
    if width <= max_width && height <= max_height {
        return img;
    }
    
    let ratio = (max_width as f32 / width as f32).min(max_height as f32 / height as f32);
    let new_width = (width as f32 * ratio) as u32;
    let new_height = (height as f32 * ratio) as u32;
    
    image::imageops::resize(
        &img,
        new_width,
        new_height,
        image::imageops::FilterType::Lanczos3,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_screenshot_available() {
        // Just check if we can query screens
        let screens = screenshots::Screen::all();
        assert!(screens.is_ok());
    }
}
