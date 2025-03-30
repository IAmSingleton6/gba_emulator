// use eframe::egui;
mod cpu;

fn main() {
    let mut cpu: cpu::CPU = cpu::CPU::new();
    let mut memory: [u32; 256] = [0; 256];
    memory[0] = 0xE0000000;

    for _ in 0..5 {
        cpu.fetch_decode_execute(&mut memory);
    }
}