use image::{DynamicImage, RgbaImage, Luma, imageops::FilterType};
use std::path::{Path, PathBuf};
use std::fs;
use anyhow::{Result, Context, anyhow};
use crate::config;
use ort::{inputs, session::Session, value::Value};
use ndarray::Array4;
use std::sync::{OnceLock, Mutex};

static SESSION: OnceLock<Mutex<Session>> = OnceLock::new();

pub fn generate_alpha_png(input_path: &Path, output_path: &Path) -> Result<DynamicImage> {
    if output_path.exists() {
        return Ok(image::open(output_path)?);
    }

    let model_path = prepare_model()?;
    let mutex = SESSION.get_or_init(|| {
        println!("🚀 Loading ONNX model (ort)...");
        let session = Session::builder().unwrap().commit_from_file(model_path).expect("Failed to load model");
        Mutex::new(session)
    });

    let mut session = mutex.lock().map_err(|_| anyhow!("Failed to lock session mutex"))?;

    let img = image::open(input_path)?.to_rgba8();
    let (width, height) = img.dimensions();

    // 1. Pre-process
    let resized = DynamicImage::ImageRgba8(img.clone()).resize_exact(320, 320, FilterType::Lanczos3);
    let mut input_array = Array4::<f32>::zeros((1, 3, 320, 320));
    
    for (x, y, pixel) in resized.to_rgb8().enumerate_pixels() {
        input_array[[0, 0, y as usize, x as usize]] = (pixel[0] as f32 / 255.0 - 0.485) / 0.229;
        input_array[[0, 1, y as usize, x as usize]] = (pixel[1] as f32 / 255.0 - 0.456) / 0.224;
        input_array[[0, 2, y as usize, x as usize]] = (pixel[2] as f32 / 255.0 - 0.406) / 0.225;
    }

    // 2. Inference
    println!("🧠 Running AI Inference...");
    let shape = vec![1, 3, 320, 320];
    let data = input_array.into_raw_vec_and_offset().0.into_boxed_slice();
    let input_tensor = Value::from_array((shape, data))?;
    
    // Identify IO names BEFORE the inference call to avoid borrow conflicts
    let input_name = session.inputs()[0].name().to_string();
    let output_name = session.outputs()[0].name().to_string();

    let input_map = inputs![input_name => input_tensor];
    let outputs = session.run(input_map)?;
    
    let (_mask_shape, mask_slice) = outputs[output_name].try_extract_tensor::<f32>()?;

    // 3. Post-process mask
    let mut mask_img = image::ImageBuffer::new(320, 320);
    for y in 0..320 {
        for x in 0..320 {
            let val = mask_slice[y * 320 + x];
            let pixel_val = (val * 255.0).clamp(0.0, 255.0) as u8;
            mask_img.put_pixel(x as u32, y as u32, Luma([pixel_val]));
        }
    }

    // Resize mask back to original size
    let mask_resized = DynamicImage::ImageLuma8(mask_img).resize_exact(width, height, FilterType::Lanczos3).to_luma8();

    // Apply mask to original image
    let mut final_img = img.clone();
    for (x, y, pixel) in final_img.enumerate_pixels_mut() {
        let mask_val = mask_resized.get_pixel(x, y)[0];
        pixel[3] = (pixel[3] as u16 * mask_val as u16 / 255) as u8;
    }

    // --- Refinado avanzado ---
    clean_white_halo(&mut final_img);
    refine_alpha(&mut final_img);

    final_img.save(output_path)?;
    println!("🖼 PNG Alpha OK (Native): {:?}", output_path.file_name().unwrap());
    
    Ok(DynamicImage::ImageRgba8(final_img))
}

fn prepare_model() -> Result<PathBuf> {
    let home = dirs::home_dir().context("Could not find home directory")?;
    let model_dir = home.join(".transparente_models");
    fs::create_dir_all(&model_dir)?;
    let model_path = model_dir.join("u2net.onnx");

    if !model_path.exists() {
        println!("📥 Downloading U2NET model (~170MB)...");
        let mut response = reqwest::blocking::get("https://github.com/danielgatis/rembg/releases/download/v0.0.0/u2net.onnx")?;
        let mut file = fs::File::create(&model_path)?;
        response.copy_to(&mut file)?;
        println!("✅ Model downloaded successfully.");
    }

    Ok(model_path)
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
    // Basic implementation of alpha thresholding for now
    for pixel in img.pixels_mut() {
        if pixel.0[3] < config::MIN_ALPHA {
            pixel.0[3] = 0;
        }
    }
    // Note: Gaussian blur and morphology are harder to implement natively without extra crates like imageproc.
    // For now, we stay with simple thresholding.
}
