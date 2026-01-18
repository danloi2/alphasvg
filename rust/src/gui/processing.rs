//! Processing logic for the GUI.

use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;
use anyhow::{Result, Context};
use eframe::egui;

use crate::generators::{self, LogOutput, ModelState, ModelType};
use crate::lang::LanguageManager;

/// Returns the localized description for a given AI model type.
pub fn get_model_description_localized(lang: &LanguageManager, model: ModelType) -> String {
    use ModelType::*;
    let key = match model {
        U2Net => "desc_u2net",
        U2NetP => "desc_u2netp",
        U2NetHumanSeg => "desc_u2net_human",
        U2NetClothSeg => "desc_u2net_cloth",
        Silueta => "desc_silueta",
        IsNetGeneralUse => "desc_isnet_general",
        IsNetAnime => "desc_isnet_anime",
        Sam => "desc_sam",
        BiRefNetGeneral => "desc_birefnet_general",
        BiRefNetGeneralLite => "desc_birefnet_general",
        BiRefNetPortrait => "desc_birefnet_portrait",
        BiRefNetDis => "desc_birefnet_dis",
        BiRefNetHrsod => "desc_birefnet_hrsod",
        BiRefNetCod => "desc_birefnet_cod",
        BiRefNetMassive => "desc_birefnet_massive",
        BriaRmbg => "desc_briarmbg",
    };
    lang.t(key)
}

/// Configuration for starting a processing job.
pub struct ProcessingConfig {
    pub input: PathBuf,
    pub output: PathBuf,
    pub custom_filename: String,
    pub gen_alpha: bool,
    pub gen_gray: bool,
    pub gen_halftone: bool,
    pub gen_lineart: bool,
    pub gen_logo: bool,
    pub gen_illus: bool,
    pub gen_thumbnail: bool,
    pub selected_model: ModelType,
}

/// Spawns a background thread to process the image.
pub fn start_processing(
    config: ProcessingConfig,
    lang: LanguageManager,
    logger: LogOutput,
    processing: Arc<Mutex<bool>>,
    model_status: Arc<Mutex<ModelState>>,
    ctx: egui::Context,
) {
    if !config.input.exists() || !config.output.exists() {
        logger.send(lang.t("error_invalid_paths"));
        return;
    }

    *processing.lock().unwrap() = true;
    logger.send(lang.t("status_processing"));

    thread::spawn(move || {
        let res = run_processing_pipeline(&config, &lang, &logger, &model_status, &ctx);

        if let Err(e) = res {
            logger.send(format!("Error: {}", e));
        }
        
        *processing.lock().unwrap() = false;
        ctx.request_repaint();
    });
}

fn run_processing_pipeline(
    config: &ProcessingConfig,
    lang: &LanguageManager,
    logger: &LogOutput,
    model_status: &Arc<Mutex<ModelState>>,
    ctx: &egui::Context,
) -> Result<()> {
    let file_stem = config.input.file_stem().context("No filename")?.to_str().context("Decodification error")?;
    let base_name = if config.custom_filename.is_empty() {
        file_stem.to_string()
    } else {
        config.custom_filename.clone()
    };
    
    let paths = [
        ("alpha", config.output.join(format!("{}_alpha.png", base_name))),
        ("gray", config.output.join(format!("{}_gray.svg", base_name))),
        ("halftone", config.output.join(format!("{}_halftone.svg", base_name))),
        ("lineart", config.output.join(format!("{}_lineart.svg", base_name))),
        ("color_logo", config.output.join(format!("{}_logo.svg", base_name))),
        ("color_illus", config.output.join(format!("{}_illustration.svg", base_name))),
        ("thumb", config.output.join(format!("{}_thumb.png", base_name))),
    ];

    let any_conversion = config.gen_gray || config.gen_halftone || config.gen_lineart || config.gen_logo || config.gen_illus;
    let needs_alpha_gen = config.gen_alpha || any_conversion;

    let img = if needs_alpha_gen {
         logger.send(lang.t("status_gen_alpha"));
         ctx.request_repaint();
         let out_path = if config.gen_alpha { Some(paths[0].1.as_path()) } else { None };
         generators::generate_alpha_png(&config.input, out_path, lang, logger, model_status, config.selected_model)?
    } else {
         image::open(&config.input).context("Failed to open input image")?
    };

    if config.gen_gray {
        logger.send(lang.t("status_gen_gray"));
        ctx.request_repaint();
        generators::generate_grayscale_svg(&img, &paths[1].1, 8, lang, logger)?;
    }

    if config.gen_halftone {
        logger.send(lang.t("status_gen_halftone"));
        ctx.request_repaint();
        generators::generate_halftone_svg(&img, &paths[2].1, lang, logger)?;
    }

    if config.gen_lineart {
        logger.send(lang.t("status_gen_lineart"));
        ctx.request_repaint();
        generators::generate_lineart_svg(&img, &paths[3].1, lang, logger)?;
    }

    if config.gen_logo {
        logger.send(lang.t("status_gen_logo"));
        ctx.request_repaint();
        generators::generate_logo(&img, &paths[4].1, lang, logger)?;
    }

    if config.gen_illus {
        logger.send(lang.t("status_gen_illus"));
        ctx.request_repaint();
        generators::generate_illustration(&img, &paths[5].1, lang, logger)?;
    }

    if config.gen_thumbnail {
        logger.send(lang.t("status_gen_thumb"));
        ctx.request_repaint();
        generators::generate_thumbnail(&img, &paths[6].1, lang, logger)?;
    }

    logger.send(lang.t("status_done"));
    Ok(())
}
