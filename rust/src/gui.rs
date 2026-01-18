use eframe::egui;
use std::path::PathBuf;
use rfd::FileDialog;
use crate::generators::{self, LogOutput, ModelState};
use crate::lang::LanguageManager;
use anyhow::{Result, Context};
use std::sync::{Arc, Mutex};
use std::thread;

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
            configure_styles(&cc.egui_ctx);
            Ok(Box::new(MyApp::default()))
        }),
    ).map_err(|e| anyhow::anyhow!("Eframe error: {}", e))
}

fn configure_styles(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::light();
    
    // Elegant color palette
    let accent_color = egui::Color32::from_rgb(79, 70, 229); // Indigo
    let subtle_bg = egui::Color32::from_rgb(248, 250, 252);
    let border_color = egui::Color32::from_rgb(226, 232, 240);
    
    // Window styling
    visuals.window_corner_radius = egui::CornerRadius::same(12);
    visuals.window_fill = egui::Color32::WHITE;
    visuals.window_stroke = egui::Stroke::new(1.0, border_color);
    visuals.window_shadow = egui::Shadow {
        offset: [0, 4],
        blur: 16,
        spread: 0,
        color: egui::Color32::from_black_alpha(20),
    };
    
    // Widget styling - rounded and elegant
    visuals.widgets.noninteractive.corner_radius = egui::CornerRadius::same(8);
    visuals.widgets.noninteractive.bg_fill = subtle_bg;
    visuals.widgets.noninteractive.bg_stroke = egui::Stroke::new(1.0, border_color);
    
    visuals.widgets.inactive.corner_radius = egui::CornerRadius::same(8);
    visuals.widgets.inactive.bg_fill = egui::Color32::WHITE;
    visuals.widgets.inactive.bg_stroke = egui::Stroke::new(1.0, border_color);
    
    visuals.widgets.hovered.corner_radius = egui::CornerRadius::same(8);
    visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(238, 242, 255);
    visuals.widgets.hovered.bg_stroke = egui::Stroke::new(1.5, accent_color);
    
    visuals.widgets.active.corner_radius = egui::CornerRadius::same(8);
    visuals.widgets.active.bg_fill = egui::Color32::from_rgb(224, 231, 255);
    visuals.widgets.active.bg_stroke = egui::Stroke::new(2.0, accent_color);
    
    visuals.widgets.open.corner_radius = egui::CornerRadius::same(8);
    visuals.widgets.open.bg_fill = egui::Color32::WHITE;
    
    // Selection highlight
    visuals.selection.bg_fill = egui::Color32::from_rgb(199, 210, 254);
    visuals.selection.stroke = egui::Stroke::new(1.5, accent_color);
    
    // Text cursor
    visuals.text_cursor.stroke = egui::Stroke::new(2.0, accent_color);
    
    ctx.set_visuals(visuals);

    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing = egui::vec2(12.0, 12.0);
    style.spacing.button_padding = egui::vec2(24.0, 10.0);
    style.spacing.window_margin = egui::Margin::same(16);
    ctx.set_style(style);
}

struct MyApp {
    input_file: String,
    output_dir: String,
    
    // Log system
    log_sender: std::sync::mpsc::Sender<String>,
    log_receiver: Arc<Mutex<std::sync::mpsc::Receiver<String>>>,
    log_history: Arc<Mutex<Vec<String>>>,
    
    processing: Arc<Mutex<bool>>,
    model_status: Arc<Mutex<ModelState>>,
    selected_model: generators::ModelType,
    
    // Checkbox states
    gen_alpha_transparency: bool,
    gen_gray: bool,
    gen_halftone: bool,
    gen_lineart: bool,
    gen_color_logo: bool,
    gen_color_illus: bool,
    gen_thumbnail: bool,

    output_filename: String, // Custom base name

    // I18n
    lang_manager: LanguageManager,
    show_about: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        let lang_manager = LanguageManager::default();
        let initial_status = lang_manager.t("status_ready");
        
        // Create channel
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
            egui::MenuBar::new().ui(ui, |ui: &mut egui::Ui| {
    ui.menu_button(self.lang_manager.t("menu_file"), |ui: &mut egui::Ui| {
        if ui.button(self.lang_manager.t("menu_quit")).clicked() {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    });

    ui.menu_button(self.lang_manager.t("menu_prefs"), |ui: &mut egui::Ui| {
        ui.menu_button(self.lang_manager.t("menu_lang"), |ui: &mut egui::Ui| {
            if ui.button("Español").clicked() {
                self.lang_manager.load_language("es");
                ui.close();
            }
            if ui.button("English").clicked() {
                self.lang_manager.load_language("en");
                ui.close();
            }
            if ui.button("Euskara").clicked() {
                self.lang_manager.load_language("eu");
                ui.close();
            }
            if ui.button("Latina").clicked() {
                self.lang_manager.load_language("la");
                ui.close();
            }
        });
    });

    ui.menu_button(self.lang_manager.t("menu_help"), |ui: &mut egui::Ui| {
        if ui.button(self.lang_manager.t("menu_about")).clicked() {
            self.show_about = true;
            ui.close();
        }
    });

    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        let status = self.model_status.lock().unwrap().clone();
                let color = match &status {
                    ModelState::Unloaded => egui::Color32::RED,
                    ModelState::Loading => {
                        let time = ui.input(|i| i.time);
                        let alpha = ((time * 6.0).sin() * 0.5 + 0.5) as f32;
                        egui::Color32::from_rgba_unmultiplied(255, 140, 0, (alpha * 255.0) as u8)
                    },
                    ModelState::Ready(_) => egui::Color32::from_rgb(0, 255, 0),
                };
                
                if matches!(status, ModelState::Loading) {
                    ctx.request_repaint();
                }   ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("AI MODEL ").size(18.0).strong());
                    match &status {
                        ModelState::Ready(name) => {
                            let name_low = name.to_lowercase();
                            if name_low.contains("u2net") {
                                ui.hyperlink_to(
                                    egui::RichText::new(name).size(18.0).strong(),
                                    "https://github.com/xuebinqin/U-2-Net",
                                );
                            } else if name_low.contains("isnet") || name_low.contains("dis") {
                                ui.hyperlink_to(
                                    egui::RichText::new(name).size(18.0).strong(),
                                    "https://github.com/xuebinqin/DIS",
                                );
                            } else {
                                ui.label(egui::RichText::new(name).size(18.0).strong());
                            }
                        }
                        ModelState::Loading => {
                            ui.label(egui::RichText::new("Loading...").size(18.0).strong());
                        }
                        ModelState::Unloaded => {
                            ui.label(egui::RichText::new("Not Loaded").size(18.0).strong());
                        }
                    }
                    let (rect, _) = ui.allocate_exact_size(egui::vec2(20.0, 20.0), egui::Sense::hover());
                    ui.painter().circle_filled(rect.center(), 10.0, color);
                        let tooltip = match status {
                            ModelState::Unloaded => "Model not loaded",
                            ModelState::Loading => "Loading/Downloading model...",
                            ModelState::Ready(_) => "Model ready in memory",
                        };
                        ui.allocate_rect(rect, egui::Sense::hover()).on_hover_text(tooltip);
        });
    });
});
        });

        // About Window
        if self.show_about {
            egui::Window::new(self.lang_manager.t("about_title"))
                .open(&mut self.show_about)
                .show(ctx, |ui| {
                    ui.label(self.lang_manager.t("about_text"));
                });
        }

        let frame = egui::Frame::central_panel(&ctx.style()).inner_margin(24.0);
        egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(egui::RichText::new(self.lang_manager.t("app_title")).heading().size(22.0).strong());
            });
            ui.add_space(20.0);

            // Calculate column widths - leave extra margin to prevent overflow
            let available = ui.available_width();
            let spacing = 12.0;
            let total_spacing = spacing * 2.0;
            let safe_width = (available - total_spacing - 48.0).max(600.0); // 48px safety margin
            let col1_width = safe_width * 0.34;
            let col2_width = safe_width * 0.34;
            let col3_width = safe_width * 0.32;
            
            // Fixed width for text inputs in column 1 (accounting for button space)
            let text_input_width = col1_width - 56.0; // 36px button + 20px padding

            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = spacing;

                // Column 1: INPUT / OUTPUT (1/3)
                ui.allocate_ui_with_layout(egui::vec2(col1_width, ui.available_height()), egui::Layout::top_down(egui::Align::Min), |ui| {
                    ui.set_max_width(col1_width);
                    ui.spacing_mut().item_spacing.y = 4.0;
                    ui.vertical_centered(|ui| {
                        ui.label(egui::RichText::new(self.lang_manager.t("hdr_io")).strong().size(18.0).color(egui::Color32::from_rgb(100, 100, 255)));
                    });

                    ui.group(|ui| {
                        ui.set_width(col1_width - 16.0);
                        ui.add(egui::Label::new(
                            egui::RichText::new(format!("1. {}", self.lang_manager.t("input_group")))
                                .size(18.0).strong()
                        ).wrap_mode(egui::TextWrapMode::Wrap));
                        ui.horizontal(|ui| {
                            ui.add(egui::TextEdit::singleline(&mut self.input_file).desired_width(text_input_width));
                            if ui.button("📂").on_hover_text(self.lang_manager.t("btn_search_file")).clicked() {
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

                    ui.group(|ui| {
                        ui.set_width(col1_width - 16.0);
                        ui.add(egui::Label::new(
                            egui::RichText::new(format!("2. {}", self.lang_manager.t("output_group")))
                                .size(18.0).strong()
                        ).wrap_mode(egui::TextWrapMode::Wrap));
                        ui.horizontal(|ui| {
                            ui.add(egui::TextEdit::singleline(&mut self.output_dir).desired_width(text_input_width));
                            if ui.button("📁").on_hover_text(self.lang_manager.t("btn_choose_folder")).clicked() {
                                if let Some(path) = FileDialog::new().pick_folder() {
                                    self.output_dir = path.display().to_string();
                                }
                            }
                        });
                    });

                    ui.group(|ui| {
                        ui.set_width(col1_width - 16.0);
                        ui.add(egui::Label::new(
                            egui::RichText::new(format!("3. {}", self.lang_manager.t("lbl_output_filename")))
                                .size(18.0).strong()
                        ).wrap_mode(egui::TextWrapMode::Wrap));
                        ui.horizontal(|ui| {
                            ui.add(egui::TextEdit::singleline(&mut self.output_filename)
                                .hint_text("Default")
                                .desired_width(text_input_width));
                            // Empty space to match other rows
                            ui.add_space(36.0);
                        });
                    });
                });

                // Column 2: AI PROCESSING (30%)
                ui.allocate_ui_with_layout(egui::vec2(col2_width, ui.available_height()), egui::Layout::top_down(egui::Align::Min), |ui| {
                    ui.set_max_width(col2_width);
                    ui.spacing_mut().item_spacing.y = 4.0;
                    ui.vertical_centered(|ui| {
                        ui.label(egui::RichText::new(self.lang_manager.t("hdr_ai")).strong().size(18.0).color(egui::Color32::from_rgb(100, 140, 100)));
                    });

                    ui.group(|ui| {
                        ui.set_width(ui.available_width());
                        ui.add(egui::Label::new(
                            egui::RichText::new(format!("4. {}", self.lang_manager.t("label_ai_model")))
                                .size(18.0).strong()
                        ).wrap_mode(egui::TextWrapMode::Wrap));
                        
                        egui::ComboBox::from_id_salt("model_select")
                            .selected_text(format!("{:?}", self.selected_model))
                            .width(ui.available_width() - 10.0)
                            .show_ui(ui, |ui| {
                                use generators::ModelType::*;
                                let models = [
                                    U2Net, U2NetP, U2NetHumanSeg, U2NetClothSeg, Silueta,
                                    IsNetGeneralUse, IsNetAnime, Sam, BiRefNetGeneral,
                                    BiRefNetGeneralLite, BiRefNetPortrait, BiRefNetDis,
                                    BiRefNetHrsod, BiRefNetCod, BiRefNetMassive, BriaRmbg
                                ];
                                for model in models {
                                    ui.selectable_value(&mut self.selected_model, model, format!("{:?}", model));
                                }
                            });
                        
                        ui.add_space(2.0);
                        ui.separator();
                        ui.add(egui::Label::new(egui::RichText::new("Description:").strong().size(14.0)));
                        ui.add(egui::Label::new(egui::RichText::new(self.get_model_description_localized(self.selected_model))
                            .italics()
                            .size(14.0)
                            .color(egui::Color32::DARK_GRAY))
                            .wrap_mode(egui::TextWrapMode::Wrap));
                    });

                    ui.add_space(5.0);

                    let is_processing = *self.processing.lock().unwrap();
                    ui.vertical_centered(|ui| {
                        let btn = egui::Button::new(egui::RichText::new(self.lang_manager.t("btn_start")).strong())
                            .min_size(egui::vec2(100.0, 28.0))
                            .fill(egui::Color32::from_rgb(60, 120, 255));
                        if ui.add_enabled(!is_processing, btn).clicked() {
                            self.start_processing(ctx.clone());
                        }
                    });
                });

                // Column 3: CONVERSION OPTIONS (33%)
                ui.allocate_ui_with_layout(egui::vec2(col3_width, ui.available_height()), egui::Layout::top_down(egui::Align::Min), |ui| {
                    ui.set_max_width(col3_width);
                    ui.spacing_mut().item_spacing.y = 4.0;
                    ui.vertical_centered(|ui| {
                        ui.label(egui::RichText::new(self.lang_manager.t("hdr_options")).strong().size(18.0).color(egui::Color32::from_rgb(140, 100, 100)));
                    });

                    ui.group(|ui| {
                        ui.set_width(ui.available_width());
                        ui.add(egui::Label::new(
                            egui::RichText::new(format!("5. {}", self.lang_manager.t("options_group")))
                                .size(18.0).strong()
                        ).wrap_mode(egui::TextWrapMode::Wrap));
                        
                        // PNG Subcategory
                        ui.add(egui::Label::new(egui::RichText::new("PNG").strong().size(14.0)));
                        ui.indent("png_indent", |ui| {
                            ui.spacing_mut().item_spacing.y = 1.0;
                            ui.checkbox(&mut self.gen_alpha_transparency, egui::RichText::new(self.lang_manager.t("chk_transparent")).size(14.0));
                            ui.checkbox(&mut self.gen_thumbnail, egui::RichText::new(self.lang_manager.t("chk_thumbnail")).size(14.0));
                        });

                        ui.add_space(2.0);

                        // Group SVG
                        ui.add(egui::Label::new(egui::RichText::new(self.lang_manager.t("group_svg")).strong().size(14.0)));
                        ui.indent("svg_indent", |ui| {
                            ui.spacing_mut().item_spacing.y = 1.0;
                            // Subgroup Black and White
                            ui.add(egui::Label::new(egui::RichText::new(self.lang_manager.t("subgroup_bw")).size(14.0)));
                            ui.checkbox(&mut self.gen_gray, egui::RichText::new(self.lang_manager.t("chk_grayscale")).size(14.0));
                            ui.checkbox(&mut self.gen_halftone, egui::RichText::new(self.lang_manager.t("chk_halftone")).size(14.0));
                            ui.checkbox(&mut self.gen_lineart, egui::RichText::new(self.lang_manager.t("chk_lineart")).size(14.0));
                            // Subgroup Color
                            ui.add(egui::Label::new(egui::RichText::new(self.lang_manager.t("subgroup_color")).size(14.0)));
                            ui.checkbox(&mut self.gen_color_logo, egui::RichText::new(self.lang_manager.t("chk_logo")).size(14.0));
                            ui.checkbox(&mut self.gen_color_illus, egui::RichText::new(self.lang_manager.t("chk_illus")).size(14.0));
                        });
                    });
                });
            });

            ui.add_space(15.0);
            
            // --- Bottom: Terminal Log (Full Width) ---
            ui.vertical(|ui| {
                ui.set_width(available);
                ui.group(|ui| {
                    ui.set_width(ui.available_width());
                    ui.label(egui::RichText::new("Terminal Log").strong());
                    
                    egui::ScrollArea::vertical()
                        .id_salt("log_scroll")
                        .stick_to_bottom(true)
                        .max_height(200.0)
                        .show(ui, |ui| {
                            ui.set_min_width(ui.available_width());
                            ui.set_min_height(100.0);
                            
                            egui::Frame::canvas(ui.style()).fill(egui::Color32::from_black_alpha(240)).show(ui, |ui| {
                                ui.set_min_width(ui.available_width());
                                ui.set_min_height(100.0);
                                
                                let logs = self.log_history.lock().unwrap();
                                for log in logs.iter() {
                                    ui.label(egui::RichText::new(log).color(egui::Color32::from_rgb(0, 255, 0)).monospace().size(18.0));
                                }
                            });
                        });
                });
            });
        });
    }
}

impl MyApp {
    fn get_model_description_localized(&self, model: generators::ModelType) -> String {
        use generators::ModelType::*;
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
            BiRefNetGeneralLite => "desc_birefnet_general", // Reuse general for lite if same
            BiRefNetPortrait => "desc_birefnet_portrait",
            BiRefNetDis => "desc_birefnet_dis",
            BiRefNetHrsod => "desc_birefnet_hrsod",
            BiRefNetCod => "desc_birefnet_cod",
            BiRefNetMassive => "desc_birefnet_massive",
            BriaRmbg => "desc_briarmbg",
        };
        self.lang_manager.t(key)
    }

    fn start_processing(&mut self, ctx: egui::Context) {
        let input = PathBuf::from(&self.input_file);
        let output = PathBuf::from(&self.output_dir);
        let lang = self.lang_manager.clone(); // Clone for thread
        
        let logger_sender = self.log_sender.clone();
        let logger = LogOutput::Channel(logger_sender);

        if !input.exists() || !output.exists() {
            logger.send(lang.t("error_invalid_paths"));
            return;
        }

        // Capture flags
        let gen_alpha = self.gen_alpha_transparency;
        let gen_gray = self.gen_gray;
        let gen_halftone = self.gen_halftone;
        let gen_lineart = self.gen_lineart;
        let gen_logo = self.gen_color_logo;
        let gen_illus = self.gen_color_illus;
        let gen_thumbnail = self.gen_thumbnail;
        let selected_model = self.selected_model;

        let processing = Arc::clone(&self.processing);
        let model_status = Arc::clone(&self.model_status);
        *processing.lock().unwrap() = true;
        logger.send(lang.t("status_processing"));

        let custom_filename = self.output_filename.trim().to_string();

        thread::spawn(move || {
            let res = (|| -> Result<()> {
                let file_stem = input.file_stem().context("No filename")?.to_str().context("Decodification error")?;
                let base_name = if custom_filename.is_empty() {
                    file_stem.to_string()
                } else {
                    custom_filename
                };
                
                let paths = [
                    ("alpha", output.join(format!("{}_alpha.png", base_name))),
                    ("gray", output.join(format!("{}_gray.svg", base_name))),
                    ("halftone", output.join(format!("{}_halftone.svg", base_name))),
                    ("lineart", output.join(format!("{}_lineart.svg", base_name))),
                    ("color_logo", output.join(format!("{}_logo.svg", base_name))),
                    ("color_illus", output.join(format!("{}_illustration.svg", base_name))),
                    ("thumb", output.join(format!("{}_thumb.png", base_name))),
                ];

                let any_conversion = gen_gray || gen_halftone || gen_lineart || gen_logo || gen_illus;
                let needs_alpha_gen = gen_alpha || any_conversion;

                let img = if needs_alpha_gen {
                     logger.send(lang.t("status_gen_alpha"));
                     ctx.request_repaint();
                     let out_path = if gen_alpha { Some(paths[0].1.as_path()) } else { None };
                     generators::generate_alpha_png(&input, out_path, &lang, &logger, &model_status, selected_model)?
                } else {
                     image::open(&input).context("Failed to open input image")?
                };

                if gen_gray {
                    logger.send(lang.t("status_gen_gray"));
                    ctx.request_repaint();
                    generators::generate_grayscale_svg(&img, &paths[1].1, 8, &lang, &logger)?;
                }

                if gen_halftone {
                    logger.send(lang.t("status_gen_halftone"));
                    ctx.request_repaint();
                    generators::generate_halftone_svg(&img, &paths[2].1, &lang, &logger)?;
                }

                if gen_lineart {
                    logger.send(lang.t("status_gen_lineart"));
                    ctx.request_repaint();
                    generators::generate_lineart_svg(&img, &paths[3].1, &lang, &logger)?;
                }

                if gen_logo {
                    logger.send(lang.t("status_gen_logo"));
                    ctx.request_repaint();
                    generators::generate_logo(&img, &paths[4].1, &lang, &logger)?;
                }

                if gen_illus {
                    logger.send(lang.t("status_gen_illus"));
                    ctx.request_repaint();
                    generators::generate_illustration(&img, &paths[5].1, &lang, &logger)?;
                }

                if gen_thumbnail {
                    logger.send(lang.t("status_gen_thumb"));
                    ctx.request_repaint();
                    generators::generate_thumbnail(&img, &paths[6].1, &lang, &logger)?;
                }

                logger.send(lang.t("status_done"));
                Ok(())
            })();

            if let Err(e) = res {
                logger.send(format!("Error: {}", e));
            }
            
            *processing.lock().unwrap() = false;
            ctx.request_repaint();
        });
    }
}
