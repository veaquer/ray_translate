use app::main_window::MainWindow;
use single_instance::SingleInstance;

mod app;
const APP_NAME: &str = "Ray Translate";

fn main() {
    env_logger::init();
    let instance = SingleInstance::new("ray_translate_veaquer").unwrap();

    // Check if another instance is already running
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_decorations(false)
            .with_transparent(true)
            .with_fullscreen(true)
            .with_always_on_top(),
        ..Default::default()
    };
    if instance.is_single() {
        eframe::run_native(
            APP_NAME,
            options,
            Box::new(|cc| {
                egui_extras::install_image_loaders(&cc.egui_ctx);

                Ok(Box::<MainWindow>::default())
            }),
        );
    }
}
