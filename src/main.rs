slint::include_modules!();

mod colorblind;
mod color_parser;
mod image_handler;

use slint::{Image, SharedPixelBuffer, Rgb8Pixel, VecModel, ModelRc};
use std::rc::Rc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();

    // Store current image data and color palette
    let current_image: Rc<std::cell::RefCell<Option<image::RgbaImage>>> = 
        Rc::new(std::cell::RefCell::new(None));
    
    // Store original colors (before simulation)
    let original_colors: Rc<std::cell::RefCell<Vec<(u8, u8, u8)>>> = 
        Rc::new(std::cell::RefCell::new(Vec::new()));

    // Load Image callback with resizing
    {
        let ui_handle = ui_handle.clone();
        let current_image = current_image.clone();
        
        ui.on_load_image(move || {
            if let Some(ui) = ui_handle.upgrade() {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("Images", &["png", "jpg", "jpeg", "bmp", "gif"])
                    .pick_file()
                {
                    match image_handler::load_image(&path) {
                        Ok(img) => {
                            // Resize to fit window (max 700x400 for display)
                            let resized = image_handler::resize_if_needed(img, 700, 400);
                            *current_image.borrow_mut() = Some(resized.clone());
                            
                            if let Ok(slint_img) = image_handler::rgba_to_slint_image(&resized) {
                                ui.set_preview_image(slint_img);
                                ui.set_has_image(true);
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to load image: {}", e);
                        }
                    }
                }
            }
        });
    }

    // Take Screenshot callback with resizing
    {
        let ui_handle = ui_handle.clone();
        let current_image = current_image.clone();
        
        ui.on_take_screenshot(move || {
            if let Some(ui) = ui_handle.upgrade() {
                match image_handler::take_screenshot() {
                    Ok(img) => {
                        // Resize to fit window
                        let resized = image_handler::resize_if_needed(img, 700, 400);
                        *current_image.borrow_mut() = Some(resized.clone());
                        
                        if let Ok(slint_img) = image_handler::rgba_to_slint_image(&resized) {
                            ui.set_preview_image(slint_img);
                            ui.set_has_image(true);
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to take screenshot: {}", e);
                    }
                }
            }
        });
    }

    // Add Color callback
    {
        let ui_handle = ui_handle.clone();
        let original_colors = original_colors.clone();
        
        ui.on_add_color(move |input| {
            if let Some(ui) = ui_handle.upgrade() {
                match color_parser::parse_color(&input) {
                    Ok((r, g, b)) => {
                        // Add to original colors
                        original_colors.borrow_mut().push((r, g, b));
                        
                        // Update the display
                        update_color_palette(&ui, &original_colors.borrow(), colorblind::ColorBlindnessType::Normal);
                        
                        ui.set_color_input("".into());
                        ui.set_status_message(format!("Added color: RGB({}, {}, {})", r, g, b).into());
                    }
                    Err(e) => {
                        ui.set_status_message(format!("Error: {}", e).into());
                    }
                }
            }
        });
    }

    // Remove Color callback
    {
        let ui_handle = ui_handle.clone();
        let original_colors = original_colors.clone();
        
        ui.on_remove_color(move |index| {
            if let Some(ui) = ui_handle.upgrade() {
                let mut colors = original_colors.borrow_mut();
                if (index as usize) < colors.len() {
                    colors.remove(index as usize);
                    drop(colors); // Release borrow before update
                    
                    update_color_palette(&ui, &original_colors.borrow(), colorblind::ColorBlindnessType::Normal);
                    ui.set_status_message(format!("Removed color at position {}", index + 1).into());
                }
            }
        });
    }

    // Simulate Color Blindness callback
    {
        let ui_handle = ui_handle.clone();
        let current_image = current_image.clone();
        let original_colors = original_colors.clone();
        
        ui.on_simulate_colorblindness(move |simulation_type| {
            if let Some(ui) = ui_handle.upgrade() {
                let cb_type = match simulation_type.as_str() {
                    "Protanopia (Red-Blind)" => colorblind::ColorBlindnessType::Protanopia,
                    "Deuteranopia (Green-Blind)" => colorblind::ColorBlindnessType::Deuteranopia,
                    "Tritanopia (Blue-Blind)" => colorblind::ColorBlindnessType::Tritanopia,
                    "Protanomaly" => colorblind::ColorBlindnessType::Protanomaly,
                    "Deuteranomaly" => colorblind::ColorBlindnessType::Deuteranomaly,
                    "Tritanomaly" => colorblind::ColorBlindnessType::Tritanomaly,
                    "Achromatopsia" => colorblind::ColorBlindnessType::Achromatopsia,
                    _ => colorblind::ColorBlindnessType::Normal,
                };

                // Update color palette with simulation
                update_color_palette(&ui, &original_colors.borrow(), cb_type);

                // Simulate for image if loaded
                if let Some(img) = current_image.borrow().as_ref() {
                    let simulated = colorblind::simulate_image_colorblindness(img, cb_type);
                    if let Ok(slint_img) = image_handler::rgba_to_slint_image(&simulated) {
                        ui.set_preview_image(slint_img);
                    }
                }
            }
        });
    }

    ui.run()?;
    Ok(())
}

fn update_color_palette(ui: &AppWindow, colors: &[(u8, u8, u8)], cb_type: colorblind::ColorBlindnessType) {
    let color_items: Vec<ColorItem> = colors.iter().map(|&(r, g, b)| {
        let (sim_r, sim_g, sim_b) = colorblind::simulate_color_blindness(r, g, b, cb_type);
        
        ColorItem {
            color: slint::Color::from_rgb_u8(sim_r, sim_g, sim_b),
            original_rgb: format!("RGB({}, {}, {}) / #{:02X}{:02X}{:02X}", r, g, b, r, g, b).into(),
            simulated_rgb: if matches!(cb_type, colorblind::ColorBlindnessType::Normal) {
                "No simulation applied".into()
            } else {
                format!("RGB({}, {}, {}) / #{:02X}{:02X}{:02X}", sim_r, sim_g, sim_b, sim_r, sim_g, sim_b).into()
            },
        }
    }).collect();
    
    let model = Rc::new(VecModel::from(color_items));
    ui.set_color_palette(ModelRc::from(model));
}
