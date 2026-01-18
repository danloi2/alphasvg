//! GUI module for the Transparente application.
//! 
//! This module provides the graphical user interface using `eframe` and `egui`.

mod styles;
mod panels;
pub mod processing;

use eframe::egui;
use std::sync::{Arc, Mutex};
use anyhow::Result;
use std::path::PathBuf;

use crate::generators::{self, LogOutput, ModelState, ModelType};
use crate::lang::LanguageManager;

/// Launches the GUI application.
pub fn run_gui() -> Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_maximized(true)
            .with_inner_size([1024.0, 768.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Procesador Transparente - Rust",
        options,
        Box::new(|cc| {
            styles::configure_styles(&cc.egui_ctx);
            Ok(Box::new(MyApp::default()))
        }),
    ).map_err(|e| anyhow::anyhow!("Eframe error: {}", e))
}

/// Main application state.
struct MyApp {
    input_file: String,
    output_dir: String,
    
    // Log system
    log_sender: std::sync::mpsc::Sender<String>,
    log_receiver: Arc<Mutex<std::sync::mpsc::Receiver<String>>>,
    log_history: Arc<Mutex<Vec<String>>>,
    
    processing: Arc<Mutex<bool>>,
    model_status: Arc<Mutex<ModelState>>,
    selected_model: ModelType,
    
    // Checkbox states
    gen_alpha_transparency: bool,
    gen_gray: bool,
    gen_halftone: bool,
    gen_lineart: bool,
    gen_color_logo: bool,
    gen_color_illus: bool,
    gen_thumbnail: bool,

    output_filename: String,

    // I18n
    lang_manager: LanguageManager,
    show_about: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        let lang_manager = LanguageManager::default();
        let initial_status = lang_manager.t("status_ready");
        
        let (tx, rx) = std::sync::mpsc::channel();

        Self {
            input_file: String::new(),
            output_dir: String::new(),
            
            log_sender: tx,
            log_receiver: Arc::new(Mutex::new(rx)),
            log_history: Arc::new(Mutex::new(vec![initial_status])),
            
            processing: Arc::new(Mutex::new(false)),
            model_status: Arc::new(Mutex::new(ModelState::Unloaded)),
            selected_model: generators::ModelType::default(),
            
            gen_alpha_transparency: true,
            gen_gray: true,
            gen_halftone: true,
            gen_lineart: true,
            gen_color_logo: true,
            gen_color_illus: true,
            gen_thumbnail: true,

            output_filename: String::new(),

            lang_manager,
            show_about: false,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Consume logs from channel
        if let Ok(rx) = self.log_receiver.lock() {
            let mut history = self.log_history.lock().unwrap();
            while let Ok(msg) = rx.try_recv() {
                history.push(msg);
            }
        }

        // Menu Bar
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            let status = self.model_status.lock().unwrap().clone();
            panels::render_menu_bar(ui, ctx, &mut self.lang_manager, &mut self.show_about, &status);
        });

        // About Window
        if self.show_about {
            egui::Window::new(self.lang_manager.t("about_title"))
                .open(&mut self.show_about)
                .show(ctx, |ui| {
                    ui.label(self.lang_manager.t("about_text"));
                });
        }

        // Main content
        let frame = egui::Frame::central_panel(&ctx.style()).inner_margin(24.0);
        egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(egui::RichText::new(self.lang_manager.t("app_title")).heading().size(22.0).strong());
            });
            ui.add_space(20.0);

            // Calculate column widths
            let available = ui.available_width();
            let spacing = 12.0;
            let total_spacing = spacing * 2.0;
            let safe_width = (available - total_spacing - 48.0).max(600.0);
            let col1_width = safe_width * 0.34;
            let col2_width = safe_width * 0.34;
            let col3_width = safe_width * 0.32;
            let text_input_width = col1_width - 56.0;

            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = spacing;

                // Column 1: INPUT / OUTPUT
                ui.allocate_ui_with_layout(egui::vec2(col1_width, ui.available_height()), egui::Layout::top_down(egui::Align::Min), |ui| {
                    panels::render_io_column(
                        ui, col1_width, text_input_width, &self.lang_manager,
                        &mut self.input_file, &mut self.output_dir, &mut self.output_filename,
                    );
                });

                // Column 2: AI PROCESSING
                let mut should_start = false;
                ui.allocate_ui_with_layout(egui::vec2(col2_width, ui.available_height()), egui::Layout::top_down(egui::Align::Min), |ui| {
                    let is_processing = *self.processing.lock().unwrap();
                    should_start = panels::render_ai_column(
                        ui, col2_width, &self.lang_manager, &mut self.selected_model, is_processing,
                    );
                });
                if should_start {
                    self.start_processing(ctx.clone());
                }

                // Column 3: CONVERSION OPTIONS
                ui.allocate_ui_with_layout(egui::vec2(col3_width, ui.available_height()), egui::Layout::top_down(egui::Align::Min), |ui| {
                    panels::render_options_column(
                        ui, col3_width, &self.lang_manager,
                        &mut self.gen_alpha_transparency, &mut self.gen_thumbnail,
                        &mut self.gen_gray, &mut self.gen_halftone, &mut self.gen_lineart,
                        &mut self.gen_color_logo, &mut self.gen_color_illus,
                    );
                });
            });

            ui.add_space(15.0);
            
            // Terminal Log
            ui.vertical(|ui| {
                ui.set_width(available);
                let logs = self.log_history.lock().unwrap();
                panels::render_terminal_log(ui, &logs);
            });
        });
    }
}

impl MyApp {
    fn start_processing(&mut self, ctx: egui::Context) {
        let config = processing::ProcessingConfig {
            input: PathBuf::from(&self.input_file),
            output: PathBuf::from(&self.output_dir),
            custom_filename: self.output_filename.trim().to_string(),
            gen_alpha: self.gen_alpha_transparency,
            gen_gray: self.gen_gray,
            gen_halftone: self.gen_halftone,
            gen_lineart: self.gen_lineart,
            gen_logo: self.gen_color_logo,
            gen_illus: self.gen_color_illus,
            gen_thumbnail: self.gen_thumbnail,
            selected_model: self.selected_model,
        };

        processing::start_processing(
            config,
            self.lang_manager.clone(),
            LogOutput::Channel(self.log_sender.clone()),
            Arc::clone(&self.processing),
            Arc::clone(&self.model_status),
            ctx,
        );
    }
}
