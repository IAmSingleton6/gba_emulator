mod gb;
use gb::ram::RAM as memory;

use eframe::egui;


struct EmulatorApp {
    running: bool,
    last_frame: Instant,
    framebuffer: Vec<u8>,
    sdl_context: sdl2::Sdl,
    sdl_canvas: sdl2::render::Canvas<Window>,
}

fn main() {
    let mut mem = memory::new();
    mem.write(42, 0xF3);


    let native_options = eframe::NativeOptions::default();
    eframe::run_native("gba_emulator", native_options, Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))));
}


#[derive(Default)]
struct MyEguiApp {}


impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
        });
    }
}