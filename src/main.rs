mod text_core;
mod gui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("Quartz IDE"),
        ..Default::default()
    };

    eframe::run_native(
        "Quartz IDE",
        options,
        Box::new(|cc| Ok(Box::new(gui::IdeApp::new(cc)))),
    )
}
