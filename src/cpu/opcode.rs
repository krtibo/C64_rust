#![allow(non_snake_case)]
#![allow(unused)]

use super::cpu::MOS6510;

pub struct Opcode {
    pub table : [fn(&mut Opcode, &mut MOS6510) -> u8; 256],
    pub recent_opc : Vec<String>,
}

impl Opcode {

    pub fn new() -> Opcode {
        Opcode {
            table : [Opcode::nop; 256],
            recent_opc : Vec::new(),
        }
    }

    pub fn nop(&mut self, cpu : &mut MOS6510) -> u8 {
        self.recent_opc.push(String::from("NOP"));
        4
    }

    // pub fn fetch(&mut self, cpu : &mut CPU) -> u8 {
    //     let ret = cpu.RAM[(cpu.PC) as usize];
    //     cpu.PC += 1;
    //     ret
    // }
    //
    //
    // pub fn execute(&mut self, cpu : &mut CPU) -> u8 {
    //     self.last_opcode = self.fetch(cpu);
    //     self.opc[self.last_opcode as usize](self, cpu)
    // }

}
