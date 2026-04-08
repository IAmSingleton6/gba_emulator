mod cpu;
mod gba;
mod gba_app;
mod memory;
mod ppu;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([520.0, 400.0]),
        ..Default::default()
    };

    eframe::run_native(
        "GBA Emulator",
        options,
        Box::new(|_cc| Ok(Box::new(gba_app::GBAApp::new()))),
    )
}
