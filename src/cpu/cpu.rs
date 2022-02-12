// Register set source:
// http://codebase64.org/doku.php?id=base:6502_registers

#![allow(non_snake_case)]
#![allow(unused)]

use super::mmu::MMU;
use super::debugger::Debugger;
use super::opcode::Opcode;
use super::ppu::PPU;
use std::fs::File;
use std::io::Read;
use std::{thread, time};

pub struct MOS6510 {

    pub A  : u8,
    pub X  : u8,
    pub Y  : u8,
    pub S  : u8,
    pub PC : u16,
    pub P  : u8, // flags: N V 1 B D I Z C

    //  N - Negative flag
    //  V - oVerflow flag
    //  1 - unused flag, always 1
    //  B - Break flag
    //  D - Decimal mode flag
    //  I - Interrupt disable flag
    //  Z - Zero flag
    //  C - Carry flag
    //  initial value: 0010_0100

     pub mmu : MMU,
     pub opc : Opcode,
}

impl MOS6510 {
    pub fn new() -> MOS6510 {
        MOS6510 {
            A   : 0x00,
            X   : 0x00,
            Y   : 0x00,
            S   : 0xFF, // not sure about this one
            PC  : 0x0000,
            P   : 0b0010_0100,
            mmu : MMU::new(),
            opc : Opcode::new(),
        }
    }

    pub fn cycle(&mut self) {
        let mut dbg: Debugger = Debugger::new();
        let mut ppu: PPU = PPU::new();
        loop {
            dbg.poll();
            if (!ppu.poll()) { return }
            
            // let's do the trusty old fetch-decode-execute steps
            
            if self.PC < 255 {
                self.PC+=1;
            }


            // ppu rendering
            ppu.clear();
            ppu.render(self);
        
            // debugger initialization
            // it should be at the end of a cycle
            dbg.clear();
            dbg.create_snapshot(format!("  0x{:02X}    {:X}", self.PC, self.mmu.read(self.PC)), self);
            dbg.render_instructions();
            dbg.render_registers();

            // self.mmu.randomize();
            dbg.memory_map(&self.mmu.RAM);
            dbg.render();
        }
    } // cycle

    pub fn init(&mut self) {
        // loading basic ROM to A000 - BFFF: 8k
        self.mmu.copy_file_to_ram("/Users/rustboi/MEGASync/PROGRAMMING/C64_rust/rom/basic.rom", 0xA000);
        // loading charset to D000 - DFFF: 4k
        self.mmu.copy_file_to_ram("/Users/rustboi/MEGASync/PROGRAMMING/C64_rust/rom/character.rom", 0xD000);
        // loading kernal ROM to E000 - FFFF: 8k
        self.mmu.copy_file_to_ram("/Users/rustboi/MEGASync/PROGRAMMING/C64_rust/rom/kernal.rom", 0xE000);

        // the program counter is loaded with the value at FFFC-FFFD, Default: $FCE2.
        self.PC = (self.mmu.read(0xFFFD) as u16) << 8 | self.mmu.read(0xFFFC) as u16;
    }

    // pub fn byte_cat(self, h : u8, l : u8) -> u16 {
    //     let high : u16 = (h as u16) << 8;
    //     let low : u16 = l as u16;
    //     (high | low) as u16
    // }
}
