// Register set source:
// http://codebase64.org/doku.php?id=base:6502_registers

#![allow(non_snake_case)]
#![allow(unused)]

use super::mmu::MMU;
use super::debugger::debugger::Debugger;
use super::Opcode;
use super::ppu::PPU;
use std::fs::File;
use std::io::Read;
use std::{thread, time};
use std::time::{Duration, Instant};

const STACK: u16 = 0x0100;

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

pub enum Flags {
    N, V, Unused, B, D, I, Z, C
}

impl MOS6510 {
    pub fn new() -> MOS6510 {
        MOS6510 {
            A   : 0x00,
            X   : 0x00,
            Y   : 0x00,
            S   : 0xFF, // not sure about this one
            PC  : 0xFCE2,
            P   : 0b0010_0100,
            mmu : MMU::new(),
            cycle : 0, // 19656 is a raster cycle
        }
    }

    pub fn cycle(&mut self) {
        let mut dbg: Debugger = Debugger::new(0.5);
        dbg.init();
        // let mut ppu: PPU = PPU::new();
        let mut opc: Opcode = Opcode::new();
        opc.init();
        let mut start = Instant::now();
        loop {
            // CHECK FOR EVENTS
            // if (!ppu.poll()) { return }
            
            // FETCHING
            // this will fetch a byte from the memory where the PC is then execute it
            opc.execute(self);
            
            
            // PPU RENDER
            // if the cycle reaches 19656 a ppu render should occur
            if (self.cycle >= 19656) {
                self.cycle = 0;
                let duration = start.elapsed();
                start = Instant::now();
                let fps: f32 = (1.0/(duration.as_millis()) as f32 * 1000.0);
                //thread::sleep(time::Duration::from_millis(1000));
                // render ppu
                // ppu.clear();
                // ppu.render(self, fps);
                dbg.create_snapshot(format!("=================== PPU RENDER ==================="), self);
            }

            // DEBUGGER PAUSE LOOP
            let dbg_event = dbg.poll();
            if (dbg_event == dbg.events.PAUSE) {
                loop {
                    dbg.clear();
                    dbg.render(&self.mmu.RAM);
                    let dbg_event_stopped = dbg.poll();
                    if dbg_event_stopped == dbg.events.PAUSE { break }
                    // if (!ppu.poll()) { return }
                    if dbg_event_stopped == dbg.events.EXIT { return }
                }
            } 

            // STATE RESET
            if (dbg_event == dbg.events.RESET) {
                self.reset();
            }

            if(dbg_event == dbg.events.EXIT) { return }
            
            // DEBUGGER RENDER
            dbg.clear();
            dbg.create_snapshot(format!("  0x{:04X}   {}", self.PC, opc.current_operation), self);
            dbg.render(&self.mmu.RAM);
            // WHOA SLOW DOWN
            thread::sleep(time::Duration::from_millis(100));
        }
    } // cycle

    pub fn init(&mut self) {
        // loading basic ROM to A000 - BFFF: 8k
        self.mmu.copy_file_to_ram("/Users/krtibo/MEGA/PROGRAMMING/C64_rust/rom/basic.rom", 0xA000);
        // loading charset to D000 - DFFF: 4k
        self.mmu.copy_file_to_ram("/Users/krtibo/MEGA/PROGRAMMING/C64_rust/rom/character.rom", 0xD000);
        // loading kernal ROM to E000 - FFFF: 8k
        self.mmu.copy_file_to_ram("/Users/krtibo/MEGA/PROGRAMMING/C64_rust/rom/kernal.rom", 0xE000);
        self.mmu.init();

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

    // flags: N V 1 B D I Z C
    pub fn set_flag(&mut self, flag : Flags, value: u8) {
        match flag {
            Flags::N       => if value == 1 { self.P |= (value << 7) } else { self.P &= !(1 << 7) },
            Flags::V       => if value == 1 { self.P |= (value << 6) } else { self.P &= !(1 << 6) },
            Flags::Unused  => if value == 1 { self.P |= (value << 5) } else { self.P &= !(1 << 5) },
            Flags::B       => if value == 1 { self.P |= (value << 4) } else { self.P &= !(1 << 4) },
            Flags::D       => if value == 1 { self.P |= (value << 3) } else { self.P &= !(1 << 3) },
            Flags::I       => if value == 1 { self.P |= (value << 2) } else { self.P &= !(1 << 2) },
            Flags::Z       => if value == 1 { self.P |= (value << 1) } else { self.P &= !(1 << 1) },
            Flags::C       => if value == 1 { self.P |= value } else { self.P &= !1 },
        }
    }

    pub fn get_flag(&mut self, flag : Flags) -> bool {
        match flag {
            Flags::N       => return (self.P & (1 << 7)) > 0,
            Flags::V       => return (self.P & (1 << 6)) > 0,
            Flags::Unused  => return (self.P & (1 << 5)) > 0,
            Flags::B       => return (self.P & (1 << 4)) > 0,
            Flags::D       => return (self.P & (1 << 3)) > 0,
            Flags::I       => return (self.P & (1 << 2)) > 0,
            Flags::Z       => return (self.P & (1 << 1)) > 0,
            Flags::C       => return (self.P & 1) > 0,
        }
    }

    pub fn stack_addr(&mut self) -> u16 {
        STACK + self.S as u16
    }

    pub fn push_on_stack(&mut self, value : u8) {
        let stack: u16 = self.stack_addr();
        self.mmu.write(value, stack);
        self.S -= 1;
        // self.S = self.S.wrapping_sub(1);
    }

    pub fn pull_from_stack(&mut self) -> u8 {
        self.S += 1;
        // self.S = self.S.wrapping_add(1);
        let stack: u16 = self.stack_addr();
        self.mmu.read(stack)
    }
}
