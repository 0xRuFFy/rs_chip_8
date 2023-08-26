#[allow(dead_code)]

mod cpu_8;

use cpu_8::C8Cpu;
use std::fs;

fn main() { 

    let rom = fs::read("roms/ibm_logo.ch8").expect("Unable to read file");
    println!("rom size: {}b", rom.len());

    let mut cpu = C8Cpu::new();
    // println!("{}", cpu);

    cpu.load_rom(rom);

    while cpu.is_running() {
        cpu.single_cycle();

        if cpu.draw_flag {
            cpu.draw_flag = false;
            cpu.print_display();
        }
    }
}
