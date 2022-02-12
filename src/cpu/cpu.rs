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
use std::time::{Duration, Instant};

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
     pub cycle : u16,
}

impl MOS6510 {
    pub fn new() -> MOS6510 {
        MOS6510 {
            A   : 0x00,
            X   : 0x00,
            Y   : 0x00,
            S   : 0xFF, // not sure about this one
            PC  : 0xA000,
            P   : 0b0010_0100,
            mmu : MMU::new(),
            cycle : 0, // 19656 is a raster cycle
        }
    }

    pub fn cycle(&mut self) {
        let mut dbg: Debugger = Debugger::new();
        let mut ppu: PPU = PPU::new();
        let mut opc: Opcode = Opcode::new();
        opc.init();
        let mut start = Instant::now();
        loop {
            // CHECK FOR EVENTS
            if (!ppu.poll()) { return }
            
            // FETCHING
            // this will fetch a byte from the memory where the PC is then execute it
            opc.execute(self);
            
            
            // PPU RENDER
            // if the cycle reaches 19656 a ppu render should occur
            if (self.cycle >= 19656) {
                self.cycle = 0;
                let duration = start.elapsed();
                start = Instant::now();
                let fps: f32 = 1.0/(duration.as_micros() as f32/1000000.0);
                //thread::sleep(time::Duration::from_millis(1000));
                // render ppu
                ppu.clear();
                ppu.render(self, fps);
                dbg.create_snapshot(format!("=================== PPU RENDER ==================="), self);
            }

            // DEBUGGER PAUSE LOOP
            let dbg_event = dbg.poll();
            if (dbg_event == dbg.events.PAUSE) {
                loop {
                    dbg.clear();
                    dbg.render(&self.mmu.RAM);
                    if (dbg.poll() == dbg.events.PAUSE) { break; }
                    if (!ppu.poll()) { return }
                }
            } 

            // STATE RESET
            if (dbg_event == dbg.events.RESET) {
                self.reset();
            }
            
            // DEBUGGER RENDER
            dbg.clear();
            dbg.create_snapshot(format!("  0x{:02X}   {}", self.PC, opc.current_operation), self);
            dbg.render(&self.mmu.RAM);
            // SLOW DOWN
            // thread::sleep(time::Duration::from_millis(100));

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
        // B
        // self.PC = (self.mmu.read(0xFFFD) as u16) << 8 | self.mmu.read(0xFFFC) as u16;
    } // init

    pub fn reset(&mut self) {
        self.A = 0x00;
        self.X = 0x00;
        self.Y = 0x00;
        self.S = 0xFF;
        self.P = 0b0010_0100;
        self.cycle = 0;
        self.mmu.clear();
        self.init();
    }

    // pub fn byte_cat(self, h : u8, l : u8) -> u16 {
    //     let high : u16 = (h as u16) << 8;
    //     let low : u16 = l as u16;
    //     (high | low) as u16
    // }
}
