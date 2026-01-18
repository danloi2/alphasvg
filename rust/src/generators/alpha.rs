use image::{DynamicImage, RgbaImage};
use std::path::Path;
use anyhow::Result;
use crate::config;
use std::sync::{Mutex, Arc};
use crate::lang::LanguageManager;
use crate::generators::{LogOutput, ModelState, ModelType, ai};

pub fn generate_alpha_png(input_path: &Path, output_path: Option<&Path>, lang: &LanguageManager, logger: &LogOutput, status: &Arc<Mutex<ModelState>>, model_type: ModelType) -> Result<DynamicImage> {
    // If output path is provided and exists, return loaded image (Cache)
    if let Some(path) = output_path {
        if path.exists() {
            return Ok(image::open(path)?);
        }
    }

    let img = image::open(input_path)?;
    let rgba = img.to_rgba8();
    
    // 1. Get Mask from AI module
    let mask_resized = ai::get_model_mask(&img, lang, logger, status, model_type)?;

    // 2. Apply mask to original image
    let mut final_img = rgba.clone();
    for (x, y, pixel) in final_img.enumerate_pixels_mut() {
        let mask_val = mask_resized.get_pixel(x, y)[0];
        pixel[3] = (pixel[3] as u16 * mask_val as u16 / 255) as u8;
    }

    // 3. Post-processing Refinements
    clean_white_halo(&mut final_img);
    refine_alpha(&mut final_img);

    if let Some(path) = output_path {
         final_img.save(path)?;
         logger.send(format!("{}{:?}", lang.t("log_alpha_ok"), path.file_name().unwrap()));
    } else {
         logger.send(lang.t("log_alpha_mem"));
    }
    
    Ok(DynamicImage::ImageRgba8(final_img))
}

fn clean_white_halo(img: &mut RgbaImage) {
    let [tr_r, tr_g, tr_b] = config::TRANSPARENT_COLOR;
    let tol = config::TOLERANCE;
    let strength = config::DESPILL_STRENGTH;

    for pixel in img.pixels_mut() {
        let [r, g, b, a] = pixel.0;
        if a > 0 {
            let rd = (r as i16 - tr_r as i16).abs() as u8;
            let gd = (g as i16 - tr_g as i16).abs() as u8;
            let bd = (b as i16 - tr_b as i16).abs() as u8;

            if rd <= tol && gd <= tol && bd <= tol {
                pixel.0[3] = 0;
                pixel.0[0] = (r as f32 * strength) as u8;
                pixel.0[1] = (g as f32 * strength) as u8;
                pixel.0[2] = (b as f32 * strength) as u8;
            }
        }
    }
}

fn refine_alpha(img: &mut RgbaImage) {
    for pixel in img.pixels_mut() {
        if pixel.0[3] < config::MIN_ALPHA {
            pixel.0[3] = 0;
        }
    }
}
