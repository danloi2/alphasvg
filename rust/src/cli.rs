//! Command-line interface batch processing.
//!
//! Handles batch image processing when run from the command line.

use std::path::Path;
use walkdir::WalkDir;
use anyhow::Result;

use crate::generators::{self, LogOutput, ModelState, ModelType};
use crate::lang::LanguageManager;

/// Processes all images in a directory.
pub fn process_batch(input_dir: &str, output_dir: &str, lang: &LanguageManager, logger: &LogOutput) -> Result<()> {
    let input_path = Path::new(input_dir);
    let output_path = Path::new(output_dir);

    if !input_path.exists() {
        println!("❌ Input directory not found: {}", input_dir);
        return Ok(());
    }

    std::fs::create_dir_all(output_path)?;

    let extensions = ["png", "jpg", "jpeg"];
    let mut files = Vec::new();

    for entry in WalkDir::new(input_path).max_depth(1) {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                if extensions.contains(&ext.to_lowercase().as_str()) {
                    let name = path.file_name().unwrap().to_str().unwrap();
                    if !name.contains(".temp.") && !name.contains(".vtrace_temp.") {
                        files.push(path.to_path_buf());
                    }
                }
            }
        }
    }

    if files.is_empty() {
        println!("ℹ️ No image files found in {}", input_dir);
        return Ok(());
    }

    println!("🚀 Processing {} images modularly...", files.len());

    for file_path in files {
        process_single_image(&file_path, output_path, lang, logger)?;
    }

    println!("\n✅ All image processing complete.");
    Ok(())
}

/// Processes a single image through all generation pipelines.
fn process_single_image(input_path: &Path, output_dir: &Path, lang: &LanguageManager, logger: &LogOutput) -> Result<()> {
    let file_name = input_path.file_stem().unwrap().to_str().unwrap();
    let base_name = format!("{}_alpha", file_name);

    let alpha_path = output_dir.join(format!("{}.png", base_name));
    let gray_path = output_dir.join(format!("{}_gray.svg", base_name));
    let halftone_path = output_dir.join(format!("{}_halftone.svg", base_name));
    let lineart_path = output_dir.join(format!("{}_lineart.svg", base_name));
    let color_logo_path = output_dir.join(format!("{}_color_logo.svg", base_name));
    let color_illus_path = output_dir.join(format!("{}_color_illus.svg", base_name));
    let thumb_path = output_dir.join(format!("{}_thumb.png", base_name));

    println!("\n📦 Processing: {:?}...", input_path.file_name().unwrap());

    // 1. Generate the AI-processed Alpha PNG first
    let dummy_status = std::sync::Arc::new(std::sync::Mutex::new(ModelState::Unloaded));
    let img = generators::generate_alpha_png(input_path, Some(&alpha_path), lang, logger, &dummy_status, ModelType::default())?;

    // 2. Use the processed Alpha PNG as source for everything else
    generators::generate_grayscale_svg(&img, &gray_path, 8, lang, logger)?;
    generators::generate_halftone_svg(&img, &halftone_path, lang, logger)?;
    generators::generate_lineart_svg(&img, &lineart_path, lang, logger)?;
    generators::generate_logo(&img, &color_logo_path, lang, logger)?;
    generators::generate_illustration(&img, &color_illus_path, lang, logger)?;
    generators::generate_thumbnail(&img, &thumb_path, lang, logger)?;

    Ok(())
}
