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
        let dispcnt = memory.read_u32(0x04000000);
        let bg0cnt = memory.read_u32(0x04000008);

        let bg_enabled = (dispcnt & (1 << 8)) != 0;
        if !bg_enabled {
            return;
        }

        let screen_block = (bg0cnt >> 8) & 0x1F;
        let color_mode = (bg0cnt >> 7) & 1;

        let screen_base = 0x06000000 + (screen_block as u32) * 0x800;
        let char_base = 0x06000000 + ((bg0cnt >> 2) & 0xF) as u32 * 0x4000;

        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let map_x = x / 8;
                let map_y = y / 8;
                let tile_index_addr = screen_base + (map_y as u32 * 32 + map_x as u32) * 2;
                let tile_entry = memory.read_u16(tile_index_addr);

                let tile_num = tile_entry & 0x3FF;
                let palette_num = (tile_entry >> 10) & 0xF;

                let tile_addr = char_base + tile_num as u32 * 32;
                let pixel_in_tile = (y % 8) * 8 + (x % 8);

                let pixel_addr = tile_addr + pixel_in_tile as u32;
                let pixel = memory.read_u8(pixel_addr);

                let color_addr = if color_mode == 0 {
                    0x05000000 + pixel as u32 * 2
                } else {
                    0x05000000 + palette_num as u32 * 16 * 2 + pixel as u32 * 2
                };
                let color = memory.read_u16(color_addr);

                let color565 = self.rgb555_to_rgb565(color);
                let rgb = self.rgb565_to_rgba8(color565);

                let fb_offset = (y * SCREEN_WIDTH + x) * 4;
                self.framebuffer[fb_offset] = rgb.0;
                self.framebuffer[fb_offset + 1] = rgb.1;
                self.framebuffer[fb_offset + 2] = rgb.2;
                self.framebuffer[fb_offset + 3] = 255;
            }
        }
    }

    fn rgb555_to_rgb565(&self, rgb555: u16) -> u16 {
        let r = (rgb555 >> 10) & 0x1F;
        let g = (rgb555 >> 5) & 0x1F;
        let b = rgb555 & 0x1F;
        (r << 11) | (g << 6) | b
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
