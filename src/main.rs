#[allow(dead_code)]
mod cpu_8;

use cpu_8::{C8Cpu, DISPLAY_HEIGHT, DISPLAY_WIDTH};
use macroquad::prelude::*;
use std::fs;

const DISPLAY_SCALE: u32 = 20;

// fn main() {
//     // let rom = fs::read("roms/ibm_logo.ch8").expect("Unable to read file");
//     let rom = fs::read("roms/test_opcode.ch8").expect("Unable to read file");
//     println!("rom size: {}b", rom.len());

//     let mut cpu = C8Cpu::new();
//     // println!("{}", cpu);

//     cpu.load_rom(rom);

//     while cpu.is_running() {
//         cpu.single_cycle();

//         if cpu.draw_flag {
//             cpu.draw_flag = false;
//             cpu.print_display();
//         }
//     }
// }

#[macroquad::main("Chip-8")]
async fn main() {
    // let rom = fs::read("roms/ibm_logo.ch8").expect("Unable to read file");
    // let rom = fs::read("roms/test_opcode.ch8").expect("Unable to read file");
    // let rom = fs::read("roms/chip8-logo.ch8").expect("Unable to read file");
    // let rom = fs::read("roms/3-corax+.ch8").expect("Unable to read file");
    // let rom = fs::read("roms/4-flags.ch8").expect("Unable to read file");
    // let rom = fs::read("roms/5-quirks.ch8").expect("Unable to read file");
    // let rom = fs::read("roms/fishie.ch8").expect("Unable to read file");
    // let rom = fs::read("roms/maze.ch8").expect("Unable to read file");
    let rom = fs::read("roms/particle.ch8").expect("Unable to read file");
    println!("rom size: {}b", rom.len());

    let mut cpu = C8Cpu::new();
    println!("{}", cpu);

    cpu.load_rom(rom);

    let mut display = [false; DISPLAY_WIDTH * DISPLAY_HEIGHT];

    loop {
        request_new_screen_size(
            (DISPLAY_WIDTH * DISPLAY_SCALE as usize) as f32,
            (DISPLAY_HEIGHT * DISPLAY_SCALE as usize) as f32,
        );

        cpu.single_cycle();

        if cpu.draw_flag {
            // println!("{}", cpu);
            cpu.draw_flag = false;

            display = *cpu.get_display();
        }

        clear_background(BLACK);
        for i in 0..DISPLAY_WIDTH * DISPLAY_HEIGHT {
            let x = i % DISPLAY_WIDTH;
            let y = i / DISPLAY_WIDTH;

            if display[i] {
                draw_rectangle(
                    x as f32 * DISPLAY_SCALE as f32,
                    y as f32 * DISPLAY_SCALE as f32,
                    DISPLAY_SCALE as f32,
                    DISPLAY_SCALE as f32,
                    WHITE,
                );
            }
        }

        next_frame().await
    }
}
