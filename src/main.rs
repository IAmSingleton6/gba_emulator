use eframe::egui;

mod cpu;
mod memory;
mod ppu;

use cpu::CPU;
use memory::Memory;
use ppu::PPU;

struct GBAApp {
    memory: Option<Memory>,
    cpu: Option<CPU>,
    ppu: PPU,
    rom_path: String,
    error_message: Option<String>,
    running: bool,
}

impl GBAApp {
    fn new() -> Self {
        let mut app = GBAApp {
            memory: None,
            cpu: None,
            ppu: PPU::new(),
            rom_path: String::new(),
            error_message: None,
            running: false,
        };

        // Auto-load stripes.gba for testing
        app.load_rom("tests/roms/ppu/stripes.gba");

        app
    }

    fn load_rom(&mut self, path: &str) {
        let mut memory = Memory::new();

        match memory.load_rom(path) {
            Ok(()) => {
                let mut cpu = CPU::new(Box::new(memory.clone()));
                cpu.initialize_gba();

                self.memory = Some(memory);
                self.cpu = Some(cpu);
                self.error_message = None;
                self.running = true;
            }
            Err(e) => {
                self.error_message = Some(e);
            }
        }
    }

    fn step(&mut self) {
        if let Some(ref mut cpu) = self.cpu {
            cpu.fetch_decode_execute();
        }
    }

    fn run_frame(&mut self) {
        if !self.running {
            return;
        }

        for _ in 0..100000 {
            self.step();
        }

        if let Some(ref cpu) = self.cpu {
            self.ppu.render(cpu.get_memory());
        }
    }
}

impl eframe::App for GBAApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("GBA Emulator");

            ui.horizontal(|ui| {
                ui.label("ROM Path:");
                ui.text_edit_singleline(&mut self.rom_path);

                if ui.button("Load ROM").clicked() {
                    if !self.rom_path.is_empty() {
                        let path = self.rom_path.clone();
                        self.load_rom(&path);
                    }
                }
            });

            ui.horizontal(|ui| {
                if self.running {
                    if ui.button("Pause").clicked() {
                        self.running = false;
                    }
                } else {
                    if ui.button("Start").clicked() {
                        if self.memory.is_some() {
                            self.running = true;
                        }
                    }
                }

                if ui.button("Step").clicked() {
                    self.step();
                }
            });

            if let Some(ref err) = self.error_message {
                ui.colored_label(egui::Color32::RED, err);
            }

            if self.running {
                self.run_frame();
            }

            let fb = self.ppu.get_framebuffer();
            let img = egui::ColorImage::from_rgba_unmultiplied([240, 160], fb);
            let texture = ctx.load_texture("framebuffer", img, egui::TextureOptions::default());
            ui.image((texture.id(), egui::vec2(480.0, 320.0)));
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([520.0, 400.0]),
        ..Default::default()
    };

    eframe::run_native(
        "GBA Emulator",
        options,
        Box::new(|_cc| Ok(Box::new(GBAApp::new()))),
    )
}
