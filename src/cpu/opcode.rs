// https://www.pagetable.com/c64ref/6502/

#![allow(non_snake_case)]
#![allow(unused)]

use super::cpu::MOS6510;
use super::cpu::Flags;

pub struct AddrReturn {
    pub operand : u8,
    pub address : u16,
    pub high    : Option<u8>,
    pub low     : u8,
}

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
        self.table[0x01] = Opcode::ora_01;
        self.table[0x05] = Opcode::ora_05;
        self.table[0x06] = Opcode::asl_06;
        self.table[0x08] = Opcode::php_08;
        self.table[0x09] = Opcode::ora_09;
        self.table[0x0a] = Opcode::asl_0a;
        self.table[0x0e] = Opcode::asl_0e;
        self.table[0x0d] = Opcode::ora_0d;
        self.table[0x11] = Opcode::ora_11;
        self.table[0x15] = Opcode::ora_15;
        self.table[0x16] = Opcode::asl_16;
        self.table[0x18] = Opcode::clc_18;
        self.table[0x19] = Opcode::ora_19;
        self.table[0x1d] = Opcode::ora_1d;
        self.table[0x1e] = Opcode::asl_1e;
        self.table[0x20] = Opcode::jsr_20;
        self.table[0x21] = Opcode::and_21;
        self.table[0x25] = Opcode::and_25;
        self.table[0x26] = Opcode::rol_26;
        self.table[0x28] = Opcode::plp_28;
        self.table[0x29] = Opcode::and_29;
        self.table[0x2a] = Opcode::rol_2a;
        self.table[0x2d] = Opcode::and_2d;
        self.table[0x2e] = Opcode::rol_2e;
        self.table[0x31] = Opcode::and_31;
        self.table[0x35] = Opcode::and_35;
        self.table[0x36] = Opcode::rol_36;
        self.table[0x38] = Opcode::sec_38;
        self.table[0x39] = Opcode::and_39;
        self.table[0x3d] = Opcode::and_3d;
        self.table[0x3e] = Opcode::rol_3e;
        self.table[0x40] = Opcode::rti_40;
        self.table[0x41] = Opcode::eor_41;
        self.table[0x45] = Opcode::eor_45;
        self.table[0x46] = Opcode::lsr_46;
        self.table[0x48] = Opcode::pha_48;
        self.table[0x49] = Opcode::eor_49;
        self.table[0x4a] = Opcode::lsr_4a;
        self.table[0x4c] = Opcode::jmp_4c;
        self.table[0x4d] = Opcode::eor_4d;
        self.table[0x4e] = Opcode::lsr_4e;
        self.table[0x51] = Opcode::eor_51;
        self.table[0x55] = Opcode::eor_55;
        self.table[0x56] = Opcode::lsr_56;
        self.table[0x58] = Opcode::cli_58;
        self.table[0x59] = Opcode::eor_59;
        self.table[0x5e] = Opcode::lsr_5e;
        self.table[0x5d] = Opcode::eor_5d;
        self.table[0x60] = Opcode::rts_60;
        self.table[0x66] = Opcode::ror_66;
        self.table[0x68] = Opcode::pla_68;
        self.table[0x6a] = Opcode::ror_6a;
        self.table[0x6c] = Opcode::jmp_6c;
        self.table[0x6e] = Opcode::ror_6e;
        self.table[0x76] = Opcode::ror_76;
        self.table[0x78] = Opcode::sei_78;
        self.table[0x7e] = Opcode::ror_7e;
        self.table[0x81] = Opcode::sta_81;
        self.table[0x84] = Opcode::sty_84;
        self.table[0x85] = Opcode::sta_85;
        self.table[0x86] = Opcode::stx_86;
        self.table[0x88] = Opcode::dey_88;
        self.table[0x8a] = Opcode::txa_8a;
        self.table[0x8c] = Opcode::sty_8c;
        self.table[0x8d] = Opcode::sta_8d;
        self.table[0x8e] = Opcode::stx_8e;
        self.table[0x91] = Opcode::sta_91;
        self.table[0x94] = Opcode::sty_94;
        self.table[0x95] = Opcode::sta_95;
        self.table[0x96] = Opcode::stx_96;
        self.table[0x98] = Opcode::tya_98;
        self.table[0x99] = Opcode::sta_99;
        self.table[0x9a] = Opcode::txs_9a;
        self.table[0x9d] = Opcode::sta_9d;
        self.table[0xa0] = Opcode::ldy_a0;
        self.table[0xa1] = Opcode::lda_a1;
        self.table[0xa2] = Opcode::ldx_a2;
        self.table[0xa4] = Opcode::ldy_a4;
        self.table[0xa5] = Opcode::lda_a5;
        self.table[0xa6] = Opcode::ldx_a6;
        self.table[0xa8] = Opcode::tay_a8;
        self.table[0xa9] = Opcode::lda_a9;
        self.table[0xaa] = Opcode::tax_aa;
        self.table[0xac] = Opcode::ldy_ac;
        self.table[0xad] = Opcode::lda_ad;
        self.table[0xae] = Opcode::ldx_ae;
        self.table[0xb1] = Opcode::lda_b1;
        self.table[0xb4] = Opcode::ldy_b4;
        self.table[0xb5] = Opcode::lda_b5;
        self.table[0xb6] = Opcode::ldx_b6;
        self.table[0xb8] = Opcode::clv_b8;
        self.table[0xb9] = Opcode::lda_b9;
        self.table[0xba] = Opcode::tsx_ba;
        self.table[0xbc] = Opcode::ldy_bc;
        self.table[0xbd] = Opcode::lda_bd;
        self.table[0xbe] = Opcode::ldx_be;
        self.table[0xc0] = Opcode::cpy_c0;
        self.table[0xc1] = Opcode::cmp_c1;
        self.table[0xc4] = Opcode::cpy_c4;
        self.table[0xc5] = Opcode::cmp_c5;
        self.table[0xc6] = Opcode::dec_c6;
        self.table[0xc8] = Opcode::iny_c8;
        self.table[0xc9] = Opcode::cmp_c9;
        self.table[0xca] = Opcode::dex_ca;
        self.table[0xcc] = Opcode::cpy_cc;
        self.table[0xcd] = Opcode::cmp_cd;
        self.table[0xce] = Opcode::dec_ce;
        self.table[0xd1] = Opcode::cmp_d1;
        self.table[0xd5] = Opcode::cmp_d5;
        self.table[0xd6] = Opcode::dec_d6;
        self.table[0xd8] = Opcode::cld_d8;
        self.table[0xd9] = Opcode::cmp_d9;
        self.table[0xdd] = Opcode::cmp_dd;
        self.table[0xde] = Opcode::dec_de;
        self.table[0xe0] = Opcode::cpx_e0;
        self.table[0xe4] = Opcode::cpx_e4;
        self.table[0xe6] = Opcode::inc_e6;
        self.table[0xe8] = Opcode::inx_e8;
        self.table[0xea] = Opcode::nop_ea;
        self.table[0xec] = Opcode::cpx_ec;
        self.table[0xee] = Opcode::inc_ee;
        self.table[0xf6] = Opcode::inc_f6;
        self.table[0xf8] = Opcode::sed_f8;
        self.table[0xfe] = Opcode::inc_fe;
    }

    pub fn unknown(&mut self, cpu : &mut MOS6510) {
        self.current_operation.push_str("////////////// n/a");
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

    pub fn lda_a9(&mut self, cpu : &mut MOS6510) {
        let operand: u8 = self.fetch(cpu);
		self.current_operation.push_str(format!("LDA #${:02X}", operand).as_str());
        cpu.A = operand;
        self.check_and_set_n(cpu.A, cpu);
        self.check_and_set_z(cpu.A, cpu);
        cpu.cycle += 2;
    }

    pub fn lda_ad(&mut self, cpu : &mut MOS6510) {
        let AddrReturn { operand, address, high, low } = self.absolute(cpu);
		self.current_operation.push_str(format!("LDA ${:02X}{:02X}", high.unwrap(), low).as_str());
        cpu.A = operand;
        self.check_and_set_n(cpu.A, cpu);
        self.check_and_set_z(cpu.A, cpu);
        cpu.cycle += 4;
    }

    pub fn lda_bd(&mut self, cpu : &mut MOS6510) {
        let AddrReturn { operand, address, high, low } = self.absolute_indexed(cpu.X as u16, cpu);
		self.current_operation.push_str(format!("LDA ${:02X}{:02X}, X", high.unwrap(), low).as_str());
        cpu.A = operand;
        self.check_and_set_n(cpu.A, cpu);
        self.check_and_set_z(cpu.A, cpu);
        cpu.cycle += 4;
        // TODO: cycle is 4+1 if page is crossed
    }

    pub fn lda_b9(&mut self, cpu : &mut MOS6510) {
        let AddrReturn { operand, address, high, low } = self.absolute_indexed(cpu.Y as u16, cpu);
		self.current_operation.push_str(format!("LDA ${:02X}{:02X}, Y", high.unwrap(), low).as_str());
        cpu.A = operand;
        self.check_and_set_n(cpu.A, cpu);
        self.check_and_set_z(cpu.A, cpu);
        cpu.cycle += 4;
        // TODO: cycle is 4+1 if page is crossed
    }

    pub fn lda_a5(&mut self, cpu : &mut MOS6510) {
        let AddrReturn { operand, address, high, low } = self.zero_page(cpu);
		self.current_operation.push_str(format!("LDA ${:02X}", low).as_str());
        cpu.A = operand;    
        self.check_and_set_n(cpu.A, cpu);
        self.check_and_set_z(cpu.A, cpu);
        cpu.cycle += 3;
    }

    pub fn lda_b5(&mut self, cpu : &mut MOS6510) {
        let mut operand: u8 = self.fetch(cpu);
		self.current_operation.push_str(format!("LDA ${:02X}, X", operand).as_str());
        operand = operand.wrapping_add(cpu.X);
        let address: u16 = self.u8s_to_u16(0x00, operand);
        cpu.A = cpu.mmu.read(address);    
        self.check_and_set_n(cpu.A, cpu);
        self.check_and_set_z(cpu.A, cpu);
        cpu.cycle += 4;
    }

    pub fn lda_a1(&mut self, cpu : &mut MOS6510) {
        let operand: u8 = self.fetch(cpu);
		self.current_operation.push_str(format!("LDA (${:02X}, X)", operand).as_str());
        let low_address = operand.wrapping_add(cpu.X);
        let high_address = low_address.wrapping_add(1);
        let low: u8 = cpu.mmu.read(self.u8s_to_u16(0x00, low_address));
        let high: u8 = cpu.mmu.read(self.u8s_to_u16(0x00, high_address));
        cpu.A = cpu.mmu.read(self.u8s_to_u16(high, low));    
        self.check_and_set_n(cpu.A, cpu);
        self.check_and_set_z(cpu.A, cpu);
        cpu.cycle += 6;
    }

    pub fn lda_b1(&mut self, cpu : &mut MOS6510) {
        let low: u8 = self.fetch(cpu);
		self.current_operation.push_str(format!("LDA (${:02X}), Y", low).as_str());
        let high: u8 = low.wrapping_add(1);
        let low_address: u8 = cpu.mmu.read(self.u8s_to_u16(0x00, low));
        let high_address: u8 = cpu.mmu.read(self.u8s_to_u16(0x00, high));
        let address: u16 = self.u8s_to_u16(high_address, low_address) + cpu.Y as u16;
        cpu.A = cpu.mmu.read(address);
        self.check_and_set_n(cpu.A, cpu);
        self.check_and_set_z(cpu.A, cpu);
        cpu.cycle += 5;
        // TODO: cycle is 5+1 if page is crossed
    }

    pub fn ldx_a2(&mut self, cpu : &mut MOS6510) {
        let operand: u8 = self.fetch(cpu);
		self.current_operation.push_str(format!("LDX #${:02X}", operand).as_str());
        cpu.X = operand;
        self.check_and_set_n(cpu.X, cpu);
        self.check_and_set_z(cpu.X, cpu);
        cpu.cycle += 2;
    }

    pub fn ldx_ae(&mut self, cpu : &mut MOS6510) {
        let AddrReturn { operand, address, high, low } = self.absolute(cpu);
		self.current_operation.push_str(format!("LDX ${:02X}{:02X}", high.unwrap(), low).as_str());
        cpu.X = operand;
        self.check_and_set_n(cpu.X, cpu);
        self.check_and_set_z(cpu.X, cpu);
        cpu.cycle += 4;
    }

    pub fn ldx_be(&mut self, cpu : &mut MOS6510) {
        let AddrReturn { operand, address, high, low } = self.absolute_indexed(cpu.Y as u16, cpu);
		self.current_operation.push_str(format!("LDX ${:02X}{:02X}, Y", high.unwrap(), low).as_str());
        cpu.X = operand;
        self.check_and_set_n(cpu.X, cpu);
        self.check_and_set_z(cpu.X, cpu);
        cpu.cycle += 4;
        // TODO: cycle is 4+1 if page is crossed
    }

    pub fn ldx_a6(&mut self, cpu : &mut MOS6510) {
        let AddrReturn { operand, address, high, low } = self.zero_page(cpu);
		self.current_operation.push_str(format!("LDX ${:02X}", low).as_str());
        cpu.X = operand;    
        self.check_and_set_n(cpu.X, cpu);
        self.check_and_set_z(cpu.X, cpu);
        cpu.cycle += 3;
    }

    pub fn ldx_b6(&mut self, cpu : &mut MOS6510) {
        let mut operand: u8 = self.fetch(cpu);
		self.current_operation.push_str(format!("LDX ${:02X}, Y", operand).as_str());
        operand = operand.wrapping_add(cpu.Y);
        let address: u16 = self.u8s_to_u16(0x00, operand);
        cpu.X = cpu.mmu.read(address);
        self.check_and_set_n(cpu.X, cpu);
        self.check_and_set_z(cpu.X, cpu);
        cpu.cycle += 4;
    }

    pub fn ldy_a0(&mut self, cpu : &mut MOS6510) {
        let operand: u8 = self.fetch(cpu);
		self.current_operation.push_str(format!("LDY #${:02X}", operand).as_str());
        cpu.Y = operand;
        self.check_and_set_n(cpu.Y, cpu);
        self.check_and_set_z(cpu.Y, cpu);
        cpu.cycle += 2;
    }

    pub fn ldy_ac(&mut self, cpu : &mut MOS6510) {
        let AddrReturn { operand, address, high, low } = self.absolute(cpu);
		self.current_operation.push_str(format!("LDY ${:02X}{:02X}", high.unwrap(), low).as_str());
        cpu.Y = operand;
        self.check_and_set_n(cpu.Y, cpu);
        self.check_and_set_z(cpu.Y, cpu);
        cpu.cycle += 4;
    }

    pub fn ldy_bc(&mut self, cpu : &mut MOS6510) {
        let AddrReturn { operand, address, high, low } = self.absolute_indexed(cpu.X as u16, cpu);
		self.current_operation.push_str(format!("LDY ${:02X}{:02X}, X", high.unwrap(), low).as_str());
        cpu.Y = operand;
        self.check_and_set_n(cpu.Y, cpu);
        self.check_and_set_z(cpu.Y, cpu);
        cpu.cycle += 4;
        // TODO: cycle is 4+1 if page is crossed
    }

    pub fn ldy_a4(&mut self, cpu : &mut MOS6510) {
        let AddrReturn { operand, address, high, low } = self.zero_page(cpu);
		self.current_operation.push_str(format!("LDY ${:02X}", low).as_str());
        cpu.Y = operand;
        self.check_and_set_n(cpu.Y, cpu);
        self.check_and_set_z(cpu.Y, cpu);
        cpu.cycle += 3;
    }

    pub fn ldy_b4(&mut self, cpu : &mut MOS6510) {
        let mut operand: u8 = self.fetch(cpu);
		self.current_operation.push_str(format!("LDY ${:02X}, X", operand).as_str());
        operand = operand.wrapping_add(cpu.X);
        let address: u16 = self.u8s_to_u16(0x00, operand);
        cpu.Y = cpu.mmu.read(address);
        self.check_and_set_n(cpu.Y, cpu);
        self.check_and_set_z(cpu.Y, cpu);
        cpu.cycle += 4;
    }

    pub fn sta_8d(&mut self, cpu : &mut MOS6510) {
        let AddrReturn { operand, address, high, low } = self.absolute(cpu);
		self.current_operation.push_str(format!("STA ${:02X}{:02X}", high.unwrap(), low).as_str());
        cpu.mmu.write(cpu.A, address);
        cpu.cycle += 4;
    }

    pub fn sta_9d(&mut self, cpu : &mut MOS6510) {
        let AddrReturn { operand, address, high, low } = self.absolute_indexed(cpu.X as u16, cpu);
		self.current_operation.push_str(format!("STA ${:02X}{:02X}, X", high.unwrap(), low).as_str());
        cpu.mmu.write(cpu.A, address);
        cpu.cycle += 5;
    }

    pub fn sta_99(&mut self, cpu : &mut MOS6510) {
        let AddrReturn { operand, address, high, low } = self.absolute_indexed(cpu.Y as u16, cpu);
		self.current_operation.push_str(format!("STA ${:02X}{:02X}, Y", high.unwrap(), low).as_str());
        cpu.mmu.write(cpu.A, address);
        cpu.cycle += 5;
    }

    pub fn sta_85(&mut self, cpu : &mut MOS6510) {
        let AddrReturn { operand, address, high, low } = self.zero_page(cpu);
		self.current_operation.push_str(format!("STA ${:02X}", low).as_str());
        cpu.mmu.write(cpu.A, address);
        cpu.cycle += 3;
    }

    pub fn sta_95(&mut self, cpu : &mut MOS6510) {
        let mut operand: u8 = self.fetch(cpu);
		self.current_operation.push_str(format!("STA ${:02X}, X", operand).as_str());
        operand = operand.wrapping_add(cpu.X);
        let address: u16 = self.u8s_to_u16(0x00, operand);
        cpu.mmu.write(cpu.A, address);
        cpu.cycle += 4;
    }

    pub fn sta_81(&mut self, cpu : &mut MOS6510) {
        let operand: u8 = self.fetch(cpu);
		self.current_operation.push_str(format!("STA (${:02X}, X)", operand).as_str());
        let low_address = operand.wrapping_add(cpu.X);
        let high_address = low_address.wrapping_add(1);
        let low: u8 = cpu.mmu.read(self.u8s_to_u16(0x00, low_address));
        let high: u8 = cpu.mmu.read(self.u8s_to_u16(0x00, high_address));
        let address: u16 = self.u8s_to_u16(high, low);
        cpu.mmu.write(cpu.A, address);
        cpu.cycle += 6;
    }

    pub fn sta_91(&mut self, cpu : &mut MOS6510) {
        let mut low: u8 = self.fetch(cpu);
		self.current_operation.push_str(format!("STA (${:02X}), Y", low).as_str());
        let high: u8 = low.wrapping_add(1);
        let low_address: u8 = cpu.mmu.read(self.u8s_to_u16(0x00, low));
        let high_address: u8 = cpu.mmu.read(self.u8s_to_u16(0x00, high));
        let address: u16 = self.u8s_to_u16(high_address, low_address) + cpu.Y as u16;
        cpu.mmu.write(cpu.A, address);
        cpu.cycle += 6;
    }

    pub fn stx_8e(&mut self, cpu : &mut MOS6510) {
        let AddrReturn { operand, address, high, low } = self.absolute(cpu);
		self.current_operation.push_str(format!("STX ${:02X}{:02X}", high.unwrap(), low).as_str());
        cpu.mmu.write(cpu.X, address);
        cpu.cycle += 4;
    }

    pub fn stx_86(&mut self, cpu : &mut MOS6510) {
        let AddrReturn { operand, address, high, low } = self.zero_page(cpu);
		self.current_operation.push_str(format!("STX ${:02X}", low).as_str());
        cpu.mmu.write(cpu.X, address);
        cpu.cycle += 3;
    }

    pub fn stx_96(&mut self, cpu : &mut MOS6510) {
        let mut operand: u8 = self.fetch(cpu);
		self.current_operation.push_str(format!("STX ${:02X}, Y", operand).as_str());
        operand = operand.wrapping_add(cpu.Y);
        let address: u16 = self.u8s_to_u16(0x00, operand);
        cpu.mmu.write(cpu.X, address);
        cpu.cycle += 4;
    }

    pub fn sty_8c(&mut self, cpu : &mut MOS6510) {
        let AddrReturn { operand, address, high, low } = self.absolute(cpu);
		self.current_operation.push_str(format!("STY ${:02X}{:02X}", high.unwrap(), low).as_str());
        cpu.mmu.write(cpu.Y, address);
        cpu.cycle += 4;
    }

    pub fn sty_84(&mut self, cpu : &mut MOS6510) {
        let AddrReturn { operand, address, high, low } = self.zero_page(cpu);
		self.current_operation.push_str(format!("STY ${:02X}", low).as_str());
        cpu.mmu.write(cpu.Y, address);
        cpu.cycle += 3;
    }

    pub fn sty_94(&mut self, cpu : &mut MOS6510) {
        let mut operand: u8 = self.fetch(cpu);
		self.current_operation.push_str(format!("STY ${:02X}, X", operand).as_str());
        operand = operand.wrapping_add(cpu.X);
        let address: u16 = self.u8s_to_u16(0x00, operand);
        cpu.mmu.write(cpu.Y, address);
        cpu.cycle += 4;
    }

    pub fn cmp_c9(&mut self, cpu : &mut MOS6510) {
        let operand: u8 = self.fetch(cpu);
		self.current_operation.push_str(format!("CMP #${:02X}", operand).as_str());
        let result: u8 = cpu.A.wrapping_sub(operand);
        if result == 0 { cpu.set_flag(Flags::Z, 1) } else { cpu.set_flag(Flags::Z, 0) }
        if operand <= cpu.A { cpu.set_flag(Flags::C, 1) } else { cpu.set_flag(Flags::C, 0) }
        self.check_and_set_n(result, cpu);
        cpu.cycle += 2;
    }

    pub fn cmp_cd(&mut self, cpu : &mut MOS6510) {
        let AddrReturn { operand, address, high, low } = self.absolute(cpu);
		self.current_operation.push_str(format!("CMP ${:02X}{:02X}", high.unwrap(), low).as_str());
        let result: u8 = cpu.A.wrapping_sub(operand);
        if result == 0 { cpu.set_flag(Flags::Z, 1) } else { cpu.set_flag(Flags::Z, 0) }
        if operand <= cpu.A { cpu.set_flag(Flags::C, 1) } else { cpu.set_flag(Flags::C, 0) }
        self.check_and_set_n(result, cpu);
        cpu.cycle += 4;
    }

    pub fn cmp_dd(&mut self, cpu : &mut MOS6510) {
        let AddrReturn { operand, address, high, low } = self.absolute_indexed(cpu.X as u16, cpu);
		self.current_operation.push_str(format!("CMP ${:02X}{:02X}, X", high.unwrap(), low).as_str());
        let result: u8 = cpu.A.wrapping_sub(operand);
        if result == 0 { cpu.set_flag(Flags::Z, 1) } else { cpu.set_flag(Flags::Z, 0) }
        if operand <= cpu.A { cpu.set_flag(Flags::C, 1) } else { cpu.set_flag(Flags::C, 0) }
        self.check_and_set_n(result, cpu);
        cpu.cycle += 4;
    }

    pub fn cmp_d9(&mut self, cpu : &mut MOS6510) {
        let AddrReturn { operand, address, high, low } = self.absolute_indexed(cpu.Y as u16, cpu);
		self.current_operation.push_str(format!("CMP ${:02X}{:02X}, Y", high.unwrap(), low).as_str());
        let result: u8 = cpu.A.wrapping_sub(operand);
        if result == 0 { cpu.set_flag(Flags::Z, 1) } else { cpu.set_flag(Flags::Z, 0) }
        if operand <= cpu.A { cpu.set_flag(Flags::C, 1) } else { cpu.set_flag(Flags::C, 0) }
        self.check_and_set_n(result, cpu);
        cpu.cycle += 4;
    }

    pub fn cmp_c5(&mut self, cpu : &mut MOS6510) {
        let AddrReturn { operand, address, high, low } = self.zero_page(cpu);
		self.current_operation.push_str(format!("CMP ${:02X}", low).as_str());
        let result: u8 = cpu.A.wrapping_sub(operand);
        if result == 0 { cpu.set_flag(Flags::Z, 1) } else { cpu.set_flag(Flags::Z, 0) }
        if operand <= cpu.A { cpu.set_flag(Flags::C, 1) } else { cpu.set_flag(Flags::C, 0) }
        self.check_and_set_n(result, cpu);
        cpu.cycle += 3;
    }

    pub fn cmp_d5(&mut self, cpu : &mut MOS6510) {
        let address: u8 = self.fetch(cpu);
		self.current_operation.push_str(format!("CMP ${:02X}, X", address).as_str());
        let operand = cpu.mmu.read(self.u8s_to_u16(0x00, address.wrapping_add(cpu.X)));
        let result: u8 = cpu.A.wrapping_sub(operand);
        if result == 0 { cpu.set_flag(Flags::Z, 1) } else { cpu.set_flag(Flags::Z, 0) }
        if operand <= cpu.A { cpu.set_flag(Flags::C, 1) } else { cpu.set_flag(Flags::C, 0) }
        self.check_and_set_n(result, cpu);
        cpu.cycle += 4;
    }

    pub fn cmp_c1(&mut self, cpu : &mut MOS6510) {
        let operand: u8 = self.fetch(cpu);
		self.current_operation.push_str(format!("CMP (${:02X}, X)", operand).as_str());
        let low_address = operand.wrapping_add(cpu.X);
        let high_address = low_address.wrapping_add(1);
        let low: u8 = cpu.mmu.read(self.u8s_to_u16(0x00, low_address));
        let high: u8 = cpu.mmu.read(self.u8s_to_u16(0x00, high_address));
        let value = cpu.mmu.read(self.u8s_to_u16(high, low));
        let result: u8 = cpu.A.wrapping_sub(value);
        if result == 0 { cpu.set_flag(Flags::Z, 1) } else { cpu.set_flag(Flags::Z, 0) }
        if value <= cpu.A { cpu.set_flag(Flags::C, 1) } else { cpu.set_flag(Flags::C, 0) }
        self.check_and_set_n(value, cpu);
        cpu.cycle += 6;
    }

    pub fn cmp_d1(&mut self, cpu : &mut MOS6510) {
        let mut low: u8 = self.fetch(cpu);
		self.current_operation.push_str(format!("CMP (${:02X}), Y", low).as_str());
        let high: u8 = low.wrapping_add(1);
        let low_address: u8 = cpu.mmu.read(self.u8s_to_u16(0x00, low));
        let high_address: u8 = cpu.mmu.read(self.u8s_to_u16(0x00, high));
        let address: u16 = self.u8s_to_u16(high_address, low_address) + cpu.Y as u16;
        let value = cpu.mmu.read(address);
        let result: u8 = cpu.A.wrapping_sub(value);
        if result == 0 { cpu.set_flag(Flags::Z, 1) } else { cpu.set_flag(Flags::Z, 0) }
        if value <= cpu.A { cpu.set_flag(Flags::C, 1) } else { cpu.set_flag(Flags::C, 0) }
        self.check_and_set_n(value, cpu);
        cpu.cycle += 5;
    }

    pub fn dec_ce(&mut self, cpu : &mut MOS6510) {
        let AddrReturn { operand, address, high, low } = self.absolute(cpu);
		self.current_operation.push_str(format!("DEC ${:02X}{:02X}", high.unwrap(), low).as_str());
        let value = operand.wrapping_sub(1);
        cpu.mmu.write(value, address);
        self.check_and_set_n(value, cpu);
        self.check_and_set_z(value, cpu);
        cpu.cycle += 6;
    }

    pub fn dec_de(&mut self, cpu : &mut MOS6510) {
        let AddrReturn { operand, address, high, low } = self.absolute_indexed(cpu.X as u16, cpu);
		self.current_operation.push_str(format!("DEC ${:02X}{:02X}, X", high.unwrap(), low).as_str());
        let value = operand.wrapping_sub(1);
        cpu.mmu.write(value, address);
        self.check_and_set_n(value, cpu);
        self.check_and_set_z(value, cpu);
        cpu.cycle += 7;
    }

    pub fn dec_c6(&mut self, cpu : &mut MOS6510) {
        let AddrReturn { operand, address, high, low } = self.zero_page(cpu);
		self.current_operation.push_str(format!("DEC ${:02X}", low).as_str());
        let value = operand.wrapping_sub(1);
        cpu.mmu.write(value, address);
        self.check_and_set_n(value, cpu);
        self.check_and_set_z(value, cpu);
        cpu.cycle += 5;
    }

    pub fn dec_d6(&mut self, cpu : &mut MOS6510) {
        let operand: u8 = self.fetch(cpu);
		self.current_operation.push_str(format!("DEC ${:02X}, X", operand).as_str());
        let address = self.u8s_to_u16(0x00, operand.wrapping_add(cpu.X));
        let value = cpu.mmu.read(address).wrapping_sub(1);
        cpu.mmu.write(value, address);
        self.check_and_set_n(value, cpu);
        self.check_and_set_z(value, cpu);
        cpu.cycle += 6;
    }

    pub fn inc_ee(&mut self, cpu : &mut MOS6510) {
        let AddrReturn { operand, address, high, low } = self.absolute(cpu);
		self.current_operation.push_str(format!("INC ${:02X}{:02X}", high.unwrap(), low).as_str());
        let value = operand.wrapping_add(1);
        cpu.mmu.write(value, address);
        self.check_and_set_n(value, cpu);
        self.check_and_set_z(value, cpu);
        cpu.cycle += 6;
    }

    pub fn inc_fe(&mut self, cpu : &mut MOS6510) {
        let AddrReturn { operand, address, high, low } = self.absolute_indexed(cpu.X as u16, cpu);
		self.current_operation.push_str(format!("INC ${:02X}{:02X}, X", high.unwrap(), low).as_str());
        let value = operand.wrapping_add(1);
        cpu.mmu.write(value, address);
        self.check_and_set_n(value, cpu);
        self.check_and_set_z(value, cpu);
        cpu.cycle += 7;
    }

    pub fn inc_e6(&mut self, cpu : &mut MOS6510) {
        let AddrReturn { operand, address, high, low } = self.zero_page(cpu);
		self.current_operation.push_str(format!("INC ${:02X}", low).as_str());
        let value = operand.wrapping_add(1);
        cpu.mmu.write(value, address);
        self.check_and_set_n(value, cpu);
        self.check_and_set_z(value, cpu);
        cpu.cycle += 5;
    }

    pub fn inc_f6(&mut self, cpu : &mut MOS6510) {
        let operand: u8 = self.fetch(cpu);
		self.current_operation.push_str(format!("INC ${:02X}, X", operand).as_str());
        let address = self.u8s_to_u16(0x00, operand.wrapping_add(cpu.X));
        let value = cpu.mmu.read(address).wrapping_add(1);
        cpu.mmu.write(value, address);
        self.check_and_set_n(value, cpu);
        self.check_and_set_z(value, cpu);
        cpu.cycle += 6;
    }

    pub fn cpx_e0(&mut self, cpu : &mut MOS6510) {
        let operand: u8 = self.fetch(cpu);
		self.current_operation.push_str(format!("CPX #${:02X}", operand).as_str());
        let result: u8 = cpu.X.wrapping_sub(operand);
        if result == 0 { cpu.set_flag(Flags::Z, 1) } else { cpu.set_flag(Flags::Z, 0) }
        if operand <= cpu.X { cpu.set_flag(Flags::C, 1) } else { cpu.set_flag(Flags::C, 0) }
        self.check_and_set_n(result, cpu);
        cpu.cycle += 2;
    }

    pub fn cpx_ec(&mut self, cpu : &mut MOS6510) {
        let AddrReturn { operand, address, high, low } = self.absolute(cpu);
		self.current_operation.push_str(format!("CPX ${:02X}{:02X}", high.unwrap(), low).as_str());
        let result: u8 = cpu.X.wrapping_sub(operand);
        if result == 0 { cpu.set_flag(Flags::Z, 1) } else { cpu.set_flag(Flags::Z, 0) }
        if operand <= cpu.X { cpu.set_flag(Flags::C, 1) } else { cpu.set_flag(Flags::C, 0) }
        self.check_and_set_n(result, cpu);
        cpu.cycle += 4;
    }

    pub fn cpx_e4(&mut self, cpu : &mut MOS6510) {
        let AddrReturn { operand, address, high, low } = self.zero_page(cpu);
		self.current_operation.push_str(format!("CPX ${:02X}", low).as_str());
        let result: u8 = cpu.X.wrapping_sub(operand);
        if result == 0 { cpu.set_flag(Flags::Z, 1) } else { cpu.set_flag(Flags::Z, 0) }
        if operand <= cpu.X { cpu.set_flag(Flags::C, 1) } else { cpu.set_flag(Flags::C, 0) }
        self.check_and_set_n(result, cpu);
        cpu.cycle += 3;
    }

    pub fn cpy_c0(&mut self, cpu : &mut MOS6510) {
        let operand: u8 = self.fetch(cpu);
		self.current_operation.push_str(format!("CPY #${:02X}", operand).as_str());
        let result: u8 = cpu.Y.wrapping_sub(operand);
        if result == 0 { cpu.set_flag(Flags::Z, 1) } else { cpu.set_flag(Flags::Z, 0) }
        if operand <= cpu.Y { cpu.set_flag(Flags::C, 1) } else { cpu.set_flag(Flags::C, 0) }
        self.check_and_set_n(result, cpu);
        cpu.cycle += 2;
    }

    pub fn cpy_cc(&mut self, cpu : &mut MOS6510) {
        let AddrReturn { operand, address, high, low } = self.absolute(cpu);
		self.current_operation.push_str(format!("CPY ${:02X}{:02X}", high.unwrap(), low).as_str());
        let result: u8 = cpu.Y.wrapping_sub(operand);
        if result == 0 { cpu.set_flag(Flags::Z, 1) } else { cpu.set_flag(Flags::Z, 0) }
        if operand <= cpu.Y { cpu.set_flag(Flags::C, 1) } else { cpu.set_flag(Flags::C, 0) }
        self.check_and_set_n(result, cpu);
        cpu.cycle += 4;
    }

    pub fn cpy_c4(&mut self, cpu : &mut MOS6510) {
        let AddrReturn { operand, address, high, low } = self.zero_page(cpu);
		self.current_operation.push_str(format!("CPY ${:02X}", low).as_str());
        let result: u8 = cpu.Y.wrapping_sub(operand);
        if result == 0 { cpu.set_flag(Flags::Z, 1) } else { cpu.set_flag(Flags::Z, 0) }
        if operand <= cpu.Y { cpu.set_flag(Flags::C, 1) } else { cpu.set_flag(Flags::C, 0) }
        self.check_and_set_n(result, cpu);
        cpu.cycle += 3;
    }

    pub fn jmp_4c(&mut self, cpu : &mut MOS6510) {
        let AddrReturn { operand, address, high, low } = self.absolute(cpu);
		self.current_operation.push_str(format!("JMP ${:02X}{:02X}", high.unwrap(), low).as_str());
        cpu.PC = address;
        cpu.cycle += 3;
    }

    pub fn jmp_6c(&mut self, cpu : &mut MOS6510) {
        let AddrReturn { operand, address, high, low } = self.absolute_indirect(cpu);
		self.current_operation.push_str(format!("JMP (${:02X}{:02X})", high.unwrap(), low).as_str());
        cpu.PC = address;
        cpu.cycle += 5;
    }

    pub fn jsr_20(&mut self, cpu : &mut MOS6510) {
        let AddrReturn { operand, address, high, low } = self.absolute(cpu);
		self.current_operation.push_str(format!("JSR ${:02X}{:02X}", high.unwrap(), low).as_str());
        let pc_bytes = self.u16_to_u8s(cpu.PC);
        cpu.push_on_stack(pc_bytes[0]);
        cpu.push_on_stack(pc_bytes[1]);
        cpu.PC = address;
        cpu.cycle += 6;
    }

    pub fn asl_0a(&mut self, cpu : &mut MOS6510) {
		self.current_operation.push_str(format!("ASL A").as_str());
        cpu.set_flag(Flags::C, self.get_bit(cpu.A, 7));
        cpu.set_flag(Flags::N, self.get_bit(cpu.A, 6));
        cpu.A = cpu.A << 1;
        self.check_and_set_z(cpu.A, cpu);
        cpu.cycle += 2;
    }

    pub fn asl_0e(&mut self, cpu : &mut MOS6510) {
        let AddrReturn { mut operand, address, high, low } = self.absolute(cpu);
		self.current_operation.push_str(format!("ASL ${:02X}{:02X}", high.unwrap(), low).as_str());
        cpu.set_flag(Flags::C, self.get_bit(operand, 7));
        cpu.set_flag(Flags::N, self.get_bit(operand, 6));
        operand = operand << 1;
        cpu.mmu.write(operand, address);
        self.check_and_set_z(operand, cpu);
        cpu.cycle += 6;
    }

    pub fn asl_1e(&mut self, cpu : &mut MOS6510) {
        let AddrReturn { mut operand, address, high, low } = self.absolute_indexed(cpu.X as u16, cpu);
		self.current_operation.push_str(format!("ASL ${:02X}{:02X}, X", high.unwrap(), low).as_str());
        cpu.set_flag(Flags::C, self.get_bit(operand, 7));
        cpu.set_flag(Flags::N, self.get_bit(operand, 6));
        operand = operand << 1;
        cpu.mmu.write(operand, address);
        self.check_and_set_z(operand, cpu);
        cpu.cycle += 7;
    }

    pub fn asl_06(&mut self, cpu : &mut MOS6510) {
        let low: u8 = self.fetch(cpu);
		self.current_operation.push_str(format!("ASL ${:02X}", low).as_str());
        let address = self.u8s_to_u16(0x00, low);
        let mut operand = cpu.mmu.read(address);
        cpu.set_flag(Flags::C, self.get_bit(operand, 7));
        cpu.set_flag(Flags::N, self.get_bit(operand, 6));
        operand = operand << 1;
        cpu.mmu.write(operand, address);
        self.check_and_set_z(operand, cpu);
        cpu.cycle += 5;
    }

    pub fn asl_16(&mut self, cpu : &mut MOS6510) {
        let low: u8 = self.fetch(cpu);
		self.current_operation.push_str(format!("ASL ${:02X}, X", low).as_str());
        let address = self.u8s_to_u16(0x00, low.wrapping_add(cpu.X));
        let mut operand = cpu.mmu.read(address);
        cpu.set_flag(Flags::C, self.get_bit(operand, 7));
        cpu.set_flag(Flags::N, self.get_bit(operand, 6));
        operand = operand << 1;
        cpu.mmu.write(operand, address);
        self.check_and_set_z(operand, cpu);
        cpu.cycle += 6;
    }

    pub fn lsr_4a(&mut self, cpu : &mut MOS6510) {
		self.current_operation.push_str(format!("LSR A").as_str());
        cpu.set_flag(Flags::N, 0);
        cpu.set_flag(Flags::C, self.get_bit(cpu.A, 0));
        cpu.A = cpu.A >> 1;
        self.check_and_set_z(cpu.A, cpu);
        cpu.cycle += 2;
    }

    pub fn lsr_4e(&mut self, cpu : &mut MOS6510) {
        let AddrReturn { mut operand, address, high, low } = self.absolute(cpu);
		self.current_operation.push_str(format!("LSR ${:02X}{:02X}", high.unwrap(), low).as_str());
        cpu.set_flag(Flags::N, 0);
        cpu.set_flag(Flags::C, self.get_bit(operand, 0));
        operand = operand >> 1;
        cpu.mmu.write(operand, address);
        self.check_and_set_z(operand, cpu);
        cpu.cycle += 6;
    }

    pub fn lsr_5e(&mut self, cpu : &mut MOS6510) {
        let AddrReturn { mut operand, address, high, low } = self.absolute_indexed(cpu.X as u16, cpu);
		self.current_operation.push_str(format!("LSR ${:02X}{:02X}, X", high.unwrap(), low).as_str());
        cpu.set_flag(Flags::N, 0);
        cpu.set_flag(Flags::C, self.get_bit(operand, 0));
        operand = operand >> 1;
        cpu.mmu.write(operand, address);
        self.check_and_set_z(operand, cpu);
        cpu.cycle += 7;
    }

    pub fn lsr_46(&mut self, cpu : &mut MOS6510) {
        let AddrReturn { mut operand, address, high, low } = self.zero_page(cpu);
		self.current_operation.push_str(format!("LSR ${:02X}", low).as_str());
        cpu.set_flag(Flags::N, 0);
        cpu.set_flag(Flags::C, self.get_bit(operand, 0));
        operand = operand >> 1;
        cpu.mmu.write(operand, address);
        self.check_and_set_z(operand, cpu);
        cpu.cycle += 5;
    }

    pub fn lsr_56(&mut self, cpu : &mut MOS6510) {
        let low: u8 = self.fetch(cpu);
		self.current_operation.push_str(format!("LSR ${:02X}, X", low).as_str());
        let address = self.u8s_to_u16(0x00, low.wrapping_add(cpu.X));
        let mut operand = cpu.mmu.read(address);
        cpu.set_flag(Flags::N, 0);
        cpu.set_flag(Flags::C, self.get_bit(operand, 0));
        operand = operand >> 1;
        cpu.mmu.write(operand, address);
        self.check_and_set_z(operand, cpu);
        cpu.cycle += 6;
    }

    pub fn rol_2a(&mut self, cpu : &mut MOS6510) {
		self.current_operation.push_str(format!("ROL A").as_str());
        cpu.set_flag(Flags::N, self.get_bit(cpu.A, 6));
        let input_bit_7: u8 = self.get_bit(cpu.A, 7);
        cpu.A = cpu.A << 1;
        if cpu.get_flag(Flags::C) { cpu.A = cpu.A | 0b0000_0001 }
        cpu.set_flag(Flags::C, input_bit_7);
        self.check_and_set_z(cpu.A, cpu);
        cpu.cycle += 2;
    }

    pub fn rol_2e(&mut self, cpu : &mut MOS6510) {
        let AddrReturn { mut operand, address, high, low } = self.absolute(cpu);
		self.current_operation.push_str(format!("ROL ${:02X}{:02X}", high.unwrap(), low).as_str());
        cpu.set_flag(Flags::N, self.get_bit(operand, 6));
        let input_bit_7: u8 = self.get_bit(operand, 7);
        operand = operand << 1;
        if cpu.get_flag(Flags::C) { operand = operand | 0b0000_0001 }
        cpu.mmu.write(operand, address);
        cpu.set_flag(Flags::C, input_bit_7);
        self.check_and_set_z(operand, cpu);
        cpu.cycle += 6;
    }

    pub fn rol_3e(&mut self, cpu : &mut MOS6510) {
        let AddrReturn { mut operand, address, high, low } = self.absolute_indexed(cpu.X as u16, cpu);
		self.current_operation.push_str(format!("ROL ${:02X}{:02X}, X", high.unwrap(), low).as_str());
        cpu.set_flag(Flags::N, self.get_bit(operand, 6));
        let input_bit_7: u8 = self.get_bit(operand, 7);
        operand = operand << 1;
        if cpu.get_flag(Flags::C) { operand = operand | 0b0000_0001 }
        cpu.mmu.write(operand, address);
        cpu.set_flag(Flags::C, input_bit_7);
        self.check_and_set_z(operand, cpu);
        cpu.cycle += 7;
    }

    pub fn rol_26(&mut self, cpu : &mut MOS6510) {
        let AddrReturn { mut operand, address, high, low } = self.zero_page(cpu);
		self.current_operation.push_str(format!("ROL ${:02X}", low).as_str());
        cpu.set_flag(Flags::N, self.get_bit(operand, 6));
        let input_bit_7: u8 = self.get_bit(operand, 7);
        operand = operand << 1;
        if cpu.get_flag(Flags::C) { operand = operand | 0b0000_0001 }
        cpu.mmu.write(operand, address);
        cpu.set_flag(Flags::C, input_bit_7);
        self.check_and_set_z(operand, cpu);
        cpu.cycle += 5;
    }

    pub fn rol_36(&mut self, cpu : &mut MOS6510) {
        let low: u8 = self.fetch(cpu);
		self.current_operation.push_str(format!("ROL ${:02X}, X", low).as_str());
        let address = self.u8s_to_u16(0x00, low.wrapping_add(cpu.X));
        let mut operand = cpu.mmu.read(address);
        cpu.set_flag(Flags::N, self.get_bit(operand, 6));
        let input_bit_7: u8 = self.get_bit(operand, 7);
        operand = operand << 1;
        if cpu.get_flag(Flags::C) { operand = operand | 0b0000_0001 }
        cpu.mmu.write(operand, address);
        cpu.set_flag(Flags::C, input_bit_7);
        self.check_and_set_z(operand, cpu);
        cpu.cycle += 6;
    }

    pub fn ror_6a(&mut self, cpu : &mut MOS6510) {
		self.current_operation.push_str(format!("ROR A").as_str());
        if cpu.get_flag(Flags::C) { cpu.set_flag(Flags::N, 1) } else { cpu.set_flag(Flags::N, 0) }
        let input_bit_0: u8 = self.get_bit(cpu.A, 0);
        cpu.A = cpu.A >> 1;
        if cpu.get_flag(Flags::C) { cpu.A = cpu.A | 0b1000_0000 }
        cpu.set_flag(Flags::C, input_bit_0);
        self.check_and_set_z(cpu.A, cpu);
        cpu.cycle += 2;
    }

    pub fn ror_6e(&mut self, cpu : &mut MOS6510) {
        let AddrReturn { mut operand, address, high, low } = self.absolute(cpu);
		self.current_operation.push_str(format!("ROR ${:02X}{:02X}", high.unwrap(), low).as_str());
        if cpu.get_flag(Flags::C) { cpu.set_flag(Flags::N, 1) } else { cpu.set_flag(Flags::N, 0) }
        let input_bit_0: u8 = self.get_bit(operand, 0);
        operand = operand >> 1;
        if cpu.get_flag(Flags::C) { operand = operand | 0b1000_0000 }
        cpu.mmu.write(operand, address);
        cpu.set_flag(Flags::C, input_bit_0);
        self.check_and_set_z(operand, cpu);
        cpu.cycle += 6;
    }

    pub fn ror_7e(&mut self, cpu : &mut MOS6510) {
        let AddrReturn { mut operand, address, high, low } = self.absolute_indexed(cpu.X as u16, cpu);
		self.current_operation.push_str(format!("ROR ${:02X}{:02X}, X", high.unwrap(), low).as_str());
        if cpu.get_flag(Flags::C) { cpu.set_flag(Flags::N, 1) } else { cpu.set_flag(Flags::N, 0) }
        let input_bit_0: u8 = self.get_bit(operand, 0);
        operand = operand >> 1;
        if cpu.get_flag(Flags::C) { operand = operand | 0b1000_0000 }
        cpu.mmu.write(operand, address);
        cpu.set_flag(Flags::C, input_bit_0);
        self.check_and_set_z(operand, cpu);
        cpu.cycle += 7;
    }

    pub fn ror_66(&mut self, cpu : &mut MOS6510) {
        let AddrReturn { mut operand, address, high, low } = self.zero_page(cpu);
		self.current_operation.push_str(format!("ROR ${:02X}", low).as_str());
        if cpu.get_flag(Flags::C) { cpu.set_flag(Flags::N, 1) } else { cpu.set_flag(Flags::N, 0) }
        let input_bit_0: u8 = self.get_bit(operand, 0);
        operand = operand >> 1;
        if cpu.get_flag(Flags::C) { operand = operand | 0b1000_0000 }
        cpu.mmu.write(operand, address);
        cpu.set_flag(Flags::C, input_bit_0);
        self.check_and_set_z(operand, cpu);
        cpu.cycle += 5;
    }

    pub fn ror_76(&mut self, cpu : &mut MOS6510) {
        let low: u8 = self.fetch(cpu);
		self.current_operation.push_str(format!("ROR ${:02X}, X", low).as_str());
        let address = self.u8s_to_u16(0x00, low.wrapping_add(cpu.X));
        let mut operand = cpu.mmu.read(address);
        if cpu.get_flag(Flags::C) { cpu.set_flag(Flags::N, 1) } else { cpu.set_flag(Flags::N, 0) }
        let input_bit_0: u8 = self.get_bit(operand, 0);
        operand = operand >> 1;
        if cpu.get_flag(Flags::C) { operand = operand | 0b1000_0000 }
        cpu.mmu.write(operand, address);
        cpu.set_flag(Flags::C, input_bit_0);
        self.check_and_set_z(operand, cpu);
        cpu.cycle += 6;
    }

    pub fn rti_40(&mut self, cpu : &mut MOS6510) {
        let status: u8 = self.fetch(cpu);
        let low: u8 = self.fetch(cpu);
        let high: u8 = self.fetch(cpu);
        self.current_operation.push_str(format!("RTI flags: {:08b} PC: {:02X}{:02X}", status, high, low).as_str());
        cpu.PC = self.u8s_to_u16(high, low);
        cpu.P = status;
        cpu.cycle += 6;
    }

    pub fn and_29(&mut self, cpu : &mut MOS6510) {
        let operand: u8 = self.fetch(cpu);
		self.current_operation.push_str(format!("AND #${:02X}", operand).as_str());
        cpu.A = cpu.A & operand;
        self.check_and_set_z(cpu.A, cpu);
        self.check_and_set_n(cpu.A, cpu);
        cpu.cycle += 2;
    }

    pub fn and_2d(&mut self, cpu : &mut MOS6510) {
        let AddrReturn { operand, address, high, low } = self.absolute(cpu);
		self.current_operation.push_str(format!("AND ${:02X}{:02X}", high.unwrap(), low).as_str());
        cpu.A = cpu.A & operand;
        self.check_and_set_z(cpu.A, cpu);
        self.check_and_set_n(cpu.A, cpu);
        cpu.cycle += 4;
    }

    pub fn and_3d(&mut self, cpu : &mut MOS6510) {
        let AddrReturn { operand, address, high, low } = self.absolute_indexed(cpu.X as u16, cpu);
		self.current_operation.push_str(format!("AND ${:02X}{:02X}, X", high.unwrap(), low).as_str());
        cpu.A = cpu.A & operand;
        self.check_and_set_z(cpu.A, cpu);
        self.check_and_set_n(cpu.A, cpu);
        cpu.cycle += 4;
    }

    pub fn and_39(&mut self, cpu : &mut MOS6510) {
        let AddrReturn { operand, address, high, low } = self.absolute_indexed(cpu.Y as u16, cpu);
		self.current_operation.push_str(format!("AND ${:02X}{:02X}, Y", high.unwrap(), low).as_str());
        cpu.A = cpu.A & operand;
        self.check_and_set_z(cpu.A, cpu);
        self.check_and_set_n(cpu.A, cpu);
        cpu.cycle += 4;
    }

    pub fn and_25(&mut self, cpu : &mut MOS6510) {
        let AddrReturn { operand, address, high, low } = self.zero_page(cpu);
		self.current_operation.push_str(format!("AND ${:02X}", low).as_str());
        cpu.A = cpu.A & operand;
        self.check_and_set_z(cpu.A, cpu);
        self.check_and_set_n(cpu.A, cpu);
        cpu.cycle += 3;
    }

    pub fn and_35(&mut self, cpu : &mut MOS6510) {
        let low: u8 = self.fetch(cpu);
		self.current_operation.push_str(format!("AND ${:02X}, X", low).as_str());
        let operand: u8 = cpu.mmu.read(self.u8s_to_u16(0x00, low) + cpu.X as u16);
        cpu.A = cpu.A & operand;
        self.check_and_set_z(cpu.A, cpu);
        self.check_and_set_n(cpu.A, cpu);
        cpu.cycle += 4;
    }

    pub fn and_21(&mut self, cpu : &mut MOS6510) {
        let operand: u8 = self.fetch(cpu);
		self.current_operation.push_str(format!("AND (${:02X}, X)", operand).as_str());
        let low_address = operand.wrapping_add(cpu.X);
        let high_address = low_address.wrapping_add(1);
        let low: u8 = cpu.mmu.read(self.u8s_to_u16(0x00, low_address));
        let high: u8 = cpu.mmu.read(self.u8s_to_u16(0x00, high_address));
        cpu.A = cpu.A & cpu.mmu.read(self.u8s_to_u16(high, low));    
        self.check_and_set_n(cpu.A, cpu);
        self.check_and_set_z(cpu.A, cpu);
        cpu.cycle += 6;
    }

    pub fn and_31(&mut self, cpu : &mut MOS6510) {
        let low: u8 = self.fetch(cpu);
		self.current_operation.push_str(format!("AND (${:02X}), Y", low).as_str());
        let high: u8 = low.wrapping_add(1);
        let low_address: u8 = cpu.mmu.read(self.u8s_to_u16(0x00, low));
        let high_address: u8 = cpu.mmu.read(self.u8s_to_u16(0x00, high));
        let address: u16 = self.u8s_to_u16(high_address, low_address) + cpu.Y as u16;
        cpu.A = cpu.A & cpu.mmu.read(address);
        self.check_and_set_n(cpu.A, cpu);
        self.check_and_set_z(cpu.A, cpu);
        cpu.cycle += 5;
    }

    pub fn ora_09(&mut self, cpu : &mut MOS6510) {
        let operand: u8 = self.fetch(cpu);
		self.current_operation.push_str(format!("ORA #${:02X}", operand).as_str());
        cpu.A = cpu.A | operand;
        self.check_and_set_z(cpu.A, cpu);
        self.check_and_set_n(cpu.A, cpu);
        cpu.cycle += 2;
    }

    pub fn ora_0d(&mut self, cpu : &mut MOS6510) {
        let AddrReturn { operand, address, high, low } = self.absolute(cpu);
		self.current_operation.push_str(format!("ORA ${:02X}{:02X}", high.unwrap(), low).as_str());
        cpu.A = cpu.A | operand;
        self.check_and_set_z(cpu.A, cpu);
        self.check_and_set_n(cpu.A, cpu);
        cpu.cycle += 4;
    }

    pub fn ora_1d(&mut self, cpu : &mut MOS6510) {
        let AddrReturn { operand, address, high, low } = self.absolute_indexed(cpu.X as u16, cpu);
		self.current_operation.push_str(format!("ORA ${:02X}{:02X}, X", high.unwrap(), low).as_str());
        cpu.A = cpu.A | operand;
        self.check_and_set_z(cpu.A, cpu);
        self.check_and_set_n(cpu.A, cpu);
        cpu.cycle += 4;
    }

    pub fn ora_19(&mut self, cpu : &mut MOS6510) {
        let AddrReturn { operand, address, high, low } = self.absolute_indexed(cpu.Y as u16, cpu);
		self.current_operation.push_str(format!("ORA ${:02X}{:02X}, Y", high.unwrap(), low).as_str());
        cpu.A = cpu.A | operand;
        self.check_and_set_z(cpu.A, cpu);
        self.check_and_set_n(cpu.A, cpu);
        cpu.cycle += 4;
    }

    pub fn ora_05(&mut self, cpu : &mut MOS6510) {
        let AddrReturn { operand, address, high, low } = self.zero_page(cpu);
		self.current_operation.push_str(format!("ORA ${:02X}", low).as_str());
        cpu.A = cpu.A | operand;
        self.check_and_set_z(cpu.A, cpu);
        self.check_and_set_n(cpu.A, cpu);
        cpu.cycle += 3;
    }

    pub fn ora_15(&mut self, cpu : &mut MOS6510) {
        let low: u8 = self.fetch(cpu);
		self.current_operation.push_str(format!("ORA ${:02X}, X", low).as_str());
        let operand: u8 = cpu.mmu.read(self.u8s_to_u16(0x00, low) + cpu.X as u16);
        cpu.A = cpu.A | operand;
        self.check_and_set_z(cpu.A, cpu);
        self.check_and_set_n(cpu.A, cpu);
        cpu.cycle += 4;
    }

    pub fn ora_01(&mut self, cpu : &mut MOS6510) {
        let operand: u8 = self.fetch(cpu);
		self.current_operation.push_str(format!("ORA (${:02X}, X)", operand).as_str());
        let low_address = operand.wrapping_add(cpu.X);
        let high_address = low_address.wrapping_add(1);
        let low: u8 = cpu.mmu.read(self.u8s_to_u16(0x00, low_address));
        let high: u8 = cpu.mmu.read(self.u8s_to_u16(0x00, high_address));
        cpu.A = cpu.A | cpu.mmu.read(self.u8s_to_u16(high, low));    
        self.check_and_set_n(cpu.A, cpu);
        self.check_and_set_z(cpu.A, cpu);
        cpu.cycle += 6;
    }

    pub fn ora_11(&mut self, cpu : &mut MOS6510) {
        let low: u8 = self.fetch(cpu);
		self.current_operation.push_str(format!("ORA (${:02X}), Y", low).as_str());
        let high: u8 = low.wrapping_add(1);
        let low_address: u8 = cpu.mmu.read(self.u8s_to_u16(0x00, low));
        let high_address: u8 = cpu.mmu.read(self.u8s_to_u16(0x00, high));
        let address: u16 = self.u8s_to_u16(high_address, low_address) + cpu.Y as u16;
        cpu.A = cpu.A | cpu.mmu.read(address);
        self.check_and_set_n(cpu.A, cpu);
        self.check_and_set_z(cpu.A, cpu);
        cpu.cycle += 5;
    }

    pub fn eor_49(&mut self, cpu : &mut MOS6510) {
        let operand: u8 = self.fetch(cpu);
		self.current_operation.push_str(format!("EOR #${:02X}", operand).as_str());
        cpu.A = cpu.A ^ operand;
        self.check_and_set_z(cpu.A, cpu);
        self.check_and_set_n(cpu.A, cpu);
        cpu.cycle += 2;
    }

    pub fn eor_4d(&mut self, cpu : &mut MOS6510) {
        let AddrReturn { operand, address, high, low } = self.absolute(cpu);
		self.current_operation.push_str(format!("EOR ${:02X}{:02X}", high.unwrap(), low).as_str());
        cpu.A = cpu.A ^ operand;
        self.check_and_set_z(cpu.A, cpu);
        self.check_and_set_n(cpu.A, cpu);
        cpu.cycle += 4;
    }

    pub fn eor_5d(&mut self, cpu : &mut MOS6510) {
        let AddrReturn { operand, address, high, low } = self.absolute_indexed(cpu.X as u16, cpu);
		self.current_operation.push_str(format!("EOR ${:02X}{:02X}, X", high.unwrap(), low).as_str());
        cpu.A = cpu.A ^ operand;
        self.check_and_set_z(cpu.A, cpu);
        self.check_and_set_n(cpu.A, cpu);
        cpu.cycle += 4;
    }

    pub fn eor_59(&mut self, cpu : &mut MOS6510) {
        let AddrReturn { operand, address, high, low } = self.absolute_indexed(cpu.Y as u16, cpu);
		self.current_operation.push_str(format!("EOR ${:02X}{:02X}, Y", high.unwrap(), low).as_str());
        cpu.A = cpu.A ^ operand;
        self.check_and_set_z(cpu.A, cpu);
        self.check_and_set_n(cpu.A, cpu);
        cpu.cycle += 4;
    }

    pub fn eor_45(&mut self, cpu : &mut MOS6510) {
        let AddrReturn { operand, address, high, low } = self.zero_page(cpu);
		self.current_operation.push_str(format!("EOR ${:02X}", low).as_str());
        cpu.A = cpu.A ^ operand;
        self.check_and_set_z(cpu.A, cpu);
        self.check_and_set_n(cpu.A, cpu);
        cpu.cycle += 3;
    }

    pub fn eor_55(&mut self, cpu : &mut MOS6510) {
        let low: u8 = self.fetch(cpu);
		self.current_operation.push_str(format!("EOR ${:02X}, X", low).as_str());
        let operand: u8 = cpu.mmu.read(self.u8s_to_u16(0x00, low) + cpu.X as u16);
        cpu.A = cpu.A ^ operand;
        self.check_and_set_z(cpu.A, cpu);
        self.check_and_set_n(cpu.A, cpu);
        cpu.cycle += 4;
    }

    pub fn eor_41(&mut self, cpu : &mut MOS6510) {
        let operand: u8 = self.fetch(cpu);
		self.current_operation.push_str(format!("EOR (${:02X}, X)", operand).as_str());
        let low_address = operand.wrapping_add(cpu.X);
        let high_address = low_address.wrapping_add(1);
        let low: u8 = cpu.mmu.read(self.u8s_to_u16(0x00, low_address));
        let high: u8 = cpu.mmu.read(self.u8s_to_u16(0x00, high_address));
        cpu.A = cpu.A ^ cpu.mmu.read(self.u8s_to_u16(high, low));    
        self.check_and_set_n(cpu.A, cpu);
        self.check_and_set_z(cpu.A, cpu);
        cpu.cycle += 6;
    }

    pub fn eor_51(&mut self, cpu : &mut MOS6510) {
        let low: u8 = self.fetch(cpu);
		self.current_operation.push_str(format!("EOR (${:02X}), Y", low).as_str());
        let high: u8 = low.wrapping_add(1);
        let low_address: u8 = cpu.mmu.read(self.u8s_to_u16(0x00, low));
        let high_address: u8 = cpu.mmu.read(self.u8s_to_u16(0x00, high));
        let address: u16 = self.u8s_to_u16(high_address, low_address) + cpu.Y as u16;
        cpu.A = cpu.A ^ cpu.mmu.read(address);
        self.check_and_set_n(cpu.A, cpu);
        self.check_and_set_z(cpu.A, cpu);
        cpu.cycle += 5;
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
        if value == 0 { cpu.set_flag(Flags::Z, 1) } else { cpu.set_flag(Flags::Z, 0) }
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

    pub fn get_bit(&mut self, byte: u8, index: u8) -> u8 {
        let helper: u8 = 0b0000_0001 << index;
        let result: u8 = byte & helper;
        if result == 0 { return 0 } else { return 1 }
    }

    pub fn bcd(&mut self, byte: u8) -> u8 {
        let high: u8 = (byte & 0xF0) >> 4;
        let low: u8 = byte & 0x0F;
        high * 10 + low
    }

    pub fn absolute(&mut self, cpu : &mut MOS6510) -> AddrReturn {
        let low: u8 = self.fetch(cpu);
        let high: u8 = self.fetch(cpu);
        let address: u16 = self.u8s_to_u16(high, low);
        AddrReturn { operand: cpu.mmu.read(address), address, high: Some(high), low }
    }

    pub fn absolute_indexed(&mut self, index: u16, cpu : &mut MOS6510) -> AddrReturn {
        let low: u8 = self.fetch(cpu);
        let high: u8 = self.fetch(cpu);
        let address: u16 = self.u8s_to_u16(high, low) + index;
        AddrReturn { operand: cpu.mmu.read(address), address, high: Some(high), low }
    }

    pub fn absolute_indirect(&mut self, cpu : &mut MOS6510) -> AddrReturn {
        let low: u8 = self.fetch(cpu);
        let high: u8 = self.fetch(cpu);
        let low_address: u8 = cpu.mmu.read(self.u8s_to_u16(high, low));
        let high_address: u8 = cpu.mmu.read(self.u8s_to_u16(high, low.wrapping_add(1)));
        let address: u16 = self.u8s_to_u16(high_address, low_address);
        AddrReturn { operand: cpu.mmu.read(address), address, high: Some(high), low }
    }

    pub fn zero_page(&mut self, cpu : &mut MOS6510) -> AddrReturn {
        let low: u8 = self.fetch(cpu);
        let address: u16 = self.u8s_to_u16(0x00, low);
        AddrReturn { operand: cpu.mmu.read(address), address, high: None, low }
    }

    pub fn zero_page_indexed(&mut self, index: u8, cpu : &mut MOS6510) -> AddrReturn {
        let low: u8 = self.fetch(cpu);
        let address: u16 = self.u8s_to_u16(0x00, low.wrapping_add(index));
        AddrReturn { operand: cpu.mmu.read(address), address, high: None, low }
    }

    pub fn zero_page_indexed_indirect(&mut self, index: u8, cpu : &mut MOS6510) -> AddrReturn {
        let low: u8 = self.fetch(cpu);
        let low_address: u8 = cpu.mmu.read(self.u8s_to_u16(0x00, low.wrapping_add(index)));
        let high_address: u8 = cpu.mmu.read(self.u8s_to_u16(0x00, low.wrapping_add(index).wrapping_add(1)));
        let address: u16 = self.u8s_to_u16(high_address, low_address);
        AddrReturn { operand: cpu.mmu.read(address), address, high: None, low }
    }

    pub fn zero_page_indirect_indexed(&mut self, index: u8, cpu : &mut MOS6510) -> AddrReturn {
        let low: u8 = self.fetch(cpu);
        let low_address: u8 = cpu.mmu.read(self.u8s_to_u16(0x00, low)).wrapping_add(index);
        let high_address: u8 = cpu.mmu.read(self.u8s_to_u16(0x00, low)).wrapping_add(index).wrapping_add(1);
        let address: u16 = self.u8s_to_u16(high_address, low_address);
        AddrReturn { operand: cpu.mmu.read(address), address, high: None, low }
    }

    pub fn relative(&mut self, index: u8, cpu : &mut MOS6510) -> AddrReturn {
        let offset: i16 = self.fetch(cpu) as i16;
        let pc_bytes: [u8; 2] = self.u16_to_u8s(cpu.PC);
        let value: u8 = offset.wrapping_add(pc_bytes[1] as i16) as u8;
        let address: u16 = self.u8s_to_u16(pc_bytes[0], value);
        cpu.PC = address;
        AddrReturn { operand: value, address, high: None, low: offset as u8 }
    }
}
