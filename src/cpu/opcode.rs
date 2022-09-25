// https://www.pagetable.com/c64ref/6502/

#![allow(non_snake_case)]
#![allow(unused)]

use super::cpu::MOS6510;
use super::cpu::Flags;

pub struct Opcode {
    pub table : [fn(&mut Opcode, &mut MOS6510); 256],
    pub current_operation : String,
}

impl Opcode {

    pub fn new() -> Opcode {
        Opcode {
            table : [Opcode::unknown; 256],
            current_operation : String::from(""),
        }
    }

    pub fn init(&mut self) {
        self.table[0xEA] = Opcode::nop_ea;
        self.table[0x18] = Opcode::clc_18;
        self.table[0x38] = Opcode::sec_38;
        self.table[0x58] = Opcode::cli_58;
        self.table[0x78] = Opcode::sei_78;
        self.table[0xb8] = Opcode::clv_b8;
        self.table[0xd8] = Opcode::cld_d8;
        self.table[0xf8] = Opcode::sed_f8;
    }

    pub fn unknown(&mut self, cpu : &mut MOS6510) {
        self.current_operation.push_str("/// Unknown/unimplemented opcode.");
        cpu.cycle += 4;
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
        self.current_operation = format!("{:02X} - ", current_opcode);
        self.table[current_opcode as usize](self, cpu)
    }

    pub fn nop_ea(&mut self, cpu : &mut MOS6510) {
        self.current_operation = String::from("NOP");
        cpu.cycle += 2;
    }

    pub fn clc_18(&mut self, cpu : &mut MOS6510) {
        self.current_operation = String::from("CLC");
        cpu.set_flag(Flags::C, 0);
        cpu.cycle += 2;
    }

    pub fn cld_d8(&mut self, cpu : &mut MOS6510) {
        self.current_operation = String::from("CLD");
        cpu.set_flag(Flags::D, 0);
        cpu.cycle += 2;
    }

    pub fn cli_58(&mut self, cpu : &mut MOS6510) {
        self.current_operation = String::from("CLI");
        cpu.set_flag(Flags::I, 0);
        cpu.cycle += 2;
    }

    pub fn clv_b8(&mut self, cpu : &mut MOS6510) {
        self.current_operation = String::from("CLV");
        cpu.set_flag(Flags::V, 0);
        cpu.cycle += 2;
    }

    pub fn sec_38(&mut self, cpu : &mut MOS6510) {
        self.current_operation = String::from("SEC");
        cpu.set_flag(Flags::C, 1);
        cpu.cycle += 2;
    }

    pub fn sed_f8(&mut self, cpu : &mut MOS6510) {
        self.current_operation = String::from("SED");
        cpu.set_flag(Flags::D, 1);
        cpu.cycle += 2;
    }

    pub fn sei_78(&mut self, cpu : &mut MOS6510) {
        self.current_operation = String::from("SEI");
        cpu.set_flag(Flags::I, 1);
        cpu.cycle += 2;
    }

}
