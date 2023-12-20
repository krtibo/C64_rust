// https://www.pagetable.com/c64ref/6502/

#![allow(non_snake_case)]
#![allow(unused)]

use super::super::MOS6510;
use super::super::Flags;
use super::*;

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
        self.table[0x00] = brk_00;
        self.table[0x01] = ora_01;
        self.table[0x05] = ora_05;
        self.table[0x06] = asl_06;
        self.table[0x08] = php_08;
        self.table[0x09] = ora_09;
        self.table[0x0a] = asl_0a;
        self.table[0x0e] = asl_0e;
        self.table[0x0d] = ora_0d;
        self.table[0x10] = bpl_10;
        self.table[0x11] = ora_11;
        self.table[0x15] = ora_15;
        self.table[0x16] = asl_16;
        self.table[0x18] = clc_18;
        self.table[0x19] = ora_19;
        self.table[0x1d] = ora_1d;
        self.table[0x1e] = asl_1e;
        self.table[0x20] = jsr_20;
        self.table[0x21] = and_21;
        self.table[0x24] = bit_24;
        self.table[0x25] = and_25;
        self.table[0x26] = rol_26;
        self.table[0x28] = plp_28;
        self.table[0x29] = and_29;
        self.table[0x2a] = rol_2a;
        self.table[0x2c] = bit_2c;
        self.table[0x2d] = and_2d;
        self.table[0x2e] = rol_2e;
        self.table[0x30] = bmi_30;
        self.table[0x31] = and_31;
        self.table[0x35] = and_35;
        self.table[0x36] = rol_36;
        self.table[0x38] = sec_38;
        self.table[0x39] = and_39;
        self.table[0x3d] = and_3d;
        self.table[0x3e] = rol_3e;
        self.table[0x40] = rti_40;
        self.table[0x41] = eor_41;
        self.table[0x45] = eor_45;
        self.table[0x46] = lsr_46;
        self.table[0x48] = pha_48;
        self.table[0x49] = eor_49;
        self.table[0x4a] = lsr_4a;
        self.table[0x4c] = jmp_4c;
        self.table[0x4d] = eor_4d;
        self.table[0x4e] = lsr_4e;
        self.table[0x50] = bvc_50;
        self.table[0x51] = eor_51;
        self.table[0x55] = eor_55;
        self.table[0x56] = lsr_56;
        self.table[0x58] = cli_58;
        self.table[0x59] = eor_59;
        self.table[0x5e] = lsr_5e;
        self.table[0x5d] = eor_5d;
        self.table[0x60] = rts_60;
        self.table[0x61] = adc_61;
        self.table[0x65] = adc_65;
        self.table[0x66] = ror_66;
        self.table[0x68] = pla_68;
        self.table[0x69] = adc_69;
        self.table[0x6a] = ror_6a;
        self.table[0x6c] = jmp_6c;
        self.table[0x6d] = adc_6d;
        self.table[0x6e] = ror_6e;
        self.table[0x70] = bvs_70;
        self.table[0x71] = adc_71;
        self.table[0x75] = adc_75;
        self.table[0x76] = ror_76;
        self.table[0x78] = sei_78;
        self.table[0x79] = adc_79;
        self.table[0x7d] = adc_7d;
        self.table[0x7e] = ror_7e;
        self.table[0x81] = sta_81;
        self.table[0x84] = sty_84;
        self.table[0x85] = sta_85;
        self.table[0x86] = stx_86;
        self.table[0x88] = dey_88;
        self.table[0x8a] = txa_8a;
        self.table[0x8c] = sty_8c;
        self.table[0x8d] = sta_8d;
        self.table[0x8e] = stx_8e;
        self.table[0x90] = bcc_90;
        self.table[0x91] = sta_91;
        self.table[0x94] = sty_94;
        self.table[0x95] = sta_95;
        self.table[0x96] = stx_96;
        self.table[0x98] = tya_98;
        self.table[0x99] = sta_99;
        self.table[0x9a] = txs_9a;
        self.table[0x9d] = sta_9d;
        self.table[0xa0] = ldy_a0;
        self.table[0xa1] = lda_a1;
        self.table[0xa2] = ldx_a2;
        self.table[0xa4] = ldy_a4;
        self.table[0xa5] = lda_a5;
        self.table[0xa6] = ldx_a6;
        self.table[0xa8] = tay_a8;
        self.table[0xa9] = lda_a9;
        self.table[0xaa] = tax_aa;
        self.table[0xac] = ldy_ac;
        self.table[0xad] = lda_ad;
        self.table[0xae] = ldx_ae;
        self.table[0xb0] = bcs_b0;
        self.table[0xb1] = lda_b1;
        self.table[0xb4] = ldy_b4;
        self.table[0xb5] = lda_b5;
        self.table[0xb6] = ldx_b6;
        self.table[0xb8] = clv_b8;
        self.table[0xb9] = lda_b9;
        self.table[0xba] = tsx_ba;
        self.table[0xbc] = ldy_bc;
        self.table[0xbd] = lda_bd;
        self.table[0xbe] = ldx_be;
        self.table[0xc0] = cpy_c0;
        self.table[0xc1] = cmp_c1;
        self.table[0xc4] = cpy_c4;
        self.table[0xc5] = cmp_c5;
        self.table[0xc6] = dec_c6;
        self.table[0xc8] = iny_c8;
        self.table[0xc9] = cmp_c9;
        self.table[0xca] = dex_ca;
        self.table[0xcc] = cpy_cc;
        self.table[0xcd] = cmp_cd;
        self.table[0xce] = dec_ce;
        self.table[0xd0] = bne_d0;
        self.table[0xd1] = cmp_d1;
        self.table[0xd5] = cmp_d5;
        self.table[0xd6] = dec_d6;
        self.table[0xd8] = cld_d8;
        self.table[0xd9] = cmp_d9;
        self.table[0xdd] = cmp_dd;
        self.table[0xde] = dec_de;
        self.table[0xe0] = cpx_e0;
        self.table[0xe1] = sbc_e1;
        self.table[0xe4] = cpx_e4;
        self.table[0xe5] = sbc_e5;
        self.table[0xe6] = inc_e6;
        self.table[0xe8] = inx_e8;
        self.table[0xe9] = sbc_e9;
        self.table[0xea] = nop_ea;
        self.table[0xec] = cpx_ec;
        self.table[0xed] = sbc_ed;
        self.table[0xee] = inc_ee;
        self.table[0xf0] = beq_f0;
        self.table[0xf1] = sbc_f1;
        self.table[0xf5] = sbc_f5;
        self.table[0xf6] = inc_f6;
        self.table[0xf8] = sed_f8;
        self.table[0xf9] = sbc_f9;
        self.table[0xfd] = sbc_fd;
        self.table[0xfe] = inc_fe;
    }

    pub fn unknown(&mut self, cpu : &mut MOS6510) {
        self.current_operation.push_str("////////////// n/a");
        cpu.cycle += 4;
    }

    pub fn fetch(&mut self, cpu : &mut MOS6510) -> u8 {
        let ret = cpu.mmu.read(cpu.PC);
        cpu.PC = cpu.PC.wrapping_add(1);
        // if (cpu.PC < 65535) {
        //     cpu.PC += 1;
        // } else {
        //     // TODO: take this out
        //     cpu.PC = 0xA000;
        // }
        ret
    }
    
    pub fn execute(&mut self, cpu : &mut MOS6510) {
        let current_opcode: u8 = self.fetch(cpu);
        self.current_operation = format!("{:02X} - ", current_opcode);
        self.table[current_opcode as usize](self, cpu)
    }
    
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

    pub fn relative(&mut self, cpu : &mut MOS6510) -> AddrReturn {
        let offset: i16 = self.fetch(cpu) as i16;
        let pc_bytes: [u8; 2] = self.u16_to_u8s(cpu.PC);
        let value: u8 = offset.wrapping_add(pc_bytes[1] as i16) as u8;
        let address: u16 = self.u8s_to_u16(pc_bytes[0], value);
        cpu.PC = address;
        AddrReturn { operand: value, address, high: Some(pc_bytes[0]), low: value as u8 }
    }
}
