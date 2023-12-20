use super::Opcode;
use super::AddrReturn;
use super::super::MOS6510;
use super::super::Flags;

pub fn cmp_c9(opc: &mut Opcode, cpu : &mut MOS6510) {
    let operand: u8 = opc.fetch(cpu);
    opc.current_operation.push_str(format!("CMP #${:02X}", operand).as_str());
    let result: u8 = cpu.A.wrapping_sub(operand);
    if result == 0 { cpu.set_flag(Flags::Z, 1) } else { cpu.set_flag(Flags::Z, 0) }
    if operand <= cpu.A { cpu.set_flag(Flags::C, 1) } else { cpu.set_flag(Flags::C, 0) }
    opc.check_and_set_n(result, cpu);
    cpu.cycle += 2;
}

pub fn cmp_cd(opc: &mut Opcode, cpu : &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.absolute(cpu);
    opc.current_operation.push_str(format!("CMP ${:02X}{:02X}", high.unwrap(), low).as_str());
    let result: u8 = cpu.A.wrapping_sub(operand);
    if result == 0 { cpu.set_flag(Flags::Z, 1) } else { cpu.set_flag(Flags::Z, 0) }
    if operand <= cpu.A { cpu.set_flag(Flags::C, 1) } else { cpu.set_flag(Flags::C, 0) }
    opc.check_and_set_n(result, cpu);
    cpu.cycle += 4;
}

pub fn cmp_dd(opc: &mut Opcode, cpu : &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.absolute_indexed(cpu.X as u16, cpu);
    opc.current_operation.push_str(format!("CMP ${:02X}{:02X}, X", high.unwrap(), low).as_str());
    let result: u8 = cpu.A.wrapping_sub(operand);
    if result == 0 { cpu.set_flag(Flags::Z, 1) } else { cpu.set_flag(Flags::Z, 0) }
    if operand <= cpu.A { cpu.set_flag(Flags::C, 1) } else { cpu.set_flag(Flags::C, 0) }
    opc.check_and_set_n(result, cpu);
    cpu.cycle += 4;
}

pub fn cmp_d9(opc: &mut Opcode, cpu : &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.absolute_indexed(cpu.Y as u16, cpu);
    opc.current_operation.push_str(format!("CMP ${:02X}{:02X}, Y", high.unwrap(), low).as_str());
    let result: u8 = cpu.A.wrapping_sub(operand);
    if result == 0 { cpu.set_flag(Flags::Z, 1) } else { cpu.set_flag(Flags::Z, 0) }
    if operand <= cpu.A { cpu.set_flag(Flags::C, 1) } else { cpu.set_flag(Flags::C, 0) }
    opc.check_and_set_n(result, cpu);
    cpu.cycle += 4;
}

pub fn cmp_c5(opc: &mut Opcode, cpu : &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.zero_page(cpu);
    opc.current_operation.push_str(format!("CMP ${:02X}", low).as_str());
    let result: u8 = cpu.A.wrapping_sub(operand);
    if result == 0 { cpu.set_flag(Flags::Z, 1) } else { cpu.set_flag(Flags::Z, 0) }
    if operand <= cpu.A { cpu.set_flag(Flags::C, 1) } else { cpu.set_flag(Flags::C, 0) }
    opc.check_and_set_n(result, cpu);
    cpu.cycle += 3;
}

pub fn cmp_d5(opc: &mut Opcode, cpu : &mut MOS6510) {
    let AddrReturn { mut operand, address, high, low } = opc.zero_page_indexed(cpu.X, cpu);
    opc.current_operation.push_str(format!("CMP ${:02X}, X", low).as_str());
    let result: u8 = cpu.A.wrapping_sub(operand);
    if result == 0 { cpu.set_flag(Flags::Z, 1) } else { cpu.set_flag(Flags::Z, 0) }
    if operand <= cpu.A { cpu.set_flag(Flags::C, 1) } else { cpu.set_flag(Flags::C, 0) }
    opc.check_and_set_n(result, cpu);
    cpu.cycle += 4;
}

pub fn cmp_c1(opc: &mut Opcode, cpu : &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.zero_page_indexed_indirect(cpu.X, cpu);
    opc.current_operation.push_str(format!("CMP (${:02X}, X)", low).as_str());
    let result: u8 = cpu.A.wrapping_sub(operand);
    if result == 0 { cpu.set_flag(Flags::Z, 1) } else { cpu.set_flag(Flags::Z, 0) }
    if operand <= cpu.A { cpu.set_flag(Flags::C, 1) } else { cpu.set_flag(Flags::C, 0) }
    opc.check_and_set_n(operand, cpu);
    cpu.cycle += 6;
}

pub fn cmp_d1(opc: &mut Opcode, cpu : &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.zero_page_indirect_indexed(cpu.X, cpu);
    opc.current_operation.push_str(format!("CMP (${:02X}), Y", low).as_str());
    let result: u8 = cpu.A.wrapping_sub(operand);
    if result == 0 { cpu.set_flag(Flags::Z, 1) } else { cpu.set_flag(Flags::Z, 0) }
    if operand <= cpu.A { cpu.set_flag(Flags::C, 1) } else { cpu.set_flag(Flags::C, 0) }
    opc.check_and_set_n(operand, cpu);
    cpu.cycle += 5;
}

pub fn cpx_e0(opc: &mut Opcode, cpu : &mut MOS6510) {
    let operand: u8 = opc.fetch(cpu);
    opc.current_operation.push_str(format!("CPX #${:02X}", operand).as_str());
    let result: u8 = cpu.X.wrapping_sub(operand);
    if result == 0 { cpu.set_flag(Flags::Z, 1) } else { cpu.set_flag(Flags::Z, 0) }
    if operand <= cpu.X { cpu.set_flag(Flags::C, 1) } else { cpu.set_flag(Flags::C, 0) }
    opc.check_and_set_n(result, cpu);
    cpu.cycle += 2;
}

pub fn cpx_ec(opc: &mut Opcode, cpu : &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.absolute(cpu);
    opc.current_operation.push_str(format!("CPX ${:02X}{:02X}", high.unwrap(), low).as_str());
    let result: u8 = cpu.X.wrapping_sub(operand);
    if result == 0 { cpu.set_flag(Flags::Z, 1) } else { cpu.set_flag(Flags::Z, 0) }
    if operand <= cpu.X { cpu.set_flag(Flags::C, 1) } else { cpu.set_flag(Flags::C, 0) }
    opc.check_and_set_n(result, cpu);
    cpu.cycle += 4;
}

pub fn cpx_e4(opc: &mut Opcode, cpu : &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.zero_page(cpu);
    opc.current_operation.push_str(format!("CPX ${:02X}", low).as_str());
    let result: u8 = cpu.X.wrapping_sub(operand);
    if result == 0 { cpu.set_flag(Flags::Z, 1) } else { cpu.set_flag(Flags::Z, 0) }
    if operand <= cpu.X { cpu.set_flag(Flags::C, 1) } else { cpu.set_flag(Flags::C, 0) }
    opc.check_and_set_n(result, cpu);
    cpu.cycle += 3;
}

pub fn cpy_c0(opc: &mut Opcode, cpu : &mut MOS6510) {
    let operand: u8 = opc.fetch(cpu);
    opc.current_operation.push_str(format!("CPY #${:02X}", operand).as_str());
    let result: u8 = cpu.Y.wrapping_sub(operand);
    if result == 0 { cpu.set_flag(Flags::Z, 1) } else { cpu.set_flag(Flags::Z, 0) }
    if operand <= cpu.Y { cpu.set_flag(Flags::C, 1) } else { cpu.set_flag(Flags::C, 0) }
    opc.check_and_set_n(result, cpu);
    cpu.cycle += 2;
}

pub fn cpy_cc(opc: &mut Opcode, cpu : &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.absolute(cpu);
    opc.current_operation.push_str(format!("CPY ${:02X}{:02X}", high.unwrap(), low).as_str());
    let result: u8 = cpu.Y.wrapping_sub(operand);
    if result == 0 { cpu.set_flag(Flags::Z, 1) } else { cpu.set_flag(Flags::Z, 0) }
    if operand <= cpu.Y { cpu.set_flag(Flags::C, 1) } else { cpu.set_flag(Flags::C, 0) }
    opc.check_and_set_n(result, cpu);
    cpu.cycle += 4;
}

pub fn cpy_c4(opc: &mut Opcode, cpu : &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.zero_page(cpu);
    opc.current_operation.push_str(format!("CPY ${:02X}", low).as_str());
    let result: u8 = cpu.Y.wrapping_sub(operand);
    if result == 0 { cpu.set_flag(Flags::Z, 1) } else { cpu.set_flag(Flags::Z, 0) }
    if operand <= cpu.Y { cpu.set_flag(Flags::C, 1) } else { cpu.set_flag(Flags::C, 0) }
    opc.check_and_set_n(result, cpu);
    cpu.cycle += 3;
}

pub fn adc_69(opc: &mut Opcode, cpu : &mut MOS6510) {
    let operand: u8 = opc.fetch(cpu);
    opc.current_operation.push_str(format!("ADC #${:02X}", operand).as_str());
    let carry: u8 = if cpu.get_flag(Flags::C) { 1 } else { 0 };
    if cpu.get_flag(Flags::D) {
        cpu.A = opc.bcd(cpu.A).wrapping_add(opc.bcd(operand)).wrapping_add(carry);
    } else {
        let result: i16 = (cpu.A as i8 + operand as i8) as i16;
        if result > 255 { cpu.set_flag(Flags::C, 1) }
        if result > 127 || result < -127 { cpu.set_flag(Flags::V, 1) }
        cpu.A = cpu.A.wrapping_add(operand).wrapping_add(carry);
        opc.check_and_set_n(cpu.A, cpu);
        opc.check_and_set_z(cpu.A, cpu);
    }
    cpu.cycle += 2;
}

pub fn adc_6d(opc: &mut Opcode, cpu : &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.absolute(cpu);
    opc.current_operation.push_str(format!("ADC ${:02X}{:02X}", high.unwrap(), low).as_str());
    let carry: u8 = if cpu.get_flag(Flags::C) { 1 } else { 0 };
    if cpu.get_flag(Flags::D) {
        cpu.A = opc.bcd(cpu.A).wrapping_add(opc.bcd(operand)).wrapping_add(carry);
    } else {
        let result: i16 = (cpu.A as i8 + operand as i8) as i16;
        if result > 255 { cpu.set_flag(Flags::C, 1) }
        if result > 127 || result < -127 { cpu.set_flag(Flags::V, 1) }
        cpu.A = cpu.A.wrapping_add(operand).wrapping_add(carry);
        opc.check_and_set_n(cpu.A, cpu);
        opc.check_and_set_z(cpu.A, cpu);
    }
    cpu.cycle += 4;
}

pub fn adc_7d(opc: &mut Opcode, cpu : &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.absolute_indexed(cpu.X as u16, cpu);
    opc.current_operation.push_str(format!("ADC ${:02X}{:02X}, X", high.unwrap(), low).as_str());
    let carry: u8 = if cpu.get_flag(Flags::C) { 1 } else { 0 };
    if cpu.get_flag(Flags::D) {
        cpu.A = opc.bcd(cpu.A).wrapping_add(opc.bcd(operand)).wrapping_add(carry);
    } else {
        let result: i16 = (cpu.A as i8 + operand as i8) as i16;
        if result > 255 { cpu.set_flag(Flags::C, 1) }
        if result > 127 || result < -127 { cpu.set_flag(Flags::V, 1) }
        cpu.A = cpu.A.wrapping_add(operand).wrapping_add(carry);
        opc.check_and_set_n(cpu.A, cpu);
        opc.check_and_set_z(cpu.A, cpu);
    }
    cpu.cycle += 4;
}

pub fn adc_79(opc: &mut Opcode, cpu : &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.absolute_indexed(cpu.Y as u16, cpu);
    opc.current_operation.push_str(format!("ADC ${:02X}{:02X}, Y", high.unwrap(), low).as_str());
    let carry: u8 = if cpu.get_flag(Flags::C) { 1 } else { 0 };
    if cpu.get_flag(Flags::D) {
        cpu.A = opc.bcd(cpu.A).wrapping_add(opc.bcd(operand)).wrapping_add(carry);
    } else {
        let result: i16 = (cpu.A as i8 + operand as i8) as i16;
        if result > 255 { cpu.set_flag(Flags::C, 1) }
        if result > 127 || result < -127 { cpu.set_flag(Flags::V, 1) }
        cpu.A = cpu.A.wrapping_add(operand).wrapping_add(carry);
        opc.check_and_set_n(cpu.A, cpu);
        opc.check_and_set_z(cpu.A, cpu);
    }
    cpu.cycle += 4;
}

pub fn adc_65(opc: &mut Opcode, cpu : &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.zero_page(cpu);
    opc.current_operation.push_str(format!("ADC ${:02X}", low).as_str());
    let carry: u8 = if cpu.get_flag(Flags::C) { 1 } else { 0 };
    if cpu.get_flag(Flags::D) {
        cpu.A = opc.bcd(cpu.A).wrapping_add(opc.bcd(operand)).wrapping_add(carry);
    } else {
        let result: i16 = (cpu.A as i8 + operand as i8) as i16;
        if result > 255 { cpu.set_flag(Flags::C, 1) }
        if result > 127 || result < -127 { cpu.set_flag(Flags::V, 1) }
        cpu.A = cpu.A.wrapping_add(operand).wrapping_add(carry);
        opc.check_and_set_n(cpu.A, cpu);
        opc.check_and_set_z(cpu.A, cpu);
    }
    cpu.cycle += 3;
}

pub fn adc_75(opc: &mut Opcode, cpu : &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.zero_page_indexed(cpu.X, cpu);
    opc.current_operation.push_str(format!("ADC ${:02X}, X", low).as_str());
    let carry: u8 = if cpu.get_flag(Flags::C) { 1 } else { 0 };
    if cpu.get_flag(Flags::D) {
        cpu.A = opc.bcd(cpu.A).wrapping_add(opc.bcd(operand)).wrapping_add(carry);
    } else {
        let result: i16 = (cpu.A as i8 + operand as i8) as i16;
        if result > 255 { cpu.set_flag(Flags::C, 1) }
        if result > 127 || result < -127 { cpu.set_flag(Flags::V, 1) }
        cpu.A = cpu.A.wrapping_add(operand).wrapping_add(carry);
        opc.check_and_set_n(cpu.A, cpu);
        opc.check_and_set_z(cpu.A, cpu);
    }
    cpu.cycle += 4;
}

pub fn adc_61(opc: &mut Opcode, cpu : &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.zero_page_indexed_indirect(cpu.X, cpu);
    opc.current_operation.push_str(format!("ADC (${:02X}, X)", low).as_str());
    let carry: u8 = if cpu.get_flag(Flags::C) { 1 } else { 0 };
    if cpu.get_flag(Flags::D) {
        cpu.A = opc.bcd(cpu.A).wrapping_add(opc.bcd(operand)).wrapping_add(carry);
    } else {
        let result: i16 = (cpu.A as i8 + operand as i8) as i16;
        if result > 255 { cpu.set_flag(Flags::C, 1) }
        if result > 127 || result < -127 { cpu.set_flag(Flags::V, 1) }
        cpu.A = cpu.A.wrapping_add(operand).wrapping_add(carry);
        opc.check_and_set_n(cpu.A, cpu);
        opc.check_and_set_z(cpu.A, cpu);
    }
    cpu.cycle += 6;
}

pub fn adc_71(opc: &mut Opcode, cpu : &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.zero_page_indirect_indexed(cpu.Y, cpu);
    opc.current_operation.push_str(format!("ADC (${:02X}), Y", low).as_str());
    let carry: u8 = if cpu.get_flag(Flags::C) { 1 } else { 0 };
    if cpu.get_flag(Flags::D) {
        cpu.A = opc.bcd(cpu.A).wrapping_add(opc.bcd(operand)).wrapping_add(carry);
    } else {
        let result: i16 = (cpu.A as i8 + operand as i8) as i16;
        if result > 255 { cpu.set_flag(Flags::C, 1) }
        if result > 127 || result < -127 { cpu.set_flag(Flags::V, 1) }
        cpu.A = cpu.A.wrapping_add(operand).wrapping_add(carry);
        opc.check_and_set_n(cpu.A, cpu);
        opc.check_and_set_z(cpu.A, cpu);
    }
    cpu.cycle += 5;
}

pub fn sbc_e9(opc: &mut Opcode, cpu : &mut MOS6510) {
    let operand: u8 = opc.fetch(cpu);
    opc.current_operation.push_str(format!("SBC #${:02X}", operand).as_str());
    let carry: u8 = if cpu.get_flag(Flags::C) { 1 } else { 0 };
    if cpu.get_flag(Flags::D) {
        cpu.A = opc.bcd(cpu.A).wrapping_sub(opc.bcd(operand)).wrapping_sub(carry);
    } else {
        let result: i16 = (cpu.A as i8 - operand as i8) as i16;
        if result >= 0 { cpu.set_flag(Flags::C, 1) } else { cpu.set_flag(Flags::C, 0) }
        if result > 127 || result < -127 { cpu.set_flag(Flags::V, 1) } else { cpu.set_flag(Flags::V, 0) }
        cpu.A = cpu.A.wrapping_sub(operand).wrapping_sub(carry);
        opc.check_and_set_n(cpu.A, cpu);
        opc.check_and_set_z(cpu.A, cpu);
    }
    cpu.cycle += 2;
}

pub fn sbc_ed(opc: &mut Opcode, cpu : &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.absolute(cpu);
    opc.current_operation.push_str(format!("SBC ${:02X}{:02X}", high.unwrap(), low).as_str());
    let carry: u8 = if cpu.get_flag(Flags::C) { 1 } else { 0 };
    if cpu.get_flag(Flags::D) {
        cpu.A = opc.bcd(cpu.A).wrapping_sub(opc.bcd(operand)).wrapping_sub(carry);
    } else {
        let result: i16 = (cpu.A as i8 - operand as i8) as i16;
        if result >= 0 { cpu.set_flag(Flags::C, 1) } else { cpu.set_flag(Flags::C, 0) }
        if result > 127 || result < -127 { cpu.set_flag(Flags::V, 1) } else { cpu.set_flag(Flags::V, 0) }
        cpu.A = cpu.A.wrapping_sub(operand).wrapping_sub(carry);
        opc.check_and_set_n(cpu.A, cpu);
        opc.check_and_set_z(cpu.A, cpu);
    }
    cpu.cycle += 4;
}

pub fn sbc_fd(opc: &mut Opcode, cpu : &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.absolute_indexed(cpu.X as u16, cpu);
    opc.current_operation.push_str(format!("SBC ${:02X}{:02X}, X", high.unwrap(), low).as_str());
    let carry: u8 = if cpu.get_flag(Flags::C) { 1 } else { 0 };
    if cpu.get_flag(Flags::D) {
        cpu.A = opc.bcd(cpu.A).wrapping_sub(opc.bcd(operand)).wrapping_sub(carry);
    } else {
        let result: i16 = (cpu.A as i8 - operand as i8) as i16;
        if result >= 0 { cpu.set_flag(Flags::C, 1) } else { cpu.set_flag(Flags::C, 0) }
        if result > 127 || result < -127 { cpu.set_flag(Flags::V, 1) } else { cpu.set_flag(Flags::V, 0) }
        cpu.A = cpu.A.wrapping_sub(operand).wrapping_sub(carry);
        opc.check_and_set_n(cpu.A, cpu);
        opc.check_and_set_z(cpu.A, cpu);
    }
    cpu.cycle += 4;
}

pub fn sbc_f9(opc: &mut Opcode, cpu : &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.absolute_indexed(cpu.Y as u16, cpu);
    opc.current_operation.push_str(format!("SBC ${:02X}{:02X}, Y", high.unwrap(), low).as_str());
    let carry: u8 = if cpu.get_flag(Flags::C) { 1 } else { 0 };
    if cpu.get_flag(Flags::D) {
        cpu.A = opc.bcd(cpu.A).wrapping_sub(opc.bcd(operand)).wrapping_sub(carry);
    } else {
        let result: i16 = (cpu.A as i8 - operand as i8) as i16;
        if result >= 0 { cpu.set_flag(Flags::C, 1) } else { cpu.set_flag(Flags::C, 0) }
        if result > 127 || result < -127 { cpu.set_flag(Flags::V, 1) } else { cpu.set_flag(Flags::V, 0) }
        cpu.A = cpu.A.wrapping_sub(operand).wrapping_sub(carry);
        opc.check_and_set_n(cpu.A, cpu);
        opc.check_and_set_z(cpu.A, cpu);
    }
    cpu.cycle += 4;
}

pub fn sbc_e5(opc: &mut Opcode, cpu : &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.zero_page(cpu);
    opc.current_operation.push_str(format!("SBC ${:02X}", low).as_str());
    let carry: u8 = if cpu.get_flag(Flags::C) { 1 } else { 0 };
    if cpu.get_flag(Flags::D) {
        cpu.A = opc.bcd(cpu.A).wrapping_sub(opc.bcd(operand)).wrapping_sub(carry);
    } else {
        let result: i16 = (cpu.A as i8 - operand as i8) as i16;
        if result >= 0 { cpu.set_flag(Flags::C, 1) } else { cpu.set_flag(Flags::C, 0) }
        if result > 127 || result < -127 { cpu.set_flag(Flags::V, 1) } else { cpu.set_flag(Flags::V, 0) }
        cpu.A = cpu.A.wrapping_sub(operand).wrapping_sub(carry);
        opc.check_and_set_n(cpu.A, cpu);
        opc.check_and_set_z(cpu.A, cpu);
    }
    cpu.cycle += 3;
}

pub fn sbc_f5(opc: &mut Opcode, cpu : &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.zero_page_indexed(cpu.X, cpu);
    opc.current_operation.push_str(format!("SBC ${:02X}, X", low).as_str());
    let carry: u8 = if cpu.get_flag(Flags::C) { 1 } else { 0 };
    if cpu.get_flag(Flags::D) {
        cpu.A = opc.bcd(cpu.A).wrapping_sub(opc.bcd(operand)).wrapping_sub(carry);
    } else {
        let result: i16 = (cpu.A as i8 - operand as i8) as i16;
        if result >= 0 { cpu.set_flag(Flags::C, 1) } else { cpu.set_flag(Flags::C, 0) }
        if result > 127 || result < -127 { cpu.set_flag(Flags::V, 1) } else { cpu.set_flag(Flags::V, 0) }
        cpu.A = cpu.A.wrapping_sub(operand).wrapping_sub(carry);
        opc.check_and_set_n(cpu.A, cpu);
        opc.check_and_set_z(cpu.A, cpu);
    }
    cpu.cycle += 4;
}

pub fn sbc_e1(opc: &mut Opcode, cpu : &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.zero_page_indexed_indirect(cpu.X, cpu);
    opc.current_operation.push_str(format!("SBC (${:02X}, X)", low).as_str());
    let carry: u8 = if cpu.get_flag(Flags::C) { 1 } else { 0 };
    if cpu.get_flag(Flags::D) {
        cpu.A = opc.bcd(cpu.A).wrapping_sub(opc.bcd(operand)).wrapping_sub(carry);
    } else {
        let result: i16 = (cpu.A as i8 - operand as i8) as i16;
        if result >= 0 { cpu.set_flag(Flags::C, 1) } else { cpu.set_flag(Flags::C, 0) }
        if result > 127 || result < -127 { cpu.set_flag(Flags::V, 1) } else { cpu.set_flag(Flags::V, 0) }
        cpu.A = cpu.A.wrapping_sub(operand).wrapping_sub(carry);
        opc.check_and_set_n(cpu.A, cpu);
        opc.check_and_set_z(cpu.A, cpu);
    }
    cpu.cycle += 6;
}

pub fn sbc_f1(opc: &mut Opcode, cpu : &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.zero_page_indirect_indexed(cpu.Y, cpu);
    opc.current_operation.push_str(format!("SBC (${:02X}), Y", low).as_str());
    let carry: u8 = if cpu.get_flag(Flags::C) { 1 } else { 0 };
    if cpu.get_flag(Flags::D) {
        cpu.A = opc.bcd(cpu.A).wrapping_sub(opc.bcd(operand)).wrapping_sub(carry);
    } else {
        let result: i16 = (cpu.A as i8 - operand as i8) as i16;
        if result >= 0 { cpu.set_flag(Flags::C, 1) } else { cpu.set_flag(Flags::C, 0) }
        if result > 127 || result < -127 { cpu.set_flag(Flags::V, 1) } else { cpu.set_flag(Flags::V, 0) }
        cpu.A = cpu.A.wrapping_sub(operand).wrapping_sub(carry);
        opc.check_and_set_n(cpu.A, cpu);
        opc.check_and_set_z(cpu.A, cpu);
    }
    cpu.cycle += 5;
}
