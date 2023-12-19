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