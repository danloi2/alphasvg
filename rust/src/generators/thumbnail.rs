use image::{DynamicImage, imageops::FilterType, GenericImageView};
use std::path::Path;
use anyhow::Result;
use crate::config;
use crate::lang::LanguageManager;
use crate::generators::LogOutput;

pub fn generate_thumbnail(img: &DynamicImage, output_path: &Path, lang: &LanguageManager, logger: &LogOutput) -> Result<()> {
    let (width, height) = img.dimensions();
    let aspect_ratio = height as f32 / width as f32;
    let new_height = (config::THUMB_WIDTH as f32 * aspect_ratio) as u32;
    
    let thumb = img.resize(config::THUMB_WIDTH, new_height, FilterType::Lanczos3);
    thumb.save(output_path)?;
    logger.send(format!("{}{:?}", lang.t("log_thumb_ok"), output_path.file_name().unwrap()));
    Ok(())
}
