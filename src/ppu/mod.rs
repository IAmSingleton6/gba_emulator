use crate::memory::Memory;
use crate::memory::MemoryAccess;

const SCREEN_WIDTH: usize = 240;
const SCREEN_HEIGHT: usize = 160;
// Test: render every 1000 cycles to see if it works
const CYCLES_PER_FRAME: u64 = 1000;

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

    // Real GBA frame timing (280896 cycles / 60fps)
    const CYCLES_PER_FRAME: u64 = 70224; // 280896 / 4

    pub fn step(&mut self, memory: &Memory, cycles: u64) -> bool {
        self.cycle_counter += cycles;

        if self.cycle_counter >= CYCLES_PER_FRAME {
            eprintln!(
                "*** VBLANK: rendering frame (cycles={})",
                self.cycle_counter
            );
            self.cycle_counter -= CYCLES_PER_FRAME;
            self.render(memory);
            return true;
        }

        false
    }

    fn render(&mut self, memory: &Memory) {
        let dispcnt = memory.read_u16(0x04000000);
        let bg0cnt = memory.read_u16(0x04000008);

        eprintln!("RENDER: dispcnt=0x{:04X}, bg0cnt=0x{:04X}", dispcnt, bg0cnt);

        eprintln!("RENDER: dispcnt=0x{:04X}, bg0cnt=0x{:04X}", dispcnt, bg0cnt);

        // Only Mode 0 supported
        let mode = dispcnt & 0x7;
        if mode != 0 {
            eprintln!("RENDER: SKIP - mode={} (not mode 0)", mode);
            return;
        }

        // BG0 enable
        if (dispcnt & (1 << 8)) == 0 {
            eprintln!("RENDER: SKIP - BG0 not enabled");
            return;
        }

        // Extract fields
        // BG0CNT: bits 2-3 = char block, bits 8-12 = screen block, bit 7 = color mode
        let bg0cnt = bg0cnt as u32;
        let char_block = (bg0cnt >> 2) & 0x3;
        let screen_block = (bg0cnt >> 8) & 0x1F;
        let color_mode = (bg0cnt >> 7) & 1; // 0=4bpp, 1=8bpp
        let screen_size = (bg0cnt >> 14) & 0x3;

        let tile_base = 0x06000000 + char_block * 0x4000;
        let screen_base = 0x06000000 + screen_block * 0x800;

        eprintln!(
            "RENDER: char_block={}, screen_block={}, tile_base=0x{:X}, screen_base=0x{:X}",
            char_block, screen_block, tile_base, screen_base
        );

        // Debug: check tile data at tile_base (should be 0x11111111)
        let tile_data = memory.read_u32(tile_base);
        // Debug: check screen entry at screen_base
        let screen_entry = memory.read_u16(screen_base);

        // Dump first 16 screen entries
        eprintln!("DEBUG: Dumping screen entries at 0x06008000:");
        for i in 0..16 {
            let se = memory.read_u16(screen_base + i as u32 * 2);
            eprintln!("  screen_entry[{}] = 0x{:04X}", i, se);
        }

        // For pixel (0,0): tile_x=0, tile_y=0, so screen_addr = screen_base
        // screen_entry should be tile_num (0 or 1 alternating)
        eprintln!(
            "DEBUG: tile_data[0]=0x{:08X}, screen_entry[0]=0x{:04X}",
            tile_data, screen_entry
        );

        // Debug: check palette at palette 0 and 1
        let pal0 = memory.read_u16(0x05000000);
        let pal1 = memory.read_u16(0x05000002);
        eprintln!(
            "DEBUG: palette[0]=0x{:04X}, palette[1]=0x{:04X}",
            pal0, pal1
        );

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

                if x == 0 && y == 0 {
                    eprintln!(
                        "DEBUG: world_x={}, world_y={}, tile_x={}, tile_y={}, screen_addr=0x{:08X}",
                        world_x, world_y, tile_x, tile_y, screen_addr
                    );
                }

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
