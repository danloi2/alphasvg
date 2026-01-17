use eframe::egui;
use std::path::PathBuf;
use rfd::FileDialog;
use crate::generators;
use anyhow::{Result, Context};
use std::sync::{Arc, Mutex};
use std::thread;

pub fn run_gui() -> Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 450.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Procesador Transparente - Rust",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    ).map_err(|e| anyhow::anyhow!("Eframe error: {}", e))
}

struct MyApp {
    input_file: String,
    output_dir: String,
    status: Arc<Mutex<String>>,
    processing: Arc<Mutex<bool>>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            input_file: String::new(),
            output_dir: String::new(),
            status: Arc::new(Mutex::new("Ready. Please select a file.".to_string())),
            processing: Arc::new(Mutex::new(false)),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Procesador Transparente");
            });
            ui.add_space(20.0);

            ui.group(|ui| {
                ui.label("1. Imagen de entrada (.png, .jpg)");
                ui.horizontal(|ui| {
                    ui.text_edit_singleline(&mut self.input_file);
                    if ui.button("Buscar Archivo").clicked() {
                        if let Some(path) = FileDialog::new()
                            .add_filter("Images", &["png", "jpg", "jpeg"])
                            .pick_file() 
                        {
                            self.input_file = path.display().to_string();
                            if self.output_dir.is_empty() {
                                if let Some(parent) = path.parent() {
                                    self.output_dir = parent.display().to_string();
                                }
                            }
                        }
                    }
                });
            });

            ui.add_space(10.0);

            ui.group(|ui| {
                ui.label("2. Carpeta de destino");
                ui.horizontal(|ui| {
                    ui.text_edit_singleline(&mut self.output_dir);
                    if ui.button("Elegir Carpeta").clicked() {
                        if let Some(path) = FileDialog::new().pick_folder() {
                            self.output_dir = path.display().to_string();
                        }
                    }
                });
            });

            ui.add_space(30.0);

            let is_processing = *self.processing.lock().unwrap();
            ui.vertical_centered(|ui| {
                if ui.add_enabled(!is_processing, egui::Button::new("INICIAR CONVERSION")).clicked() {
                    self.start_processing(ctx.clone());
                }
            });

            ui.add_space(20.0);
            let status_text = self.status.lock().unwrap().clone();
            ui.label(egui::RichText::new(status_text).italics());
        });
    }
}

impl MyApp {
    fn start_processing(&mut self, ctx: egui::Context) {
        let input = PathBuf::from(&self.input_file);
        let output = PathBuf::from(&self.output_dir);

        if !input.exists() || !output.exists() {
            *self.status.lock().unwrap() = "Error: Invalid paths.".to_string();
            return;
        }

        let status = Arc::clone(&self.status);
        let processing = Arc::clone(&self.processing);
        *processing.lock().unwrap() = true;
        *status.lock().unwrap() = "Iniciando proceso...".to_string();

        thread::spawn(move || {
            let res = (|| -> Result<()> {
                let file_name = input.file_stem().context("No filename")?.to_str().context("Decodification error")?;
                let base_name = format!("{}_alpha", file_name);
                
                let paths = [
                    ("alpha", output.join(format!("{}.png", base_name))),
                    ("gray", output.join(format!("{}_gray.svg", base_name))),
                    ("halftone", output.join(format!("{}_halftone.svg", base_name))),
                    ("lineart", output.join(format!("{}_lineart.svg", base_name))),
                    ("color_logo", output.join(format!("{}_color_logo.svg", base_name))),
                    ("color_illus", output.join(format!("{}_color_illus.svg", base_name))),
                    ("thumb", output.join(format!("{}_thumb.png", base_name))),
                ];

                *status.lock().unwrap() = "Generando Alpha PNG (IA)...".to_string();
                ctx.request_repaint();
                let img = generators::generate_alpha_png(&input, &paths[0].1)?;

                *status.lock().unwrap() = "Generando SVG Grayscale...".to_string();
                ctx.request_repaint();
                generators::generate_grayscale_svg(&img, &paths[1].1, 8)?;

                *status.lock().unwrap() = "Generando SVG Halftone...".to_string();
                ctx.request_repaint();
                generators::generate_halftone_svg(&img, &paths[2].1)?;

                *status.lock().unwrap() = "Generando SVG Lineart...".to_string();
                ctx.request_repaint();
                generators::generate_lineart_svg(&img, &paths[3].1)?;

                *status.lock().unwrap() = "Generando SVG Color (Logo)...".to_string();
                ctx.request_repaint();
                generators::generate_color_svg(&img, &paths[4].1, 16)?;

                *status.lock().unwrap() = "Generando SVG Color (Ilustración)...".to_string();
                ctx.request_repaint();
                generators::generate_color_svg(&img, &paths[5].1, 48)?;

                *status.lock().unwrap() = "Generando Miniatura...".to_string();
                ctx.request_repaint();
                generators::generate_thumbnail(&img, &paths[6].1)?;

                *status.lock().unwrap() = "¡Completado!".to_string();
                Ok(())
            })();

            if let Err(e) = res {
                *status.lock().unwrap() = format!("Error: {}", e);
            }
            *processing.lock().unwrap() = false;
            ctx.request_repaint();
        });
    }
}
