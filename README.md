
# CHIP-8 Emulator in Rust

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A CHIP-8 emulator written in Rust.

## Table of Contents

- [Introduction](#introduction)
- [Features](#features)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
  - [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)
- [Author](#author)

## Introduction

This project is a CHIP-8 emulator implemented in Rust. CHIP-8 is an interpreted programming language that was initially designed for the COSMAC VIP computer. It has become popular among hobbyist programmers due to its simplicity and ease of implementation.

## Features

- Emulation of the CHIP-8 CPU with 16 8-bit general-purpose registers.
- 64x32 monochrome display emulation.
- Support for loading ROMs into memory.

### Not yet implemented

- Keyboard input handling.
- Playing Sound

## Getting Started

Provide instructions on how to set up and run your emulator.

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) installed on your machine.

### Installation

1. Clone the repository:
  ```bash
  git clone https://github.com/your-username/chip8-emulator-rust.git
  ```
2. Navigate to the project directory:
   ```bash
   cd .../rs_chip_8
   ```
3. Build the project:
   ```bash
   cargo build --release
   ```

### Usage

Run the emulator with the path to a CHIP-8 ROM from the terminal. Preferable in the same directory as the roms folder but not necessary.

- using cargo
  ```bash
  cargo run --release <ROM_PATH>
  ```
- running the binary
  ```bash
  mv target/release/chip8 . && ./chip8 <ROM_PATH>
  ```

If you don't provide a ROM path, the emulator will attempt to list available ROMs in the roms directory.

### Contributing

Feel free to contribute by opening issues, submitting feature requests, or creating pull requests. Your input is highly appreciated!

### License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

### Author

Konstantin Opora
