use super::Opcode;
use super::AddrReturn;
use super::super::MOS6510;
use super::super::Flags;

pub fn cmp(opc: &mut Opcode, cpu: &mut MOS6510, operand: u8, register: u8) {
    let result: u8 = register.wrapping_sub(operand);
    if result == 0 { cpu.set_flag(Flags::Z, 1) } else { cpu.set_flag(Flags::Z, 0) }
    if operand <= register { cpu.set_flag(Flags::C, 1) } else { cpu.set_flag(Flags::C, 0) }
    opc.check_and_set_n(result, cpu);
}

pub fn cmp_c9(opc: &mut Opcode, cpu: &mut MOS6510) {
    let operand: u8 = opc.fetch(cpu);
    opc.current_operation.push_str(format!("CMP #${:02X}", operand).as_str());
    cmp(opc, cpu, operand, cpu.A);
    cpu.cycle += 2;
}

pub fn cmp_cd(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.absolute(cpu);
    opc.current_operation.push_str(format!("CMP ${:02X}{:02X}", high.unwrap(), low).as_str());
    cmp(opc, cpu, operand, cpu.A);
    cpu.cycle += 4;
}

pub fn cmp_dd(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.absolute_indexed(cpu.X as u16, cpu);
    opc.current_operation.push_str(format!("CMP ${:02X}{:02X}, X", high.unwrap(), low).as_str());
    cmp(opc, cpu, operand, cpu.A);
    cpu.cycle += 4;
}

pub fn cmp_d9(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.absolute_indexed(cpu.Y as u16, cpu);
    opc.current_operation.push_str(format!("CMP ${:02X}{:02X}, Y", high.unwrap(), low).as_str());
    cmp(opc, cpu, operand, cpu.A);
    cpu.cycle += 4;
}

pub fn cmp_c5(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.zero_page(cpu);
    opc.current_operation.push_str(format!("CMP ${:02X}", low).as_str());
    cmp(opc, cpu, operand, cpu.A);
    cpu.cycle += 3;
}

pub fn cmp_d5(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { mut operand, address, high, low } = opc.zero_page_indexed(cpu.X, cpu);
    opc.current_operation.push_str(format!("CMP ${:02X}, X", low).as_str());
    cmp(opc, cpu, operand, cpu.A);
    cpu.cycle += 4;
}

pub fn cmp_c1(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.zero_page_indexed_indirect(cpu.X, cpu);
    opc.current_operation.push_str(format!("CMP (${:02X}, X)", low).as_str());
    cmp(opc, cpu, operand, cpu.A);
    cpu.cycle += 6;
}

pub fn cmp_d1(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.zero_page_indirect_indexed(cpu.X, cpu);
    opc.current_operation.push_str(format!("CMP (${:02X}), Y", low).as_str());
    cmp(opc, cpu, operand, cpu.A);
    cpu.cycle += 5;
}

pub fn cpx_e0(opc: &mut Opcode, cpu: &mut MOS6510) {
    let operand: u8 = opc.fetch(cpu);
    opc.current_operation.push_str(format!("CPX #${:02X}", operand).as_str());
    cmp(opc, cpu, operand, cpu.X);
    cpu.cycle += 2;
}

pub fn cpx_ec(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.absolute(cpu);
    opc.current_operation.push_str(format!("CPX ${:02X}{:02X}", high.unwrap(), low).as_str());
    cmp(opc, cpu, operand, cpu.X);
    cpu.cycle += 4;
}

pub fn cpx_e4(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.zero_page(cpu);
    opc.current_operation.push_str(format!("CPX ${:02X}", low).as_str());
    cmp(opc, cpu, operand, cpu.X);
    cpu.cycle += 3;
}

pub fn cpy_c0(opc: &mut Opcode, cpu: &mut MOS6510) {
    let operand: u8 = opc.fetch(cpu);
    opc.current_operation.push_str(format!("CPY #${:02X}", operand).as_str());
    cmp(opc, cpu, operand, cpu.Y);
    cpu.cycle += 2;
}

pub fn cpy_cc(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.absolute(cpu);
    opc.current_operation.push_str(format!("CPY ${:02X}{:02X}", high.unwrap(), low).as_str());
    cmp(opc, cpu, operand, cpu.Y);
    cpu.cycle += 4;
}

pub fn cpy_c4(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.zero_page(cpu);
    opc.current_operation.push_str(format!("CPY ${:02X}", low).as_str());
    cmp(opc, cpu, operand, cpu.Y);
    cpu.cycle += 3;
}

pub fn adc(opc: &mut Opcode, cpu: &mut MOS6510, operand: u8) {
    let carry: u8 = if cpu.get_flag(Flags::C) { 1 } else { 0 };
    if cpu.get_flag(Flags::D) {
        let result: u16 = (opc.bcd(cpu.A) + opc.bcd(operand) + carry) as u16;
        if result > 99 { cpu.set_flag(Flags::C, 1) } else { cpu.set_flag(Flags::C, 0) }
        cpu.A = opc.bcd(cpu.A).wrapping_add(opc.bcd(operand)).wrapping_add(carry);
    } else {
        let result: i16 = (cpu.A as i16 + operand as i16 + carry as i16) as i16;
        if result > 255 { cpu.set_flag(Flags::C, 1) } else { cpu.set_flag(Flags::C, 0) }
        if result > 127 || result < -127 { cpu.set_flag(Flags::V, 1) } else { cpu.set_flag(Flags::V, 0) }
        cpu.A = (cpu.A as i8).wrapping_add(operand as i8).wrapping_add(carry as i8) as u8;
        opc.check_and_set_n(cpu.A, cpu);
        opc.check_and_set_z(cpu.A, cpu);
    }
}

pub fn adc_69(opc: &mut Opcode, cpu: &mut MOS6510) {
    let operand: u8 = opc.fetch(cpu);
    opc.current_operation.push_str(format!("ADC #${:02X}", operand).as_str());
    adc(opc, cpu, operand);
    cpu.cycle += 2;
}

pub fn adc_6d(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.absolute(cpu);
    opc.current_operation.push_str(format!("ADC ${:02X}{:02X}", high.unwrap(), low).as_str());
    adc(opc, cpu, operand);
    cpu.cycle += 4;
}

pub fn adc_7d(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.absolute_indexed(cpu.X as u16, cpu);
    opc.current_operation.push_str(format!("ADC ${:02X}{:02X}, X", high.unwrap(), low).as_str());
    adc(opc, cpu, operand);
    cpu.cycle += 4;
}

pub fn adc_79(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.absolute_indexed(cpu.Y as u16, cpu);
    opc.current_operation.push_str(format!("ADC ${:02X}{:02X}, Y", high.unwrap(), low).as_str());
    adc(opc, cpu, operand);
    cpu.cycle += 4;
}

pub fn adc_65(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.zero_page(cpu);
    opc.current_operation.push_str(format!("ADC ${:02X}", low).as_str());
    adc(opc, cpu, operand);
    cpu.cycle += 3;
}

pub fn adc_75(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.zero_page_indexed(cpu.X, cpu);
    opc.current_operation.push_str(format!("ADC ${:02X}, X", low).as_str());
    adc(opc, cpu, operand);
    cpu.cycle += 4;
}

pub fn adc_61(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.zero_page_indexed_indirect(cpu.X, cpu);
    opc.current_operation.push_str(format!("ADC (${:02X}, X)", low).as_str());
    adc(opc, cpu, operand);
    cpu.cycle += 6;
}

pub fn adc_71(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.zero_page_indirect_indexed(cpu.Y, cpu);
    opc.current_operation.push_str(format!("ADC (${:02X}), Y", low).as_str());
    adc(opc, cpu, operand);
    cpu.cycle += 5;
}

pub fn sbc(opc: &mut Opcode, cpu: &mut MOS6510, operand: u8) {
    let carry: u8 = if cpu.get_flag(Flags::C) { 1 } else { 0 };
    if cpu.get_flag(Flags::D) {
        cpu.A = opc.bcd(cpu.A).wrapping_sub(opc.bcd(operand)).wrapping_sub(carry);
    } else {
        let result: i16 = (cpu.A as i8 - operand as i8 - carry as i8) as i16;
        if result >= 0 { cpu.set_flag(Flags::C, 1) } else { cpu.set_flag(Flags::C, 0) }
        if result > 127 || result < -127 { cpu.set_flag(Flags::V, 1) } else { cpu.set_flag(Flags::V, 0) }
        cpu.A = (cpu.A as i8).wrapping_sub(operand as i8).wrapping_sub(carry as i8) as u8;
        opc.check_and_set_n(cpu.A, cpu);
        opc.check_and_set_z(cpu.A, cpu);
    }
}

pub fn sbc_e9(opc: &mut Opcode, cpu: &mut MOS6510) {
    let operand: u8 = opc.fetch(cpu);
    opc.current_operation.push_str(format!("SBC #${:02X}", operand).as_str());
    sbc(opc, cpu, operand);
    cpu.cycle += 2;
}

pub fn sbc_ed(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.absolute(cpu);
    opc.current_operation.push_str(format!("SBC ${:02X}{:02X}", high.unwrap(), low).as_str());
    sbc(opc, cpu, operand);
    cpu.cycle += 4;
}

pub fn sbc_fd(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.absolute_indexed(cpu.X as u16, cpu);
    opc.current_operation.push_str(format!("SBC ${:02X}{:02X}, X", high.unwrap(), low).as_str());
    sbc(opc, cpu, operand);
    cpu.cycle += 4;
}

pub fn sbc_f9(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.absolute_indexed(cpu.Y as u16, cpu);
    opc.current_operation.push_str(format!("SBC ${:02X}{:02X}, Y", high.unwrap(), low).as_str());
    sbc(opc, cpu, operand);
    cpu.cycle += 4;
}

pub fn sbc_e5(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.zero_page(cpu);
    opc.current_operation.push_str(format!("SBC ${:02X}", low).as_str());
    sbc(opc, cpu, operand);
    cpu.cycle += 3;
}

pub fn sbc_f5(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.zero_page_indexed(cpu.X, cpu);
    opc.current_operation.push_str(format!("SBC ${:02X}, X", low).as_str());
    sbc(opc, cpu, operand);
    cpu.cycle += 4;
}

pub fn sbc_e1(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.zero_page_indexed_indirect(cpu.X, cpu);
    opc.current_operation.push_str(format!("SBC (${:02X}, X)", low).as_str());
    sbc(opc, cpu, operand);
    cpu.cycle += 6;
}

pub fn sbc_f1(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.zero_page_indirect_indexed(cpu.Y, cpu);
    opc.current_operation.push_str(format!("SBC (${:02X}), Y", low).as_str());
    sbc(opc, cpu, operand);
    cpu.cycle += 5;
}
