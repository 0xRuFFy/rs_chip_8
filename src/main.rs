#[allow(dead_code)]
mod cpu_8;

use cpu_8::{C8Cpu, DISPLAY_HEIGHT, DISPLAY_WIDTH};
use macroquad::prelude::*;
use std::fs::{self, DirEntry};

const DISPLAY_SCALE: u32 = 20;

fn get_extention(entry: &DirEntry) -> Option<String> {
    let path = entry.path();
    let ext = path.extension()?.to_str()?.to_owned();
    Some(ext)
}

#[macroquad::main("Chip-8")]
async fn main() {
    let rom_path = match std::env::args().nth(1) {
        Some(path) => path,
        None => {
            println!("Please provide a rom path");
            if let Ok(roms) = fs::read_dir("./roms") {
                let mut found = false;
                for rom in roms {
                    if let Ok(rom) = rom {
                        if let Some(ext) = get_extention(&rom) {
                            if ext == "ch8" {
                                if !found {
                                    found = true;
                                    println!("Possible roms:");
                                }
                                println!("{}", rom.path().display().to_string().replace("\\", "/"));
                            }
                        }
                    }
                }
            }
            std::process::exit(1);
        }
    };
    let rom = fs::read(rom_path).expect("Unable to read file");
    // let rom = fs::read("roms/ibm_logo.ch8").expect("Unable to read file");
    // let rom = fs::read("roms/test_opcode.ch8").expect("Unable to read file");
    // let rom = fs::read("roms/chip8-logo.ch8").expect("Unable to read file");
    // let rom = fs::read("roms/3-corax+.ch8").expect("Unable to read file");
    // let rom = fs::read("roms/4-flags.ch8").expect("Unable to read file");
    // let rom = fs::read("roms/5-quirks.ch8").expect("Unable to read file");
    // let rom = fs::read("roms/fishie.ch8").expect("Unable to read file");
    // let rom = fs::read("roms/maze.ch8").expect("Unable to read file");
    // let rom = fs::read("roms/particle.ch8").expect("Unable to read file");
    // let rom = fs::read("roms/zero.ch8").expect("Unable to read file");
    // let rom = fs::read("roms/trip8.ch8").expect("Unable to read file");
    // let rom = fs::read("roms/sierpinski.ch8").expect("Unable to read file");
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
