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
        self.table[0x08] = Opcode::php_08;
        self.table[0x18] = Opcode::clc_18;
        self.table[0x28] = Opcode::plp_28;
        self.table[0x38] = Opcode::sec_38;
        self.table[0x48] = Opcode::pha_48;
        self.table[0x58] = Opcode::cli_58;
        self.table[0x68] = Opcode::pla_68;
        self.table[0x78] = Opcode::sei_78;
        self.table[0x88] = Opcode::dey_88;
        self.table[0x8a] = Opcode::txa_8a;
        self.table[0x98] = Opcode::tya_98;
        self.table[0x9a] = Opcode::txs_9a;
        self.table[0xa8] = Opcode::tay_a8;
        self.table[0xaa] = Opcode::tax_aa;
        self.table[0xb8] = Opcode::clv_b8;
        self.table[0xba] = Opcode::tsx_ba;
        self.table[0xc8] = Opcode::iny_c8;
        self.table[0xca] = Opcode::dex_ca;
        self.table[0xd8] = Opcode::cld_d8;
        self.table[0xe8] = Opcode::inx_e8;
        self.table[0xEA] = Opcode::nop_ea;
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

    pub fn dex_ca(&mut self, cpu : &mut MOS6510) {
        self.current_operation = String::from("DEX");
        cpu.X -= 1;
        self.check_and_set_n(cpu.X, cpu);
        self.check_and_set_z(cpu.X, cpu);
        cpu.cycle += 2;
    }

    pub fn dey_88(&mut self, cpu : &mut MOS6510) {
        self.current_operation = String::from("DEY");
        cpu.Y = cpu.Y.wrapping_sub(1);
        self.check_and_set_n(cpu.Y, cpu);
        self.check_and_set_z(cpu.Y, cpu);
        cpu.cycle += 2;
    }

    pub fn inx_e8(&mut self, cpu : &mut MOS6510) {
        self.current_operation = String::from("INX");
        cpu.X = cpu.X.wrapping_add(1);
        self.check_and_set_n(cpu.X, cpu);
        self.check_and_set_z(cpu.X, cpu);
        cpu.cycle += 2;
    }

    pub fn iny_c8(&mut self, cpu : &mut MOS6510) {
        self.current_operation = String::from("INY");
        cpu.Y = cpu.Y.wrapping_add(1);
        self.check_and_set_n(cpu.Y, cpu);
        self.check_and_set_z(cpu.Y, cpu);
        cpu.cycle += 2;
    }

    pub fn tax_aa(&mut self, cpu : &mut MOS6510) {
        self.current_operation = String::from("TAX");
        cpu.X = cpu.A;
        self.check_and_set_n(cpu.X, cpu);
        self.check_and_set_z(cpu.X, cpu);
        cpu.cycle += 2;
    }

    pub fn tay_a8(&mut self, cpu : &mut MOS6510) {
        self.current_operation = String::from("TAY");
        cpu.Y = cpu.A;
        self.check_and_set_n(cpu.Y, cpu);
        self.check_and_set_z(cpu.Y, cpu);
        cpu.cycle += 2;
    }

    pub fn tsx_ba(&mut self, cpu : &mut MOS6510) {
        self.current_operation = String::from("TSX");
        cpu.X = cpu.S;
        self.check_and_set_n(cpu.X, cpu);
        self.check_and_set_z(cpu.X, cpu);
        cpu.cycle += 2;
    }

    pub fn txa_8a(&mut self, cpu : &mut MOS6510) {
        self.current_operation = String::from("TXA");
        cpu.A = cpu.X;
        self.check_and_set_n(cpu.A, cpu);
        self.check_and_set_z(cpu.A, cpu);
        cpu.cycle += 2;
    }

    pub fn txs_9a(&mut self, cpu : &mut MOS6510) {
        self.current_operation = String::from("TXS");
        cpu.S = cpu.X;
        self.check_and_set_n(cpu.S, cpu);
        self.check_and_set_z(cpu.S, cpu);
        cpu.cycle += 2;
    }

    pub fn tya_98(&mut self, cpu : &mut MOS6510) {
        self.current_operation = String::from("TYA");
        cpu.A = cpu.Y;
        self.check_and_set_n(cpu.A, cpu);
        self.check_and_set_z(cpu.A, cpu);
        cpu.cycle += 2;
    }

    pub fn pha_48(&mut self, cpu : &mut MOS6510) {
        self.current_operation = String::from("PHA");
        cpu.push_on_stack(cpu.A);
        cpu.cycle += 3;
    }

    pub fn php_08(&mut self, cpu : &mut MOS6510) {
        self.current_operation = String::from("PHP");
        cpu.push_on_stack(cpu.P);
        cpu.cycle += 3;
    }

    pub fn pla_68(&mut self, cpu : &mut MOS6510) {
        self.current_operation = String::from("PLA");
        cpu.A = cpu.pull_from_stack();
        self.check_and_set_n(cpu.A, cpu);
        self.check_and_set_z(cpu.A, cpu);
        cpu.cycle += 4;
    }

    pub fn plp_28(&mut self, cpu : &mut MOS6510) {
        self.current_operation = String::from("PLP");
        cpu.P = cpu.pull_from_stack();
        cpu.cycle += 4;
    }

    // --- HELPER FUNCTIONS ---

    pub fn check_and_set_n(&mut self, value : u8, cpu : &mut MOS6510) {
        if (value & 0b1000_0000 == 0b1000_0000) { 
            cpu.set_flag(Flags::N, 1); 
        } else { 
            cpu.set_flag(Flags::N, 0);
        }
    }

    pub fn check_and_set_z(&mut self, value : u8, cpu : &mut MOS6510) {
        if (value == 0) {
            cpu.set_flag(Flags::Z, 1);
        } else {
            cpu.set_flag(Flags::Z, 0);
        }
    }

}
