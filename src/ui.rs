use crate::keybindings::{get_keybindings, Category, Keybinding};
use eframe::egui;
use egui::{
    Color32, Frame, Margin, RichText, Rounding, ScrollArea, Stroke, Ui,
    Vec2,
};
use std::collections::HashMap;

// Material 3 Dark Theme Colors
struct MaterialColors;

impl MaterialColors {
    const SURFACE: Color32 = Color32::from_rgb(20, 18, 24);
    const SURFACE_CONTAINER: Color32 = Color32::from_rgb(33, 31, 38);
    const SURFACE_CONTAINER_HIGH: Color32 = Color32::from_rgb(43, 41, 48);
    const ON_SURFACE: Color32 = Color32::from_rgb(230, 224, 233);
    const ON_SURFACE_VARIANT: Color32 = Color32::from_rgb(202, 196, 208);
    const OUTLINE: Color32 = Color32::from_rgb(147, 143, 153);
    const OUTLINE_VARIANT: Color32 = Color32::from_rgb(73, 69, 79);
    const PRIMARY: Color32 = Color32::from_rgb(208, 188, 255);
    // const ON_PRIMARY: Color32 = Color32::from_rgb(56, 30, 114);
    // const PRIMARY_CONTAINER: Color32 = Color32::from_rgb(79, 55, 139);
    // const ON_PRIMARY_CONTAINER: Color32 = Color32::from_rgb(234, 221, 255);
    // const SECONDARY_CONTAINER: Color32 = Color32::from_rgb(74, 68, 88);
    // const ON_SECONDARY_CONTAINER: Color32 = Color32::from_rgb(232, 222, 248);
    // const TERTIARY_CONTAINER: Color32 = Color32::from_rgb(99, 59, 72);
    // const ON_TERTIARY_CONTAINER: Color32 = Color32::from_rgb(255, 216, 228);
}

pub struct CheatsheetApp {
    keybindings: HashMap<Category, Vec<Keybinding>>,
    search_text: String,
    filtered_keybindings: HashMap<Category, Vec<Keybinding>>,
}

impl CheatsheetApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize fonts
        let fonts = egui::FontDefinitions::default();
        // You could load custom fonts here (e.g. Roboto)
        // For now we stick to default but adjust sizes
        cc.egui_ctx.set_fonts(fonts);

        // Configure style
        let mut style = (*cc.egui_ctx.style()).clone();
        style.visuals.dark_mode = true;
        style.visuals.window_fill = MaterialColors::SURFACE;
        style.visuals.panel_fill = MaterialColors::SURFACE;
        style.visuals.widgets.noninteractive.bg_fill = MaterialColors::SURFACE;
        style.spacing.item_spacing = Vec2::new(8.0, 8.0);
        style.spacing.window_margin = Margin::same(0.0);
        cc.egui_ctx.set_style(style);

        let keybindings_vec = get_keybindings();
        let mut keybindings: HashMap<Category, Vec<Keybinding>> = HashMap::new();

        for keybinding in keybindings_vec {
            keybindings
                .entry(keybinding.category.clone())
                .or_default()
                .push(keybinding);
        }

        let filtered_keybindings = keybindings.clone();

        Self {
            keybindings,
            search_text: String::new(),
            filtered_keybindings,
        }
    }

    fn update_filter(&mut self) {
        self.filtered_keybindings.clear();

        if self.search_text.is_empty() {
            self.filtered_keybindings = self.keybindings.clone();
        } else {
            let search_lower = self.search_text.to_lowercase();

            for (category, keybindings) in &self.keybindings {
                let filtered: Vec<Keybinding> = keybindings
                    .iter()
                    .filter(|kb| {
                        kb.function.to_lowercase().contains(&search_lower)
                            || kb.description.to_lowercase().contains(&search_lower)
                            || kb.modifiers.join(" ").to_lowercase().contains(&search_lower)
                            || kb.key.to_lowercase().contains(&search_lower)
                    })
                    .cloned()
                    .collect();

                if !filtered.is_empty() {
                    self.filtered_keybindings.insert(category.clone(), filtered);
                }
            }
        }
    }

    fn get_category_color(&self, category: &Category) -> Color32 {
        match category {
            Category::Media => Color32::from_rgb(243, 139, 168), // Red
            Category::Screenshot => Color32::from_rgb(250, 179, 135), // Orange
            Category::Applications => Color32::from_rgb(249, 226, 175), // Yellow
            Category::WindowManagement => Color32::from_rgb(166, 227, 161), // Green
            Category::Layout => Color32::from_rgb(148, 226, 213), // Teal
            Category::Gaps => Color32::from_rgb(137, 180, 250), // Blue
            Category::Navigation => Color32::from_rgb(203, 166, 247), // Purple
            Category::Tags => Color32::from_rgb(245, 194, 231), // Pink
            Category::System => Color32::from_rgb(235, 160, 172), // Maroon
            Category::Borders => Color32::from_rgb(180, 190, 254), // Lavender
        }
    }
}

impl eframe::App for CheatsheetApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle escape key
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            std::process::exit(0);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            let available_width = ui.available_width();
            
            // --- Header Section ---
            ui.add_space(24.0);
            ui.vertical_centered(|ui| {
                ui.label(
                    RichText::new("DWM Keybinding Cheatsheet")
                        .size(32.0)
                        .color(MaterialColors::ON_SURFACE)
                        .strong(),
                );
                ui.add_space(8.0);
                ui.label(
                    RichText::new("Press Escape to quit")
                        .size(14.0)
                        .color(MaterialColors::ON_SURFACE_VARIANT),
                );
            });
            ui.add_space(24.0);

            // --- Search Bar ---
            let search_bar_width = (available_width * 0.6).clamp(300.0, 600.0);
            ui.vertical_centered(|ui| {
                Frame::none()
                    .fill(MaterialColors::SURFACE_CONTAINER_HIGH)
                    .rounding(Rounding::same(24.0))
                    .inner_margin(Margin::symmetric(16.0, 12.0))
                    .stroke(Stroke::new(1.0, MaterialColors::OUTLINE_VARIANT))
                    .show(ui, |ui| {
                        ui.set_width(search_bar_width);
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("🔍").size(16.0).color(MaterialColors::ON_SURFACE_VARIANT));
                            ui.add_space(8.0);
                            let text_edit = egui::TextEdit::singleline(&mut self.search_text)
                                .frame(false)
                                .hint_text(RichText::new("Search keybindings...").color(MaterialColors::ON_SURFACE_VARIANT))
                                .text_color(MaterialColors::ON_SURFACE)
                                .desired_width(f32::INFINITY);
                            
                            let response = ui.add(text_edit);
                            if response.changed() {
                                self.update_filter();
                            }
                            response.request_focus();
                        });
                    });
            });
            ui.add_space(32.0);

            // --- Content Grid ---
            ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    let width = ui.available_width();
                    // Responsive columns
                    let num_columns = if width > 1400.0 {
                        3
                    } else if width > 900.0 {
                        2
                    } else {
                        1
                    };
                    
                    let column_width = (width - (num_columns as f32 - 1.0) * 16.0) / num_columns as f32;
                    
                    // Distribute categories into columns
                    let categories = [
                        Category::Media,
                        Category::Screenshot,
                        Category::Applications,
                        Category::WindowManagement,
                        Category::Navigation,
                        Category::Layout,
                        Category::Gaps,
                        Category::Tags,
                        Category::System,
                        Category::Borders,
                    ];

                    let mut columns: Vec<Vec<&Category>> = vec![Vec::new(); num_columns];
                    for (i, category) in categories.iter().enumerate() {
                        if self.filtered_keybindings.contains_key(category) {
                            columns[i % num_columns].push(category);
                        }
                    }

                    ui.horizontal_top(|ui| {
                        for col_idx in 0..num_columns {
                            ui.vertical(|ui| {
                                ui.set_width(column_width);
                                for category in &columns[col_idx] {
                                    if let Some(keybindings) = self.filtered_keybindings.get(category) {
                                        self.render_category_card(ui, category, keybindings);
                                        ui.add_space(16.0);
                                    }
                                }
                            });
                            
                            if col_idx < num_columns - 1 {
                                ui.add_space(16.0);
                            }
                        }
                    });
                    
                    ui.add_space(32.0);
                });
        });
    }
}

impl CheatsheetApp {
    fn render_category_card(&self, ui: &mut Ui, category: &Category, keybindings: &[Keybinding]) {
        let accent_color = self.get_category_color(category);
        
        Frame::none()
            .fill(MaterialColors::SURFACE_CONTAINER)
            .rounding(Rounding::same(16.0))
            .stroke(Stroke::new(1.0, MaterialColors::OUTLINE_VARIANT))
            .inner_margin(Margin::same(0.0))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                
                // Card Header
                Frame::none()
                    .fill(accent_color.gamma_multiply(0.15)) // Tinted background
                    .rounding(Rounding {
                        nw: 16.0,
                        ne: 16.0,
                        sw: 0.0,
                        se: 0.0,
                    })
                    .inner_margin(Margin::symmetric(20.0, 16.0))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            // Colored pill indicator
                            let (rect, _) = ui.allocate_exact_size(Vec2::new(4.0, 20.0), egui::Sense::hover());
                            ui.painter().rect_filled(rect, 2.0, accent_color);
                            
                            ui.add_space(12.0);
                            ui.label(
                                RichText::new(category.as_str())
                                    .size(18.0)
                                    .strong()
                                    .color(MaterialColors::ON_SURFACE),
                            );
                        });
                    });

                // Card Content
                ui.allocate_ui(Vec2::new(ui.available_width(), 0.0), |ui| {
                    let mut style = (**ui.style()).clone();
                    style.spacing.item_spacing = Vec2::new(0.0, 0.0); // Manual spacing
                    ui.set_style(style);
                    
                    Frame::none()
                        .inner_margin(Margin::symmetric(20.0, 16.0))
                        .show(ui, |ui| {
                            for (i, keybinding) in keybindings.iter().enumerate() {
                                if i > 0 {
                                    ui.add_space(16.0);
                                    // Divider
                                    let (rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 1.0), egui::Sense::hover());
                                    ui.painter().rect_filled(rect, 0.0, MaterialColors::OUTLINE_VARIANT.gamma_multiply(0.3));
                                    ui.add_space(16.0);
                                }
                                self.render_keybinding_row(ui, keybinding, accent_color);
                            }
                        });
                });
            });
    }

    fn render_keybinding_row(&self, ui: &mut Ui, keybinding: &Keybinding, accent_color: Color32) {
        ui.vertical(|ui| {
            // Top row: Keys and Function
            ui.horizontal(|ui| {
                // Keys
                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing = Vec2::new(4.0, 4.0);
                    for (i, modifier) in keybinding.modifiers.iter().enumerate() {
                        self.render_key_chip(ui, modifier, accent_color);
                        if i < keybinding.modifiers.len() - 1 || !keybinding.key.is_empty() {
                            ui.label(RichText::new("+").size(12.0).color(MaterialColors::OUTLINE));
                        }
                    }
                    if !keybinding.key.is_empty() {
                        self.render_key_chip(ui, &keybinding.key, accent_color);
                    }
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        RichText::new(&keybinding.function)
                            .size(14.0)
                            .strong()
                            .color(MaterialColors::PRIMARY),
                    );
                });
            });
            
            ui.add_space(4.0);
            
            // Bottom row: Description
            ui.label(
                RichText::new(&keybinding.description)
                    .size(13.0)
                    .color(MaterialColors::ON_SURFACE_VARIANT),
            );
        });
    }

    fn render_key_chip(&self, ui: &mut Ui, text: &str, _accent_color: Color32) {
        Frame::none()
            .fill(MaterialColors::SURFACE_CONTAINER_HIGH)
            .stroke(Stroke::new(1.0, MaterialColors::OUTLINE_VARIANT))
            .rounding(Rounding::same(6.0))
            .inner_margin(Margin::symmetric(8.0, 4.0))
            .show(ui, |ui| {
                ui.label(
                    RichText::new(text)
                        .size(12.0)
                        .family(egui::FontFamily::Monospace)
                        .color(MaterialColors::ON_SURFACE),
                );
            });
    }
}
