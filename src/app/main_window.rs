use std::hash::{Hash, Hasher};

use egui::{
    Align2, Area, Color32, FontDefinitions, FontFamily, Frame, Id, RichText, ScrollArea, Style,
    TextStyle, Vec2,
};

use crate::app::utils::translate;

use super::utils::{parse_args, render_ansi_text};

pub struct MainWindow {
    prompt: String,
    response: String,
}

impl Default for MainWindow {
    fn default() -> Self {
        Self {
            prompt: String::new(),
            response: String::new(),
        }
    }
}

impl eframe::App for MainWindow {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        let hashed_id = Id::new(hasher.write_i32(54).hash(&mut hasher));
        let mut style: Style = (*ctx.style()).clone();
        style.visuals.widgets.inactive.bg_fill = Color32::from_rgb(0, 0, 0);
        style.text_styles.insert(
            TextStyle::Name("big".into()),
            egui::FontId::new(24.0, FontFamily::Proportional),
        );
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "agave".to_owned(),
            egui::FontData::from_static(include_bytes!(
                "/usr/share/fonts/AgaveNerdFont-Regular.ttf"
            )),
        );
        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "agave".to_owned());
        ctx.set_fonts(fonts);
        ctx.set_style(style);

        let mut area_rect = None;

        Area::new(hashed_id)
            .anchor(Align2::CENTER_CENTER, Vec2::new(0., 0.))
            .show(ctx, |ui| {
                Frame::window(&ui.style()).show(ui, |ui| {
                    ui.set_min_size(Vec2::new(600., 300.));
                    let input_id = ui.make_persistent_id("input_field");

                    let input = egui::TextEdit::singleline(&mut self.prompt)
                        .font(TextStyle::Name("big".into()))
                        .id(input_id);

                    ui.add_sized(Vec2::new(600., 30.), input);
                    ui.memory_mut(|mem| {
                        mem.request_focus(input_id);
                    });
                    area_rect = Some(ui.min_rect());
                    let resp_area = ScrollArea::vertical();
                    resp_area.show(ui, |ui| {
                        render_ansi_text(ui, &self.response);
                    });
                });
            });
        ctx.input(|i| {
            if i.key_pressed(egui::Key::Escape) {
                let ctx = ctx.clone();
                std::thread::spawn(move || {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                });
            }
            if i.pointer.any_click() {
                if let Some(rect) = area_rect {
                    if !rect.contains(i.pointer.interact_pos().unwrap_or_default()) {
                        let ctx = ctx.clone();
                        std::thread::spawn(move || {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        });
                    }
                }
            }
            if i.key_pressed(egui::Key::Enter) {
                let args = match parse_args(&self.prompt) {
                    Ok(args) => args,
                    Err(e) => {
                        self.response = e.to_string();
                        return;
                    }
                };

                let result = translate(args);
                match result {
                    Ok(resp) => {
                        self.prompt.clear();

                        self.response = resp;
                    }
                    Err(e) => {
                        self.response = e.to_string();
                    }
                }
            }
        });
    }
}
