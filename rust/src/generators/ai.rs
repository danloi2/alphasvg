use image::{DynamicImage, Luma, imageops::FilterType};
use std::path::{Path, PathBuf};
use std::fs;
use anyhow::{Result, Context, anyhow};
use ort::{inputs, session::Session, value::Value};
use ndarray::Array4;
use std::sync::{Mutex, Arc};
use crate::lang::LanguageManager;
use crate::generators::{LogOutput, ModelState, ModelType};

static SESSION: Mutex<Option<(ModelType, Session)>> = Mutex::new(None);

pub struct ModelConfig {
    pub name: String,
    pub url: String,
    pub filename: String,
    pub resolution: u32,
    pub size_mb: u32,
}

pub fn get_model_config(model: ModelType) -> ModelConfig {
    match model {
        ModelType::U2Net => ModelConfig {
            name: "u2net".to_string(),
            url: "https://github.com/danielgatis/rembg/releases/download/v0.0.0/u2net.onnx".to_string(),
            filename: "u2net.onnx".to_string(),
            resolution: 320,
            size_mb: 170,
        },
        ModelType::U2NetP => ModelConfig {
            name: "u2netp".to_string(),
            url: "https://github.com/danielgatis/rembg/releases/download/v0.0.0/u2netp.onnx".to_string(),
            filename: "u2netp.onnx".to_string(),
            resolution: 320,
            size_mb: 4,
        },
        ModelType::U2NetHumanSeg => ModelConfig {
            name: "u2net_human_seg".to_string(),
            url: "https://github.com/danielgatis/rembg/releases/download/v0.0.0/u2net_human_seg.onnx".to_string(),
            filename: "u2net_human_seg.onnx".to_string(),
            resolution: 320,
            size_mb: 170,
        },
        ModelType::U2NetClothSeg => ModelConfig {
            name: "u2net_cloth_seg".to_string(),
            url: "https://github.com/danielgatis/rembg/releases/download/v0.0.0/u2net_cloth_seg.onnx".to_string(),
            filename: "u2net_cloth_seg.onnx".to_string(),
            resolution: 320,
            size_mb: 170,
        },
        ModelType::Silueta => ModelConfig {
            name: "silueta".to_string(),
            url: "https://github.com/danielgatis/rembg/releases/download/v0.0.0/silueta.onnx".to_string(),
            filename: "silueta.onnx".to_string(),
            resolution: 320,
            size_mb: 43,
        },
        ModelType::IsNetGeneralUse => ModelConfig {
            name: "isnet-general-use".to_string(),
            url: "https://github.com/danielgatis/rembg/releases/download/v0.0.0/isnet-general-use.onnx".to_string(),
            filename: "isnet-general-use.onnx".to_string(),
            resolution: 1024,
            size_mb: 176,
        },
        ModelType::IsNetAnime => ModelConfig {
            name: "isnet-anime".to_string(),
            url: "https://github.com/danielgatis/rembg/releases/download/v0.0.0/isnet-anime.onnx".to_string(),
            filename: "isnet-anime.onnx".to_string(),
            resolution: 1024,
            size_mb: 176,
        },
        ModelType::Sam => ModelConfig {
            name: "sam".to_string(),
            url: "https://github.com/danielgatis/rembg/releases/download/v0.0.0/vit_b-encoder-quant.onnx".to_string(),
            filename: "sam-encoder.onnx".to_string(),
            resolution: 1024,
            size_mb: 358,
        },
        ModelType::BiRefNetGeneral => ModelConfig {
            name: "birefnet-general".to_string(),
            url: "https://github.com/danielgatis/rembg/releases/download/v0.0.0/BiRefNet-general-epoch_244.onnx".to_string(),
            filename: "birefnet-general.onnx".to_string(),
            resolution: 1024,
            size_mb: 290,
        },
        ModelType::BiRefNetGeneralLite => ModelConfig {
            name: "birefnet-general-lite".to_string(),
            url: "https://github.com/danielgatis/rembg/releases/download/v0.0.0/BiRefNet-general-bb_swin_v1_tiny-epoch_232.onnx".to_string(),
            filename: "birefnet-general-lite.onnx".to_string(),
            resolution: 1024,
            size_mb: 145,
        },
        ModelType::BiRefNetPortrait => ModelConfig {
            name: "birefnet-portrait".to_string(),
            url: "https://github.com/danielgatis/rembg/releases/download/v0.0.0/BiRefNet-portrait-epoch_150.onnx".to_string(),
            filename: "birefnet-portrait.onnx".to_string(),
            resolution: 1024,
            size_mb: 290,
        },
        ModelType::BiRefNetDis => ModelConfig {
            name: "birefnet-dis".to_string(),
            url: "https://github.com/danielgatis/rembg/releases/download/v0.0.0/BiRefNet-DIS-epoch_590.onnx".to_string(),
            filename: "birefnet-dis.onnx".to_string(),
            resolution: 1024,
            size_mb: 290,
        },
        ModelType::BiRefNetHrsod => ModelConfig {
            name: "birefnet-hrsod".to_string(),
            url: "https://github.com/danielgatis/rembg/releases/download/v0.0.0/BiRefNet-HRSOD_DHU-epoch_115.onnx".to_string(),
            filename: "birefnet-hrsod.onnx".to_string(),
            resolution: 1024,
            size_mb: 290,
        },
        ModelType::BiRefNetCod => ModelConfig {
            name: "birefnet-cod".to_string(),
            url: "https://github.com/danielgatis/rembg/releases/download/v0.0.0/BiRefNet-COD-epoch_125.onnx".to_string(),
            filename: "birefnet-cod.onnx".to_string(),
            resolution: 1024,
            size_mb: 290,
        },
        ModelType::BiRefNetMassive => ModelConfig {
            name: "birefnet-massive".to_string(),
            url: "https://github.com/danielgatis/rembg/releases/download/v0.0.0/BiRefNet-massive-TR_DIS5K_TR_TEs-epoch_420.onnx".to_string(),
            filename: "birefnet-massive.onnx".to_string(),
            resolution: 1024,
            size_mb: 290,
        },
        ModelType::BriaRmbg => ModelConfig {
            name: "bria-rmbg".to_string(),
            url: "https://github.com/danielgatis/rembg/releases/download/v0.0.0/bria-rmbg-2.0.onnx".to_string(),
            filename: "bria-rmbg.onnx".to_string(),
            resolution: 1024,
            size_mb: 72,
        },
    }
}

/// Performs AI inference to get a transparency mask (saliency map).
/// Returns a Luma image of the mask.
pub fn get_model_mask(
    img: &DynamicImage, 
    lang: &LanguageManager, 
    logger: &LogOutput, 
    status: &Arc<Mutex<ModelState>>,
    model_type: ModelType,
) -> Result<image::ImageBuffer<Luma<u8>, Vec<u8>>> {
    
    let config = get_model_config(model_type);
    let model_path = prepare_model(lang, logger, status, &config)?;
    let mut session_guard = SESSION.lock().map_err(|_| anyhow!("Failed to lock session mutex"))?;

    process_model_mask(img, lang, logger, status, model_type, &config, &model_path, &mut session_guard)
}

fn process_model_mask(
    img: &DynamicImage,
    lang: &LanguageManager,
    logger: &LogOutput,
    status: &Arc<Mutex<ModelState>>,
    model_type: ModelType,
    config: &ModelConfig,
    model_path: &Path,
    session_guard: &mut Option<(ModelType, Session)>,
) -> Result<image::ImageBuffer<Luma<u8>, Vec<u8>>> {
    
    // Ensure the session is initialized for the correct model
    let is_correct_model = if let Some((current_type, _)) = session_guard {
        *current_type == model_type
    } else {
        false
    };

    if !is_correct_model {
        {
            let mut s = status.lock().unwrap();
            *s = ModelState::Loading;
        }
        logger.send(lang.t("log_loading_model"));
        
        let new_session = Session::builder()?
            .commit_from_file(model_path)
            .map_err(|e| anyhow!("Failed to load ONNX model {}: {}", config.name, e))?;
            
        *session_guard = Some((model_type, new_session));
    }

    let (_, session) = session_guard.as_mut().unwrap();
    
    {
        let mut s = status.lock().unwrap();
        *s = ModelState::Ready(config.name.clone());
    }

    let rgba = img.to_rgba8();
    let (width, height) = rgba.dimensions();
    let res = config.resolution;

    // 1. Pre-process
    let resized = img.resize_exact(res, res, FilterType::Lanczos3);
    let mut input_array = Array4::<f32>::zeros((1, 3, res as usize, res as usize));
    
    for (x, y, pixel) in resized.to_rgb8().enumerate_pixels() {
        input_array[[0, 0, y as usize, x as usize]] = (pixel[0] as f32 / 255.0 - 0.485) / 0.229;
        input_array[[0, 1, y as usize, x as usize]] = (pixel[1] as f32 / 255.0 - 0.456) / 0.224;
        input_array[[0, 2, y as usize, x as usize]] = (pixel[2] as f32 / 255.0 - 0.406) / 0.225;
    }

    // 2. Inference
    logger.send(lang.t("log_inference"));
    let shape = vec![1, 3, res as usize, res as usize];
    let data = input_array.into_raw_vec_and_offset().0.into_boxed_slice();
    let input_tensor = Value::from_array((shape, data))?;
    
    let input_name = session.inputs()[0].name().to_string();
    let output_name = session.outputs()[0].name().to_string();

    let input_map = inputs![input_name => input_tensor];
    let outputs = session.run(input_map)?;
    
    let (_mask_shape, mask_slice) = outputs[output_name].try_extract_tensor::<f32>()?;

    // 3. Post-process mask
    let mut mask_img = image::ImageBuffer::new(res, res);
    for y in 0..res {
        for x in 0..res {
            let val = mask_slice[(y * res + x) as usize];
            let pixel_val = (val * 255.0).clamp(0.0, 255.0) as u8;
            mask_img.put_pixel(x, y, Luma([pixel_val]));
        }
    }

    // Resize mask back to original size
    let mask_resized = image::DynamicImage::ImageLuma8(mask_img)
        .resize_exact(width, height, FilterType::Lanczos3)
        .to_luma8();

    Ok(mask_resized)
}

fn prepare_model(lang: &LanguageManager, logger: &LogOutput, status: &Arc<Mutex<ModelState>>, config: &ModelConfig) -> Result<PathBuf> {
    let home = dirs::home_dir().context("Could not find home directory")?;
    let model_dir = home.join(".transparente_models");
    fs::create_dir_all(&model_dir)?;
    let model_path = model_dir.join(&config.filename);

    let needs_download = if !model_path.exists() {
        true
    } else {
        // Check for "Not Found" or empty files (min 1MB)
        let len = fs::metadata(&model_path)?.len();
        len < 1024 * 1024 // Less than 1MB is almost certainly a 404 or corrupt model
    };

    if needs_download {
        {
            let mut s = status.lock().unwrap();
            *s = ModelState::Loading;
        }
        let msg = format!("{} {} (~{}MB)...", lang.t("log_downloading_model_generic"), config.name, config.size_mb);
        logger.send(msg);
        
        let mut response = reqwest::blocking::get(&config.url)?;
        if !response.status().is_success() {
            return Err(anyhow!("Failed to download model {}: HTTP {}", config.name, response.status()));
        }

        let mut file = fs::File::create(&model_path)?;
        response.copy_to(&mut file)?;
        
        // Final check after download
        let len = fs::metadata(&model_path)?.len();
        if len < 1024 * 1024 {
            let _ = fs::remove_file(&model_path); // Clean up
            return Err(anyhow!("Downloaded model {} is too small (corrupt or invalid URL)", config.name));
        }
        
        logger.send(lang.t("log_model_downloaded"));
    }

    Ok(model_path)
}
