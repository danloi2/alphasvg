//! Visual styling configuration for the GUI.

use eframe::egui;

/// Configures the visual styles for the application.
/// Sets up an elegant color palette with rounded corners and subtle shadows.
pub fn configure_styles(ctx: &egui::Context) {
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
