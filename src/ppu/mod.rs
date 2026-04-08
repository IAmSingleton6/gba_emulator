use crate::memory::Memory;
use crate::memory::MemoryAccess;

const SCREEN_WIDTH: usize = 240;
const SCREEN_HEIGHT: usize = 160;
const CYCLES_PER_FRAME: u64 = 280_896;

pub struct PPU {
    framebuffer: Vec<u8>,
    cycle_counter: u64,
}

impl PPU {
    pub fn new() -> Self {
        Self {
            framebuffer: vec![0; SCREEN_WIDTH * SCREEN_HEIGHT * 4],
            cycle_counter: 0,
        }
    }

    pub fn step(&mut self, memory: &Memory, cycles: u64) -> bool {
        self.cycle_counter += cycles;

        if self.cycle_counter >= CYCLES_PER_FRAME {
            self.cycle_counter -= CYCLES_PER_FRAME;
            self.render(memory);
            return true;
        }

        false
    }

    fn render(&mut self, memory: &Memory) {
        let dispcnt = memory.read_u16(0x04000000);
        let bg0cnt = memory.read_u16(0x04000008);

        // Only Mode 0 supported
        let mode = dispcnt & 0x7;
        if mode != 0 {
            return;
        }

        // BG0 enable
        if (dispcnt & (1 << 8)) == 0 {
            return;
        }

        // Extract fields
        let char_block = (bg0cnt & 0x3) as u32;
        let screen_block = ((bg0cnt >> 8) & 0x1F) as u32;
        let color_mode = (bg0cnt >> 7) & 1; // 0=4bpp, 1=8bpp
        let screen_size = (bg0cnt >> 14) & 0x3;

        let tile_base = 0x06000000 + char_block * 0x4000;
        let screen_base = 0x06000000 + screen_block * 0x800;

        // Scroll
        let scroll_x = memory.read_u16(0x04000010) as usize;
        let scroll_y = memory.read_u16(0x04000012) as usize;

        // BG dimensions
        let (bg_width, bg_height) = match screen_size {
            0 => (256, 256),
            1 => (512, 256),
            2 => (256, 512),
            3 => (512, 512),
            _ => unreachable!(),
        };

        let tile_size = if color_mode == 0 { 32 } else { 64 };

        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                // Apply scroll + wrapping
                let world_x = (x + scroll_x) % bg_width;
                let world_y = (y + scroll_y) % bg_height;

                let tile_x = world_x / 8;
                let tile_y = world_y / 8;

                // Screenblock addressing (handles 64x64 etc.)
                let screenblock_index = match (bg_width, bg_height) {
                    (256, 256) => 0,
                    (512, 256) => tile_x / 32,
                    (256, 512) => (tile_y / 32) * 2,
                    (512, 512) => (tile_y / 32) * 2 + (tile_x / 32),
                    _ => 0,
                };

                let screen_addr = screen_base
                    + (screenblock_index as u32) * 0x800
                    + ((tile_y % 32) * 32 + (tile_x % 32)) as u32 * 2;

                let tile_entry = memory.read_u16(screen_addr);

                let tile_num = (tile_entry & 0x3FF) as usize;
                let hflip = (tile_entry & (1 << 10)) != 0;
                let vflip = (tile_entry & (1 << 11)) != 0;
                let palette_num = ((tile_entry >> 12) & 0xF) as usize;

                let mut px = world_x % 8;
                let mut py = world_y % 8;

                if hflip {
                    px = 7 - px;
                }
                if vflip {
                    py = 7 - py;
                }

                let tile_addr = tile_base + (tile_num * tile_size) as u32;

                let pixel = if color_mode == 0 {
                    // 4bpp
                    let byte_addr = tile_addr + (py * 4 + px / 2) as u32;
                    let byte = memory.read_u8(byte_addr);

                    if px % 2 == 0 {
                        byte & 0x0F
                    } else {
                        byte >> 4
                    }
                } else {
                    // 8bpp
                    let byte_addr = tile_addr + (py * 8 + px) as u32;
                    memory.read_u8(byte_addr)
                };

                // Transparency (optional: skip)
                if pixel == 0 {
                    self.write_pixel(x, y, 0, 0, 0);
                    continue;
                }

                let palette_addr = if color_mode == 0 {
                    0x05000000 + (palette_num * 16 + pixel as usize) as u32 * 2
                } else {
                    0x05000000 + pixel as u32 * 2
                };

                let color = memory.read_u16(palette_addr);
                let (r, g, b) = Self::rgb555_to_rgba8(color);

                self.write_pixel(x, y, r, g, b);
            }
        }
    }

    fn write_pixel(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8) {
        let idx = (y * SCREEN_WIDTH + x) * 4;
        self.framebuffer[idx] = r;
        self.framebuffer[idx + 1] = g;
        self.framebuffer[idx + 2] = b;
        self.framebuffer[idx + 3] = 255;
    }

    fn rgb555_to_rgba8(color: u16) -> (u8, u8, u8) {
        let r = ((color >> 10) & 0x1F) as u8;
        let g = ((color >> 5) & 0x1F) as u8;
        let b = (color & 0x1F) as u8;

        let r = (r << 3) | (r >> 2);
        let g = (g << 3) | (g >> 2);
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
