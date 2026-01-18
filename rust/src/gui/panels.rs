//! UI panel rendering functions.

use eframe::egui;
use rfd::FileDialog;

use crate::generators::{self, ModelState, ModelType};
use crate::lang::LanguageManager;
use super::processing;

/// Renders the menu bar with file, preferences, help menus and model status indicator.
pub fn render_menu_bar(
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    lang: &mut LanguageManager,
    show_about: &mut bool,
    model_status: &ModelState,
) {
    egui::MenuBar::new().ui(ui, |ui: &mut egui::Ui| {
        ui.menu_button(lang.t("menu_file"), |ui: &mut egui::Ui| {
            if ui.button(lang.t("menu_quit")).clicked() {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        });

        ui.menu_button(lang.t("menu_prefs"), |ui: &mut egui::Ui| {
            ui.menu_button(lang.t("menu_lang"), |ui: &mut egui::Ui| {
                if ui.button("Español").clicked() {
                    lang.load_language("es");
                    ui.close();
                }
                if ui.button("English").clicked() {
                    lang.load_language("en");
                    ui.close();
                }
                if ui.button("Euskara").clicked() {
                    lang.load_language("eu");
                    ui.close();
                }
                if ui.button("Latina").clicked() {
                    lang.load_language("la");
                    ui.close();
                }
            });
        });

        ui.menu_button(lang.t("menu_help"), |ui: &mut egui::Ui| {
            if ui.button(lang.t("menu_about")).clicked() {
                *show_about = true;
                ui.close();
            }
        });

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            render_model_status_indicator(ui, ctx, model_status);
        });
    });
}

fn render_model_status_indicator(ui: &mut egui::Ui, ctx: &egui::Context, status: &ModelState) {
    let color = match status {
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
    }

    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("AI MODEL ").size(18.0).strong());
        match status {
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
}

/// Renders the input/output column (column 1).
pub fn render_io_column(
    ui: &mut egui::Ui,
    col_width: f32,
    text_input_width: f32,
    lang: &LanguageManager,
    input_file: &mut String,
    output_dir: &mut String,
    output_filename: &mut String,
) {
    ui.set_max_width(col_width);
    ui.spacing_mut().item_spacing.y = 4.0;
    ui.vertical_centered(|ui| {
        ui.label(egui::RichText::new(lang.t("hdr_io")).strong().size(18.0).color(egui::Color32::from_rgb(100, 100, 255)));
    });

    // Input file group
    ui.group(|ui| {
        ui.set_width(col_width - 16.0);
        ui.add(egui::Label::new(
            egui::RichText::new(format!("1. {}", lang.t("input_group")))
                .size(18.0).strong()
        ).wrap_mode(egui::TextWrapMode::Wrap));
        ui.horizontal(|ui| {
            ui.add(egui::TextEdit::singleline(input_file).desired_width(text_input_width));
            if ui.button("📂").on_hover_text(lang.t("btn_search_file")).clicked() {
                if let Some(path) = FileDialog::new()
                    .add_filter("Images", &["png", "jpg", "jpeg"])
                    .pick_file() 
                {
                    *input_file = path.display().to_string();
                    if output_dir.is_empty() {
                        if let Some(parent) = path.parent() {
                            *output_dir = parent.display().to_string();
                        }
                    }
                }
            }
        });
    });

    // Output directory group
    ui.group(|ui| {
        ui.set_width(col_width - 16.0);
        ui.add(egui::Label::new(
            egui::RichText::new(format!("2. {}", lang.t("output_group")))
                .size(18.0).strong()
        ).wrap_mode(egui::TextWrapMode::Wrap));
        ui.horizontal(|ui| {
            ui.add(egui::TextEdit::singleline(output_dir).desired_width(text_input_width));
            if ui.button("📁").on_hover_text(lang.t("btn_choose_folder")).clicked() {
                if let Some(path) = FileDialog::new().pick_folder() {
                    *output_dir = path.display().to_string();
                }
            }
        });
    });

    // Output filename group
    ui.group(|ui| {
        ui.set_width(col_width - 16.0);
        ui.add(egui::Label::new(
            egui::RichText::new(format!("3. {}", lang.t("lbl_output_filename")))
                .size(18.0).strong()
        ).wrap_mode(egui::TextWrapMode::Wrap));
        ui.horizontal(|ui| {
            ui.add(egui::TextEdit::singleline(output_filename)
                .hint_text("Default")
                .desired_width(text_input_width));
            ui.add_space(36.0);
        });
    });
}

/// Renders the AI processing column (column 2).
/// Returns true if the start button was clicked.
pub fn render_ai_column(
    ui: &mut egui::Ui,
    col_width: f32,
    lang: &LanguageManager,
    selected_model: &mut ModelType,
    is_processing: bool,
) -> bool {
    let mut start_clicked = false;
    
    ui.set_max_width(col_width);
    ui.spacing_mut().item_spacing.y = 4.0;
    ui.vertical_centered(|ui| {
        ui.label(egui::RichText::new(lang.t("hdr_ai")).strong().size(18.0).color(egui::Color32::from_rgb(100, 140, 100)));
    });

    ui.group(|ui| {
        ui.set_width(ui.available_width());
        ui.add(egui::Label::new(
            egui::RichText::new(format!("4. {}", lang.t("label_ai_model")))
                .size(18.0).strong()
        ).wrap_mode(egui::TextWrapMode::Wrap));
        
        egui::ComboBox::from_id_salt("model_select")
            .selected_text(format!("{:?}", selected_model))
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
                    ui.selectable_value(selected_model, model, format!("{:?}", model));
                }
            });
        
        ui.add_space(2.0);
        ui.separator();
        ui.add(egui::Label::new(egui::RichText::new("Description:").strong().size(14.0)));
        ui.add(egui::Label::new(egui::RichText::new(processing::get_model_description_localized(lang, *selected_model))
            .italics()
            .size(14.0)
            .color(egui::Color32::DARK_GRAY))
            .wrap_mode(egui::TextWrapMode::Wrap));
    });

    ui.add_space(5.0);

    ui.vertical_centered(|ui| {
        let btn = egui::Button::new(egui::RichText::new(lang.t("btn_start")).strong())
            .min_size(egui::vec2(100.0, 28.0))
            .fill(egui::Color32::from_rgb(60, 120, 255));
        if ui.add_enabled(!is_processing, btn).clicked() {
            start_clicked = true;
        }
    });
    
    start_clicked
}

/// Renders the conversion options column (column 3).
pub fn render_options_column(
    ui: &mut egui::Ui,
    col_width: f32,
    lang: &LanguageManager,
    gen_alpha: &mut bool,
    gen_thumbnail: &mut bool,
    gen_gray: &mut bool,
    gen_halftone: &mut bool,
    gen_lineart: &mut bool,
    gen_color_logo: &mut bool,
    gen_color_illus: &mut bool,
) {
    ui.set_max_width(col_width);
    ui.spacing_mut().item_spacing.y = 4.0;
    ui.vertical_centered(|ui| {
        ui.label(egui::RichText::new(lang.t("hdr_options")).strong().size(18.0).color(egui::Color32::from_rgb(140, 100, 100)));
    });

    ui.group(|ui| {
        ui.set_width(ui.available_width());
        ui.add(egui::Label::new(
            egui::RichText::new(format!("5. {}", lang.t("options_group")))
                .size(18.0).strong()
        ).wrap_mode(egui::TextWrapMode::Wrap));
        
        // PNG Subcategory
        ui.add(egui::Label::new(egui::RichText::new("PNG").strong().size(14.0)));
        ui.indent("png_indent", |ui| {
            ui.spacing_mut().item_spacing.y = 1.0;
            ui.checkbox(gen_alpha, egui::RichText::new(lang.t("chk_transparent")).size(14.0));
            ui.checkbox(gen_thumbnail, egui::RichText::new(lang.t("chk_thumbnail")).size(14.0));
        });

        ui.add_space(2.0);

        // SVG Group
        ui.add(egui::Label::new(egui::RichText::new(lang.t("group_svg")).strong().size(14.0)));
        ui.indent("svg_indent", |ui| {
            ui.spacing_mut().item_spacing.y = 1.0;
            // Black and White subgroup
            ui.add(egui::Label::new(egui::RichText::new(lang.t("subgroup_bw")).size(14.0)));
            ui.checkbox(gen_gray, egui::RichText::new(lang.t("chk_grayscale")).size(14.0));
            ui.checkbox(gen_halftone, egui::RichText::new(lang.t("chk_halftone")).size(14.0));
            ui.checkbox(gen_lineart, egui::RichText::new(lang.t("chk_lineart")).size(14.0));
            // Color subgroup
            ui.add(egui::Label::new(egui::RichText::new(lang.t("subgroup_color")).size(14.0)));
            ui.checkbox(gen_color_logo, egui::RichText::new(lang.t("chk_logo")).size(14.0));
            ui.checkbox(gen_color_illus, egui::RichText::new(lang.t("chk_illus")).size(14.0));
        });
    });
}

/// Renders the terminal log panel at the bottom.
pub fn render_terminal_log(ui: &mut egui::Ui, logs: &[String]) {
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
                    
                    for log in logs.iter() {
                        ui.label(egui::RichText::new(log).color(egui::Color32::from_rgb(0, 255, 0)).monospace().size(18.0));
                    }
                });
            });
    });
}
