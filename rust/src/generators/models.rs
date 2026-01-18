//! AI model configuration and registry.
//! 
//! Contains the configuration for all supported background removal models.

use super::ModelType;

/// Configuration for an AI model.
pub struct ModelConfig {
    pub name: String,
    pub url: String,
    pub filename: String,
    pub resolution: u32,
    pub size_mb: u32,
}

/// Returns the configuration for a given model type.
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
