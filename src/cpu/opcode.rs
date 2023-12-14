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
        self.table[0x2a] = Opcode::rol_a_2a;
        self.table[0x38] = Opcode::sec_38;
        self.table[0x48] = Opcode::pha_48;
        self.table[0x4a] = Opcode::lsr_a_4a;
        self.table[0x58] = Opcode::cli_58;
        self.table[0x60] = Opcode::rts_60;
        self.table[0x68] = Opcode::pla_68;
        self.table[0x78] = Opcode::sei_78;
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

    pub fn rol_a_2a(&mut self, cpu : &mut MOS6510) {
        self.current_operation.push_str("ROL A");
        let new_A: u8 = cpu.A << 1;
        if (cpu.get_flag(Flags::C)) { new_A | 1; }
        cpu.set_flag(Flags::C, (cpu.A & (1 << 7)) >> 7);
        cpu.set_flag(Flags::N, (cpu.A & (1 << 6)) >> 6);
        self.check_and_set_z(new_A, cpu);
        cpu.A = new_A;
        cpu.cycle += 2;
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

    pub fn lda_a9(&mut self, cpu : &mut MOS6510) {
        let operand: u8 = self.fetch(cpu);
		self.current_operation.push_str(format!("LDA #${:02X}", operand).as_str());
        cpu.A = operand;
        self.check_and_set_n(cpu.A, cpu);
        self.check_and_set_z(cpu.A, cpu);
        cpu.cycle += 2;
    }

    pub fn lda_ad(&mut self, cpu : &mut MOS6510) {
        let low: u8 = self.fetch(cpu);
        let high: u8 = self.fetch(cpu);
		self.current_operation.push_str(format!("LDA ${:02X}{:02X}", high, low).as_str());
        let address: u16 = self.u8s_to_u16(high, low);
        cpu.A = cpu.mmu.read(address);    
        self.check_and_set_n(cpu.A, cpu);
        self.check_and_set_z(cpu.A, cpu);
        cpu.cycle += 4;
    }

    pub fn lda_bd(&mut self, cpu : &mut MOS6510) {
        let low: u8 = self.fetch(cpu);
        let high: u8 = self.fetch(cpu);
		self.current_operation.push_str(format!("LDA ${:02X}{:02X}, X", high, low).as_str());
        let address: u16 = self.u8s_to_u16(high, low);
        cpu.A = cpu.mmu.read(address + cpu.X as u16);    
        self.check_and_set_n(cpu.A, cpu);
        self.check_and_set_z(cpu.A, cpu);
        cpu.cycle += 4;
        // TODO: cycle is 4+1 if page is crossed
    }

    pub fn lda_b9(&mut self, cpu : &mut MOS6510) {
        let low: u8 = self.fetch(cpu);
        let high: u8 = self.fetch(cpu);
		self.current_operation.push_str(format!("LDA ${:02X}{:02X}, Y", high, low).as_str());
        let address: u16 = self.u8s_to_u16(high, low);
        cpu.A = cpu.mmu.read(address + cpu.Y as u16);    
        self.check_and_set_n(cpu.A, cpu);
        self.check_and_set_z(cpu.A, cpu);
        cpu.cycle += 4;
        // TODO: cycle is 4+1 if page is crossed
    }

    pub fn lda_a5(&mut self, cpu : &mut MOS6510) {
        let operand: u8 = self.fetch(cpu);
		self.current_operation.push_str(format!("LDA ${:02X}", operand).as_str());
        let address: u16 = self.u8s_to_u16(0x00, operand);
        cpu.A = cpu.mmu.read(address);    
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
		self.current_operation.push_str(format!("LDA ((${:02X}, X))", operand).as_str());
        let low_address = operand.wrapping_add(cpu.X);
        let high_address = low_address.wrapping_add(1);
        let low: u8 = cpu.mmu.read(self.u8s_to_u16(0x00, low_address));
        let high: u8 = cpu.mmu.read(self.u8s_to_u16(0x00, operand));
        cpu.A = cpu.mmu.read(self.u8s_to_u16(high, low));    
        self.check_and_set_n(cpu.A, cpu);
        self.check_and_set_z(cpu.A, cpu);
        cpu.cycle += 6;
    }

    pub fn lda_b1(&mut self, cpu : &mut MOS6510) {
        let low: u8 = self.fetch(cpu);
		self.current_operation.push_str(format!("LDA ((${:02X})), Y", low).as_str());
        let high: u8 = low.wrapping_add(1);
        let address: u16 = self.u8s_to_u16(high, low) + cpu.Y as u16;
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
        let low: u8 = self.fetch(cpu);
        let high: u8 = self.fetch(cpu);
		self.current_operation.push_str(format!("LDX ${:02X}{:02X}", high, low).as_str());
        let address: u16 = self.u8s_to_u16(high, low);
        cpu.X = cpu.mmu.read(address);    
        self.check_and_set_n(cpu.X, cpu);
        self.check_and_set_z(cpu.X, cpu);
        cpu.cycle += 4;
    }

    pub fn ldx_be(&mut self, cpu : &mut MOS6510) {
        let low: u8 = self.fetch(cpu);
        let high: u8 = self.fetch(cpu);
		self.current_operation.push_str(format!("LDX ${:02X}{:02X}, Y", high, low).as_str());
        let address: u16 = self.u8s_to_u16(high, low);
        cpu.X = cpu.mmu.read(address + cpu.Y as u16);    
        self.check_and_set_n(cpu.X, cpu);
        self.check_and_set_z(cpu.X, cpu);
        cpu.cycle += 4;
        // TODO: cycle is 4+1 if page is crossed
    }

    pub fn ldx_a6(&mut self, cpu : &mut MOS6510) {
        let operand: u8 = self.fetch(cpu);
		self.current_operation.push_str(format!("LDX ${:02X}", operand).as_str());
        let address: u16 = self.u8s_to_u16(0x00, operand);
        cpu.X = cpu.mmu.read(address);    
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
        let low: u8 = self.fetch(cpu);
        let high: u8 = self.fetch(cpu);
		self.current_operation.push_str(format!("LDY ${:02X}{:02X}", high, low).as_str());
        let address: u16 = self.u8s_to_u16(high, low);
        cpu.Y = cpu.mmu.read(address);
        self.check_and_set_n(cpu.Y, cpu);
        self.check_and_set_z(cpu.Y, cpu);
        cpu.cycle += 4;
    }

    pub fn ldy_bc(&mut self, cpu : &mut MOS6510) {
        let low: u8 = self.fetch(cpu);
        let high: u8 = self.fetch(cpu);
		self.current_operation.push_str(format!("LDY ${:02X}{:02X}, X", high, low).as_str());
        let address: u16 = self.u8s_to_u16(high, low);
        cpu.Y = cpu.mmu.read(address + cpu.X as u16);    
        self.check_and_set_n(cpu.Y, cpu);
        self.check_and_set_z(cpu.Y, cpu);
        cpu.cycle += 4;
        // TODO: cycle is 4+1 if page is crossed
    }

    pub fn ldy_a4(&mut self, cpu : &mut MOS6510) {
        let operand: u8 = self.fetch(cpu);
		self.current_operation.push_str(format!("LDY ${:02X}", operand).as_str());
        let address: u16 = self.u8s_to_u16(0x00, operand);
        cpu.Y = cpu.mmu.read(address);
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
        let low: u8 = self.fetch(cpu);
        let high: u8 = self.fetch(cpu);
		self.current_operation.push_str(format!("STA ${:02X}{:02X}", high, low).as_str());
        let address: u16 = self.u8s_to_u16(high, low);
        cpu.mmu.write(cpu.A, address);
        cpu.cycle += 4;
    }

    pub fn sta_9d(&mut self, cpu : &mut MOS6510) {
        let low: u8 = self.fetch(cpu);
        let high: u8 = self.fetch(cpu);
		self.current_operation.push_str(format!("STA ${:02X}{:02X}, X", high, low).as_str());
        let address: u16 = self.u8s_to_u16(high, low);
        cpu.mmu.write(cpu.A, address + cpu.X as u16);
        cpu.cycle += 5;
    }

    pub fn sta_99(&mut self, cpu : &mut MOS6510) {
        let low: u8 = self.fetch(cpu);
        let high: u8 = self.fetch(cpu);
		self.current_operation.push_str(format!("STA ${:02X}{:02X}, Y", high, low).as_str());
        let address: u16 = self.u8s_to_u16(high, low);
        cpu.mmu.write(cpu.A, address + cpu.Y as u16);
        cpu.cycle += 5;
    }

    pub fn sta_85(&mut self, cpu : &mut MOS6510) {
        let operand: u8 = self.fetch(cpu);
		self.current_operation.push_str(format!("STA ${:02X}", operand).as_str());
        let address: u16 = self.u8s_to_u16(0x00, operand);
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
		self.current_operation.push_str(format!("STA ((${:02X}, X))", operand).as_str());
        let low_address = operand.wrapping_add(cpu.X);
        let high_address = low_address.wrapping_add(1);
        let low: u8 = cpu.mmu.read(self.u8s_to_u16(0x00, low_address));
        let high: u8 = cpu.mmu.read(self.u8s_to_u16(0x00, operand));
        let address: u16 = self.u8s_to_u16(high, low);
        cpu.mmu.write(cpu.A, address);
        cpu.cycle += 6;
    }

    pub fn sta_91(&mut self, cpu : &mut MOS6510) {
        let mut low: u8 = self.fetch(cpu);
		self.current_operation.push_str(format!("STA ((${:02X})), Y", low).as_str());
        let high: u8 = low.wrapping_add(1);
        let address: u16 = self.u8s_to_u16(high, low) + cpu.Y as u16;
        cpu.mmu.write(cpu.A, address);
        cpu.cycle += 6;
    }

    pub fn stx_8e(&mut self, cpu : &mut MOS6510) {
        let low: u8 = self.fetch(cpu);
        let high: u8 = self.fetch(cpu);
		self.current_operation.push_str(format!("STX ${:02X}{:02X}", high, low).as_str());
        let address: u16 = self.u8s_to_u16(high, low);
        cpu.mmu.write(cpu.X, address);
        cpu.cycle += 4;
    }

    pub fn stx_86(&mut self, cpu : &mut MOS6510) {
        let operand: u8 = self.fetch(cpu);
		self.current_operation.push_str(format!("STX ${:02X}", operand).as_str());
        let address: u16 = self.u8s_to_u16(0x00, operand);
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
        let low: u8 = self.fetch(cpu);
        let high: u8 = self.fetch(cpu);
		self.current_operation.push_str(format!("STY ${:02X}{:02X}", high, low).as_str());
        let address: u16 = self.u8s_to_u16(high, low);
        cpu.mmu.write(cpu.Y, address);
        cpu.cycle += 4;
    }

    pub fn sty_84(&mut self, cpu : &mut MOS6510) {
        let operand: u8 = self.fetch(cpu);
		self.current_operation.push_str(format!("STY ${:02X}", operand).as_str());
        let address: u16 = self.u8s_to_u16(0x00, operand);
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
