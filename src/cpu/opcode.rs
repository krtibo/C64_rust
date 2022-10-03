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
        self.table[0x00] = Opcode::brk_00;
        self.table[0x08] = Opcode::php_08;
        self.table[0x0a] = Opcode::asl_a_0a;
        self.table[0x18] = Opcode::clc_18;
        self.table[0x28] = Opcode::plp_28;
        self.table[0x38] = Opcode::sec_38;
        self.table[0x48] = Opcode::pha_48;
        self.table[0x4a] = Opcode::lsr_a_4a;
        self.table[0x58] = Opcode::cli_58;
        self.table[0x60] = Opcode::rts_60;
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
        self.table[0xea] = Opcode::nop_ea;
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
        } else {
            // TODO: take this out
            cpu.PC = 0xA000;
        }
        ret
    }
    
    pub fn execute(&mut self, cpu : &mut MOS6510) {
        let current_opcode: u8 = self.fetch(cpu);
        self.current_operation = format!("{:02X} - ", current_opcode);
        self.table[current_opcode as usize](self, cpu)
    }
    
    pub fn brk_00(&mut self, cpu : &mut MOS6510) {
        self.current_operation.push_str("BRK");
        let pc: [u8; 2] = self.u16_to_u8s(cpu.PC + 1);
        cpu.push_on_stack(pc[0]);
        cpu.push_on_stack(pc[1]);
        cpu.push_on_stack(cpu.P);
        cpu.PC = 0xFFFE;
        cpu.set_flag(Flags::I, 1);
        cpu.cycle += 7;
    }

    pub fn php_08(&mut self, cpu : &mut MOS6510) {
        self.current_operation.push_str("PHP");
        cpu.push_on_stack(cpu.P);
        cpu.cycle += 3;
    }

    pub fn asl_a_0a(&mut self, cpu : &mut MOS6510) {
        self.current_operation.push_str("ASL A");
        let a_u16: u16 = (cpu.A << 1) as u16;
        let a_u8s: [u8; 2] = self.u16_to_u8s(a_u16);
        if a_u8s[0] == 0 { cpu.set_flag(Flags::C, 0) } else { cpu.set_flag(Flags::C, 1) };
        self.check_and_set_n(a_u8s[1], cpu);
        self.check_and_set_z(a_u8s[1], cpu);
        if a_u8s[1] == 0 { cpu.set_flag(Flags::C, cpu.A >> 7) }
        cpu.A = a_u8s[1];
        cpu.cycle += 2;
    }

    pub fn clc_18(&mut self, cpu : &mut MOS6510) {
        self.current_operation.push_str("CLC");
        cpu.set_flag(Flags::C, 0);
        cpu.cycle += 2;
    }

    pub fn plp_28(&mut self, cpu : &mut MOS6510) {
        self.current_operation.push_str("PLP");
        cpu.P = cpu.pull_from_stack();
        cpu.cycle += 4;
    }

    pub fn sec_38(&mut self, cpu : &mut MOS6510) {
        self.current_operation.push_str("SEC");
        cpu.set_flag(Flags::C, 1);
        cpu.cycle += 2;
    }

    pub fn pha_48(&mut self, cpu : &mut MOS6510) {
        self.current_operation.push_str("PHA");
        cpu.push_on_stack(cpu.A);
        cpu.cycle += 3;
    }

    pub fn lsr_a_4a(&mut self, cpu : &mut MOS6510) {
        self.current_operation.push_str("LSR A");
        cpu.set_flag(Flags::N, 0);
        cpu.set_flag(Flags::C, cpu.A & 0x01);
        cpu.A = (cpu.A & 0b1111_1110) >> 1;
        self.check_and_set_z(cpu.A, cpu);
        cpu.cycle += 2;
    }

    pub fn cli_58(&mut self, cpu : &mut MOS6510) {
        self.current_operation.push_str("CLI");
        cpu.set_flag(Flags::I, 0);
        cpu.cycle += 2;
    }

    pub fn rts_60(&mut self, cpu : &mut MOS6510) {
        self.current_operation.push_str("RTS");
        let stack: u16 = cpu.stack_addr();
        let pc_low = cpu.mmu.read(stack);
        let pc_high = cpu.mmu.read(stack - 1);
        cpu.PC = self.u8s_to_u16(pc_high, pc_low) + 1;
        cpu.S.wrapping_add(2);
        cpu.cycle += 4;
    }

    pub fn pla_68(&mut self, cpu : &mut MOS6510) {
        self.current_operation.push_str("PLA");
        cpu.A = cpu.pull_from_stack();
        self.check_and_set_n(cpu.A, cpu);
        self.check_and_set_z(cpu.A, cpu);
        cpu.cycle += 4;
    }

    pub fn sei_78(&mut self, cpu : &mut MOS6510) {
        self.current_operation.push_str("SEI");
        cpu.set_flag(Flags::I, 1);
        cpu.cycle += 2;
    }

    pub fn dey_88(&mut self, cpu : &mut MOS6510) {
        self.current_operation.push_str("DEY");
        cpu.Y = cpu.Y.wrapping_sub(1);
        self.check_and_set_n(cpu.Y, cpu);
        self.check_and_set_z(cpu.Y, cpu);
        cpu.cycle += 2;
    }

    pub fn txa_8a(&mut self, cpu : &mut MOS6510) {
        self.current_operation.push_str("TXA");
        cpu.A = cpu.X;
        self.check_and_set_n(cpu.A, cpu);
        self.check_and_set_z(cpu.A, cpu);
        cpu.cycle += 2;
    }

    pub fn tya_98(&mut self, cpu : &mut MOS6510) {
        self.current_operation.push_str("TYA");
        cpu.A = cpu.Y;
        self.check_and_set_n(cpu.A, cpu);
        self.check_and_set_z(cpu.A, cpu);
        cpu.cycle += 2;
    }

    pub fn txs_9a(&mut self, cpu : &mut MOS6510) {
        self.current_operation.push_str("TXS");
        cpu.S = cpu.X;
        self.check_and_set_n(cpu.S, cpu);
        self.check_and_set_z(cpu.S, cpu);
        cpu.cycle += 2;
    }


    pub fn tay_a8(&mut self, cpu : &mut MOS6510) {
        self.current_operation.push_str("TAY");
        cpu.Y = cpu.A;
        self.check_and_set_n(cpu.Y, cpu);
        self.check_and_set_z(cpu.Y, cpu);
        cpu.cycle += 2;
    }

    pub fn tax_aa(&mut self, cpu : &mut MOS6510) {
        self.current_operation.push_str("TAX");
        cpu.X = cpu.A;
        self.check_and_set_n(cpu.X, cpu);
        self.check_and_set_z(cpu.X, cpu);
        cpu.cycle += 2;
    }

    pub fn clv_b8(&mut self, cpu : &mut MOS6510) {
        self.current_operation.push_str("CLV");
        cpu.set_flag(Flags::V, 0);
        cpu.cycle += 2;
    }

    pub fn tsx_ba(&mut self, cpu : &mut MOS6510) {
        self.current_operation.push_str("TSX");
        cpu.X = cpu.S;
        self.check_and_set_n(cpu.X, cpu);
        self.check_and_set_z(cpu.X, cpu);
        cpu.cycle += 2;
    }

    pub fn iny_c8(&mut self, cpu : &mut MOS6510) {
        self.current_operation.push_str("INY");
        cpu.Y = cpu.Y.wrapping_add(1);
        self.check_and_set_n(cpu.Y, cpu);
        self.check_and_set_z(cpu.Y, cpu);
        cpu.cycle += 2;
    }

    pub fn dex_ca(&mut self, cpu : &mut MOS6510) {
        self.current_operation.push_str("DEX");
        cpu.X -= 1;
        self.check_and_set_n(cpu.X, cpu);
        self.check_and_set_z(cpu.X, cpu);
        cpu.cycle += 2;
    }

    pub fn cld_d8(&mut self, cpu : &mut MOS6510) {
        self.current_operation.push_str("CLD");
        cpu.set_flag(Flags::D, 0);
        cpu.cycle += 2;
    }

    pub fn inx_e8(&mut self, cpu : &mut MOS6510) {
        self.current_operation.push_str("INX");
        cpu.X = cpu.X.wrapping_add(1);
        self.check_and_set_n(cpu.X, cpu);
        self.check_and_set_z(cpu.X, cpu);
        cpu.cycle += 2;
    }

    pub fn nop_ea(&mut self, cpu : &mut MOS6510) {
        self.current_operation.push_str("NOP");
        cpu.cycle += 2;
    }

    pub fn sed_f8(&mut self, cpu : &mut MOS6510) {
        self.current_operation.push_str("SED");
        cpu.set_flag(Flags::D, 1);
        cpu.cycle += 2;
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

    pub fn u8s_to_u16(&mut self, high : u8, low : u8) -> u16 {
        return (((high as u16) << 8) | (low as u16)) as u16;
    }

    pub fn u16_to_u8s(&mut self, value : u16) -> [u8; 2] {
        let mut u8s: [u8; 2] = [0; 2];
        u8s[0] = (value << 8) as u8;        // high byte
        u8s[1] = (value & 0x00FF) as u8;    // low byte
        u8s
    }

}
