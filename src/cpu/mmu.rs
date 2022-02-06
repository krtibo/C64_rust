#![allow(non_snake_case)]
#![allow(unused)]

extern crate rand;
use rand::Rng;

pub struct MMU {
    pub RAM : [u8; 65536],
}

impl MMU {
    pub fn new() -> MMU {
        MMU {
            RAM : [0; 65536],
        }
    }

    pub fn write(&mut self, byte : u8, address : u16) {
        self.RAM[address as usize] = byte;
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
}
