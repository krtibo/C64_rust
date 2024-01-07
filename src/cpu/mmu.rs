// https://sta.c64.org/cbm64mem.html
#![allow(non_snake_case)]
#![allow(unused)]

extern crate rand;
use rand::Rng;
use std::fs::File;
use std::io::prelude::*;

const RAM_SIZE: usize = 65536;
const KERNAL_SIZE: usize = 8192;
const CHAR_SIZE: usize = 4096;
const IO_SIZE: usize = CHAR_SIZE;
const BASIC_SIZE: usize = 8192;
const KERNAL_OFFSET: usize = 0xE000;
const CHAR_OFFSET: usize = 0xD000;
const IO_OFFSET: usize = CHAR_OFFSET;
const BASIC_OFFSET: usize = 0xA000;

pub enum ROM {
    KERNAL, BASIC, CHAR
}

pub struct MMU {
    pub RAM: [u8; RAM_SIZE],
    pub KERNAL: [u8; KERNAL_SIZE],
    pub CHAR: [u8; CHAR_SIZE],
    pub IO: [u8; IO_SIZE],
    pub BASIC: [u8; BASIC_SIZE]
}

impl MMU {
    pub fn new() -> MMU {
        MMU {
            RAM: [0; RAM_SIZE],
            KERNAL: [0; KERNAL_SIZE],
            CHAR: [0; CHAR_SIZE],
            IO: [0; IO_SIZE],
            BASIC: [0; BASIC_SIZE],
        }
    }

    pub fn init(&mut self) {
        // initialize the default values of registers in memory
        self.RAM[0x0001] = 0b0000_0111;
    }

    pub fn write(&mut self, byte : u8, address : u16) {
        let _address: usize = address as usize;
        match address {
            0xA000..=0xBFFF =>
                if self.loram() && self.hiram() { self.BASIC[_address - BASIC_OFFSET] = byte; } else { self.RAM[_address] = byte; }
            0xD000..=0xDFFF =>
                if self.loram() || self.hiram() {
                    if self.charen() { self.IO[_address - IO_OFFSET] = byte; } else { self.CHAR[_address - CHAR_OFFSET] = byte; }
                } else { self.RAM[_address] = byte; }
            0xE000..=0xFFFF => if self.hiram() { self.KERNAL[_address - KERNAL_OFFSET] = byte; } else { self.RAM[_address] = byte; }
            _ => self.RAM[address as usize] = byte,
        }
    }

    pub fn load_rom(&mut self, path: &str, rom: ROM) {
        let mut buffer : Vec<u8> = Vec::new();

        // open the file from path
        let mut f = File::open(&path)
        .expect("\n Error with file loading! \n");

        // read the file to rom_buffer
        f.read_to_end(&mut buffer)
        .expect("Error with file reading!");

        // copy rom_buffer to RAM address
        for i in 0..buffer.len() {
            match rom {
                ROM::KERNAL => self.KERNAL[i] = buffer[i],
                ROM::BASIC => self.BASIC[i] = buffer[i],
                ROM::CHAR => self.CHAR[i] = buffer[i],
            }
        }
    }

    pub fn read(&mut self, address : u16) -> u8 {
        let _address: usize = address as usize;
        // println!("{:4x}", address);
        match address {
            0xA000..=0xBFFF =>
                if self.loram() && self.hiram() { self.BASIC[_address - BASIC_OFFSET] } else { self.RAM[_address] }
            0xD000..=0xDFFF =>
                if self.loram() || self.hiram() {
                    if self.charen() { self.IO[_address - IO_OFFSET] } else { self.CHAR[_address - CHAR_OFFSET] }
                } else { self.RAM[_address] }
            0xE000..=0xFFFF => if self.hiram() { self.KERNAL[_address - KERNAL_OFFSET] } else { self.RAM[_address] }
            _ => self.RAM[address as usize]
        }
    }

    pub fn clear(&mut self) {
        for i in 0..self.RAM.len() {
            self.RAM[i] = 0;
        }
    }

    pub fn loram(&mut self) -> bool {
        (self.RAM[0x0001] & 0b0000_0001) >= 1
    }

    pub fn hiram(&mut self) -> bool {
        (self.RAM[0x0001] & 0b0000_0010) >= 1
    }

    pub fn charen(&mut self) -> bool {
        (self.RAM[0x0001] & 0b0000_0100) >= 1
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