use crate::memory::MemoryAccess;

const SCREEN_WIDTH: usize = 240;
const SCREEN_HEIGHT: usize = 160;

pub struct PPU {
    pub framebuffer: Vec<u8>,
}

impl PPU {
    pub fn new() -> Self {
        PPU {
            framebuffer: vec![0; SCREEN_WIDTH * SCREEN_HEIGHT * 4],
        }
    }

    pub fn render(&mut self, memory: &dyn MemoryAccess) {
        // Check palette and VRAM
        let palette0 = memory.read_u16(0x05000000);
        let palette2 = memory.read_u16(0x05000002);
        let vram0 = memory.read_u16(0x06000000);
        let vram4000 = memory.read_u16(0x06400000);
        eprintln!("Render: Palette[0]=0x{:04X} Palette[2]=0x{:04X} VRAM[0]=0x{:04X} VRAM[0x4000]=0x{:04X}", 
            palette0, palette2, vram0, vram4000);

        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let addr = (y * SCREEN_WIDTH + x) * 2;
                let color_u16 = memory.read_u16(0x06000000 + addr as u32);
                let color = self.rgb565_to_rgba8(color_u16);

                let fb_offset = (y * SCREEN_WIDTH + x) * 4;
                self.framebuffer[fb_offset] = color.0;
                self.framebuffer[fb_offset + 1] = color.1;
                self.framebuffer[fb_offset + 2] = color.2;
                self.framebuffer[fb_offset + 3] = 255;
            }
        }
    }

    fn rgb565_to_rgba8(&self, rgb565: u16) -> (u8, u8, u8) {
        let r = ((rgb565 >> 11) & 0x1F) as u8;
        let g = ((rgb565 >> 5) & 0x3F) as u8;
        let b = (rgb565 & 0x1F) as u8;

        let r = (r << 3) | (r >> 2);
        let g = (g << 2) | (g >> 4);
        let b = (b << 3) | (b >> 2);

        (r, g, b)
    }

    pub fn get_framebuffer(&self) -> &[u8] {
        &self.framebuffer
    }
}

impl Default for PPU {
    fn default() -> Self {
        Self::new()
    }
}
