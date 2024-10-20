use futures::executor::block_on;
use std::hash::{Hash, Hasher};

use egui::{
    Align, Align2, Area, Color32, Direction, FontData, FontDefinitions, FontFamily, Frame, Id,
    Layout, Margin, Memory, RichText, ScrollArea, Stroke, Style, TextStyle, Vec2, ViewportCommand,
};

use super::utils::{parse_args, translate_from_args};

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
        style.text_styles.insert(
            TextStyle::Name("medium".into()),
            egui::FontId::new(18.0, FontFamily::Proportional),
        );
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
                        ui.label(RichText::new(&self.response).size(20.));
                    });
                });
            });
        ctx.input(|i| {
            if i.pointer.any_click() || i.key_pressed(egui::Key::Escape) {
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
                let args = parse_args(&self.prompt);
                println!("args: {:#?}", args);
                let result_future = translate_from_args(args);

                let result = block_on(result_future);
                match result {
                    Ok(response) => {
                        self.prompt.clear();
                        self.response = response;
                    }
                    Err(e) => {
                        self.response = format!("Error: {}", e);
                    }
                }
            }
        });
    }
}
