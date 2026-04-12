use crate::gba::GBA;

pub struct GBAApp {
    gba: Option<GBA>,
    rom_path: String,
    error_message: Option<String>,
    running: bool,
}

impl GBAApp {
    pub fn new() -> Self {
        let mut app = GBAApp {
            gba: None,
            rom_path: String::new(),
            error_message: None,
            running: false,
        };

        app.load_rom("tests/roms/ppu/stripes.gba");
        app
    }

    fn load_rom(&mut self, path: &str) {
        let mut gba = GBA::new();

        match gba.load_rom(path) {
            Ok(()) => {
                self.gba = Some(gba);
                self.error_message = None;
                self.running = true;
            }
            Err(e) => {
                self.error_message = Some(e);
                self.running = false;
            }
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
                        if self.gba.is_some() {
                            self.running = true;
                        }
                    }
                }

                if ui.button("Step").clicked() {
                    if let Some(gba) = &mut self.gba {
                        gba.step();
                    }
                }
            });

            if let Some(ref err) = self.error_message {
                ui.colored_label(egui::Color32::RED, err);
            }

            if self.running {
                if let Some(gba) = &mut self.gba {
                    gba.run_frame();
                }
            }

            if let Some(gba) = &self.gba {
                let fb = gba.get_framebuffer();
                let img = egui::ColorImage::from_rgba_unmultiplied([240, 160], fb);
                let texture = ctx.load_texture("framebuffer", img, egui::TextureOptions::default());
                ui.image((texture.id(), egui::vec2(480.0, 320.0)));
            }
        });
    }
}
