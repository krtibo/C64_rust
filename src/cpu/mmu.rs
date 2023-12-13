#![allow(non_snake_case)]
#![allow(unused)]

extern crate rand;
use rand::Rng;
use std::fs::File;
use std::io::prelude::*;

pub struct MMU {
    pub RAM : [u8; 65536],
}

impl MMU {
    pub fn new() -> MMU {
        MMU {
            RAM : [0; 65536],
        }
    }

    pub fn init(&mut self) {
        // initialize the default values of registers in memory
        self.write(0b0000_0111, 0x0001);
    }

    pub fn write(&mut self, byte : u8, address : u16) {
        self.RAM[address as usize] = byte;
    }

    pub fn copy_file_to_ram(&mut self, path: &str, address: u16) {
        let mut buffer : Vec<u8> = Vec::new();

        // open the file from path
        let mut f = File::open(&path)
        .expect("\n Error with file loading! \n");

        // read the file to rom_buffer
        f.read_to_end(&mut buffer)
        .expect("Error with file reading!");

        // copy rom_buffer to RAM address
        for i in 0..buffer.len() {
            self.RAM[address as usize + i] = buffer[i];
        }
    }

    pub fn read(&mut self, address : u16) -> u8 {
        self.RAM[address as usize]
    }

    pub fn randomize(&mut self) {
        let mut rng = rand::thread_rng();
        for i in 0..self.RAM.len() {
            self.RAM[i] = rng.gen_range(0, 2);
        }
    }

    pub fn clear(&mut self) {
        for i in 0..self.RAM.len() {
            self.RAM[i] = 0;
        }
    }
}

/* Memory map
 * 0001-0001 Memory Map config
 * 0400-07FF Screen Memory 1000 bytes   (40x25)
 * D800-DBE7 Color RAM     1000 nybbles (40x25) ----cccc
 *
 * 8000-9FFF RAM CARTLO
 * A000-BFFF RAM CARTHI   BASIC
 * C000-CFFF RAM
 * D000-DFFF RAM CHARROM  IO
 * E000-FFFF RAM CARTHI   KERNAL
 *
 * I/O
 * D000 Sprite 0 X
 * D001 Sprite 0 Y
 * D010 Sprite MSB X
 * D011 Control Register 1
 * D012 RASTER
 * D014 Light Pen X
 * D015 Light Pen Y
 * D015 Sprite Enabled
 * D016 Control Register 2
 * D017 Sprite Y Expansion
 * D018 Memory Pointers
 * D019 Interrupt Register
 * D01A Interrupt Enabled
 * D01B Sprite priority
 * D01C Sprite Multicolor
 * D01D Sprite X expansion
 * D01E Sprite-sprite collision
 * D01F Sprite-data collision
 * D020 Border Color
 * D021-D024 Background color 0-3
 * D025 Sprite multicolor 0
 * D026 Sprite multicolor 1
 * D026-D02E Sprite color
 * mirror D040-D3FF = D000..D003F
 */