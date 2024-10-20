use app::main_window::MainWindow;
use eframe::WindowAttributes;
use egui::Vec2;
use tokio::runtime::Runtime;

mod app;
const APP_NAME: &str = "Ray Translate";

fn main() -> eframe::Result {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_decorations(false)
            .with_transparent(true)
            .with_fullscreen(true)
            .with_always_on_top(),
        ..Default::default()
    };

    eframe::run_native(
        APP_NAME,
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<MainWindow>::default())
        }),
    )
}
