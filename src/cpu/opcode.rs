// https://www.pagetable.com/c64ref/6502/

#![allow(non_snake_case)]
#![allow(unused)]

use super::cpu::MOS6510;

pub struct Opcode {
    pub table : [fn(&mut Opcode, &mut MOS6510); 256],
    pub current_operation : String
}

impl Opcode {

    pub fn new() -> Opcode {
        Opcode {
            table : [Opcode::unknown; 256],
            current_operation : String::from("")
        }
    }

    pub fn init(&mut self) {
        self.table[0xEA] = Opcode::nop_EA;
    }

    pub fn unknown(&mut self, cpu : &mut MOS6510) {
        self.current_operation.push_str(" - Unknown/unimplemented opcode.");
        cpu.cycle += 4;
    }

    pub fn nop_EA(&mut self, cpu : &mut MOS6510) {
        self.current_operation = String::from("NOP");
        cpu.cycle += 2;
    }

    pub fn fetch(&mut self, cpu : &mut MOS6510) -> u8 {
        let ret = cpu.mmu.read(cpu.PC);
        if (cpu.PC < 65535) {
            cpu.PC += 1;
        }
        ret
    }
    
    pub fn execute(&mut self, cpu : &mut MOS6510) {
        let current_opcode: u8 = self.fetch(cpu);
        self.current_operation = format!("{:02X}", current_opcode);
        self.table[current_opcode as usize](self, cpu)
    }

}
