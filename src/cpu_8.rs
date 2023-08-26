use std::fmt;

macro_rules! invalid_opcode {
    ($opcode:expr) => {
        panic!("Invalid opcode: 0x{:x}", $opcode);
    };
}

macro_rules! unimplemented_opcode {
    ($opcode:expr) => {
        panic!("Unimplemented opcode: 0x{:x}", $opcode);
    };
}

// CONSTANT DEFINITIONS
const MEM_SIZE: usize = 4096;
const MEM_START: u16 = 0x200;
const REG_COUNT: usize = 16;
const STACK_SIZE: usize = 16;
const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;
const INSTRUCTION_SIZE: u16 = 2;
const REFRESH_RATE: u16 = 60;

const FONTSET_SIZE: usize = 80;
const FONTSET_START_ADDRESS: usize = 0x050;
const FONTSET: [u8; FONTSET_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

// CHIP-8 CPU
pub struct C8Cpu {
    memory: [u8; MEM_SIZE],                             // 4K memory
    v: [u8; REG_COUNT],                                 // 16 8-bit general purpose (variable) registers
    stack: [u16; STACK_SIZE],                           // 16-level stack
    display: [bool; DISPLAY_WIDTH * DISPLAY_HEIGHT],    // 64x32 monochrome display
    pc: u16,                                            // program counter
    i: u16,                                             // 16-bit index register (stores memory addresses) -- only first 12-bit are used
    dt: u8,                                             // delay timer
    st: u8,                                             // sound timer 
    keypad: u16,                                        // 16-key hexadecimal keypad
    
    pub draw_flag: bool,                                // draw flag (set to true when a draw instruction is executed)
    running: bool,                                      // running flag (set to false when the program is finished)
}

impl C8Cpu {
    // PUBLIC METHODS

    /// Creates a new C8Cpu instance.
    ///
    /// # Returns
    ///
    /// * C8Cpu instance
    ///
    /// # Examples
    ///
    /// ```
    /// let cpu = C8Cpu::new();
    /// ```
    pub fn new() -> Self {
        let mut cpu = C8Cpu {
            memory: [0; MEM_SIZE],
            v: [0; REG_COUNT],
            stack: [0; STACK_SIZE],
            display: [false; DISPLAY_WIDTH * DISPLAY_HEIGHT],
            pc: MEM_START, // skip first 512 bytes of memory (traditionally reserved for interpreter)
            i: 0,
            dt: 0,
            st: 0,
            keypad: 0,
            draw_flag: false,
            running: true,
        };

        // load fontset into memory
        for i in 0..FONTSET_SIZE {
            cpu.memory[FONTSET_START_ADDRESS + i] = FONTSET[i];
        }

        cpu
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Loads a ROM into memory.
    ///
    /// # Arguments
    ///
    /// * `rom` - ROM to load into memory
    ///
    /// # Examples
    ///
    /// ```
    /// let mut cpu = C8Cpu::new();
    /// cpu.load_rom(rom);
    /// ```
    pub fn load_rom(&mut self, rom: Vec<u8>) {
        for i in 0..rom.len() {
            self.memory[MEM_START as usize + i] = rom[i];
        }
    }

    pub fn single_cycle(&mut self) {
        let opcode = self.fetch();
        // println!("opcode: 0x{:X}", opcode);
        self.execute(opcode);
    }

    pub fn print_display(&self) {
        for i in 0..DISPLAY_HEIGHT {
            for j in 0..DISPLAY_WIDTH {
                if self.display[i * DISPLAY_WIDTH + j] {
                    print!("#");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }

    // PRIVATE METHODS

    /// Increments the program counter by the size of an instruction.
    fn inc_pc(&mut self) {
        self.pc += INSTRUCTION_SIZE as u16;
    }

    /// Reads an instruction from memory and increments the program counter.
    ///
    /// # Returns
    ///
    /// * 16-bit instruction
    fn fetch(&mut self) -> u16 {
        let mut opcode: u16 = 0;
        for i in 0..INSTRUCTION_SIZE {
            opcode <<= 8;
            opcode |= self.memory[(self.pc + i) as usize] as u16;
        }
        self.inc_pc();
        opcode
    }

    /// Clears the display.
    fn cls(&mut self) {
        for i in 0..self.display.len() {
            self.display[i] = false;
        }
    }

    fn execute(&mut self, opcode: u16) {
        let nnn = opcode & 0x0fff;
        let msb = __get_nibble(opcode, 0);
        let nn = (opcode & 0x00ff) as u8;
        let n = __get_nibble(opcode, 3);
        let x = __get_nibble(opcode, 1) as usize; // usize because it's used as an index
        let y = __get_nibble(opcode, 2) as usize; // usize because it's used as an index

        match msb {
            0x0 => match opcode {
                0x00e0 => {
                    // CLS
                    self.cls()
                }
                0x00ee => {
                    // RET
                    unimplemented_opcode!(opcode);
                }
                _ => {
                    invalid_opcode!(opcode);
                }
            }
            0x1 => {
                // JP nnn
                self.pc = nnn;
            }
            0x2 => {
                // CALL nnn
                unimplemented_opcode!(opcode);
            }
            0x3 => {
                // SE Vx, nn
                unimplemented_opcode!(opcode);
            }
            0x4 => {
                // SNE Vx, nn
                unimplemented_opcode!(opcode);
            }
            0x5 => match n {
                0 => {
                    // SE Vx, Vy
                    unimplemented_opcode!(opcode);
                }
                _ => {
                    invalid_opcode!(opcode);
                }
            }
            0x6 => {
                // LD Vx, nn
                self.v[x] = nn;
            }
            0x7 => {
                // ADD Vx, nn
                self.v[x] = self.v[x].wrapping_add(nn);
            }
            0x8 => match n {
                0x0 => {
                    // LD Vx, Vy
                    unimplemented_opcode!(opcode);
                }
                0x1 => {
                    // OR Vx, Vy
                    unimplemented_opcode!(opcode);
                }
                0x2 => {
                    // AND Vx, Vy
                    unimplemented_opcode!(opcode);
                }
                0x3 => {
                    // XOR Vx, Vy
                    unimplemented_opcode!(opcode);
                }
                0x4 => {
                    // ADD Vx, Vy
                    unimplemented_opcode!(opcode);
                }
                0x5 => {
                    // SUB Vx, Vy
                    unimplemented_opcode!(opcode);
                }
                0x6 => {
                    // SHR Vx {, Vy}
                    unimplemented_opcode!(opcode);
                }
                0x7 => {
                    // SUBN Vx, Vy
                    unimplemented_opcode!(opcode);
                }
                0xe => {
                    // SHL Vx {, Vy}
                    unimplemented_opcode!(opcode);
                }
                _ => {
                    invalid_opcode!(opcode);
                }
            }
            0x9 => match n {
                0 => {
                    // SNE Vx, Vy
                    unimplemented_opcode!(opcode);
                }
                _ => {
                    invalid_opcode!(opcode);
                }
            }
            0xa => {
                // LD I, nnn
                self.i = nnn;
            }
            0xb => {
                // JP V0, nnn
                unimplemented_opcode!(opcode);
            }
            0xc => {
                // RND Vx, nn
                unimplemented_opcode!(opcode);
            }
            0xd => {
                // DRW Vx, Vy, n
                let _x = self.v[x] & (DISPLAY_WIDTH - 1) as u8;
                let _y = self.v[y] & (DISPLAY_HEIGHT - 1) as u8;

                self.v[0xf] = 0; // reset VF

                for i in 0..n {
                    let sprite = self.memory[(self.i + i as u16) as usize];
                    for j in 0..8 {
                        let pixel = (sprite >> (7 - j)) & 0x1;
                        let __x = (_x + j) as usize;
                        let __y = (_y + i) as usize;
                        let index = __y * DISPLAY_WIDTH + __x;
                        if pixel == 1 && self.display[index] {
                            self.v[0xf] = 1;
                        }
                        self.display[index] ^= pixel == 1;
                    }
                }

                self.draw_flag = true;
            }
            0xe => match nn {
                0x9e => {
                    // SKP Vx
                    unimplemented_opcode!(opcode);
                }
                0xa1 => {
                    // SKNP Vx
                    unimplemented_opcode!(opcode);
                }
                _ => {
                    invalid_opcode!(opcode);
                }
            }
            0xf => match nn {
                0x07 => {
                    // LD Vx, DT
                    unimplemented_opcode!(opcode);
                }
                0x0a => {
                    // LD Vx, K
                    unimplemented_opcode!(opcode);
                }
                0x15 => {
                    // LD DT, Vx
                    unimplemented_opcode!(opcode);
                }
                0x18 => {
                    // LD ST, Vx
                    unimplemented_opcode!(opcode);
                }
                0x1e => {
                    // ADD I, Vx
                    unimplemented_opcode!(opcode);
                }
                0x29 => {
                    // LD F, Vx
                    unimplemented_opcode!(opcode);
                }
                0x33 => {
                    // LD B, Vx
                    unimplemented_opcode!(opcode);
                }
                0x55 => {
                    // LD [I], Vx
                    unimplemented_opcode!(opcode);
                }
                0x65 => {
                    // LD Vx, [I]
                    unimplemented_opcode!(opcode);
                }
                _ => {
                    invalid_opcode!(opcode);
                }
            }
            _ => {
                invalid_opcode!(opcode);
            }
        }
    }
}

impl fmt::Display for C8Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "C8Cpu {{\n")?;
        write!(f, "    Memory Size: {}kB\n", MEM_SIZE)?;
        write!(f, "    Register Count: {}\n", REG_COUNT)?;
        write!(f, "    V: [ ")?;
        for i in 0..REG_COUNT {
            write!(f, "0x{:x}, ", self.v[i])?;
        }
        write!(f, "]\n")?;
        write!(f, "    I: 0x{:x}\n", self.i)?;
        write!(f, "    DT: 0x{:x}\n", self.dt)?;
        write!(f, "    ST: 0x{:x}\n", self.st)?;
        write!(f, "    Stack Size: {}\n", STACK_SIZE)?;
        write!(f, "    Stack: [ ")?;
        for i in 0..STACK_SIZE {
            write!(f, "0x{:x}, ", self.stack[i])?;
        }
        write!(f, "]\n")?;
        write!(f, "    PC: 0x{:x}\n", self.pc)?;
        write!(f, "}}")
    }
}

// Helper functions

/// Returns the value of the nth nibble (4-bit group) of a 16-bit value.
/// Where n = 0 is the most significant nibble and n = 3 is the least significant nibble.
///
/// # Arguments
///
/// * `value` - 16-bit value
/// * `index` - index of the nibble to return
///
/// # Returns
///
/// * 4-bit value
fn __get_nibble(value: u16, index: u8) -> u8 {
    if index > 3 {
        panic!("Index out of bounds: {}", index);
    }

    ((value >> (4 * (3 - index))) & 0x000f) as u8
}

