use std::fs::File;
use std::io::Read;

mod cpu;
mod memory;
mod ppu;

use cpu::CPU;
use memory::Memory;
use memory::MemoryAccess;
use ppu::PPU;

fn main() {
    let mut memory = Memory::new();
    memory.load_rom("tests/roms/ppu/shades.gba").unwrap();

    let mut cpu = CPU::new(Box::new(memory.clone()));
    cpu.initialize_gba();

    let mut last_pc = 0u32;
    let mut same_count = 0;
    let mut iterations = 0;
    while iterations < 100000 {
        let pc = cpu.get_pc();
        cpu.fetch_decode_execute();

        // Debug: print iterations at key points
        if iterations == 0 || iterations == 8 || iterations == 9 {
            eprintln!(
                "iter {}: PC=0x{:08X}, R0=0x{:08X}, R1=0x{:08X}, R2={}",
                iterations,
                pc,
                cpu.get_registers().get_r(0),
                cpu.get_registers().get_r(1),
                cpu.get_registers().get_r(2)
            );
        }

        if pc == last_pc {
            same_count += 1;
            if same_count > 10 {
                break;
            }
        } else {
            same_count = 0;
        }
        last_pc = pc;
        iterations += 1;
    }

    let dispcnt = cpu.get_memory().read_u32(0x04000000);
    let bg0cnt = cpu.get_memory().read_u32(0x04000008);
    println!("DISPCNT = 0x{:08X}", dispcnt);
    println!("BG0CNT = 0x{:08X}", bg0cnt);

    println!("\nPalette (first 32 entries):");
    for i in 0..32 {
        let val = cpu.get_memory().read_u16(0x05000000 + i * 2);
        if val != 0 {
            println!("  [{}] = 0x{:04X}", i, val);
        } else {
            print!(".");
        }
    }
    println!();

    let mut ppu = PPU::new();
    ppu.render(cpu.get_memory());

    // Debug: check VRAM at expected char base
    eprintln!("\nVRAM at 0x06014000 (first 32 bytes):");
    for i in 0..32 {
        let val = cpu.get_memory().read_u8(0x06014000 + i);
        eprint!("{:02X} ", val);
        if i % 16 == 15 {
            eprintln!();
        }
    }

    // Print final state
    eprintln!(
        "\nFinal R0 = 0x{:08X}, R1 = 0x{:08X}",
        cpu.get_registers().get_r(0),
        cpu.get_registers().get_r(1)
    );
}
