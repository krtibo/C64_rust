use super::Opcode;
use super::AddrReturn;
use super::super::MOS6510;
use super::super::Flags;

pub fn sl(opc: &mut Opcode, cpu: &mut MOS6510, operand: u8, address: u16) {
    cpu.set_flag(Flags::C, opc.get_bit(operand, 7));
    cpu.set_flag(Flags::N, opc.get_bit(operand, 6));
    let result: u8 = operand << 1;
    cpu.mmu.write(result, address);
    opc.check_and_set_z(result, cpu);
}

pub fn sr(opc: &mut Opcode, cpu: &mut MOS6510, operand: u8, address: u16) {
    cpu.set_flag(Flags::N, 0);
    cpu.set_flag(Flags::C, opc.get_bit(operand, 0));
    let result: u8 = operand >> 1;
    cpu.mmu.write(result, address);
    opc.check_and_set_z(result, cpu);
}

pub fn rol(opc: &mut Opcode, cpu: &mut MOS6510, operand: u8, address: u16) {
    cpu.set_flag(Flags::N, opc.get_bit(operand, 6));
    let input_bit_7: u8 = opc.get_bit(operand, 7);
    let mut result: u8 = operand << 1;
    if cpu.get_flag(Flags::C) { result = result | 0b0000_0001 }
    cpu.mmu.write(result, address);
    cpu.set_flag(Flags::C, input_bit_7);
    opc.check_and_set_z(result, cpu);
}

pub fn ror(opc: &mut Opcode, cpu: &mut MOS6510, operand: u8, address: u16) {
    if cpu.get_flag(Flags::C) { cpu.set_flag(Flags::N, 1) } else { cpu.set_flag(Flags::N, 0) }
    let input_bit_0: u8 = opc.get_bit(operand, 0);
    let mut result: u8 = operand >> 1;
    if cpu.get_flag(Flags::C) { result = result | 0b1000_0000 }
    cpu.mmu.write(result, address);
    cpu.set_flag(Flags::C, input_bit_0);
    opc.check_and_set_z(result, cpu);
}

pub fn asl_0a(opc: &mut Opcode, cpu: &mut MOS6510) {
    opc.current_operation.push_str(format!("ASL A").as_str());
    cpu.set_flag(Flags::C, opc.get_bit(cpu.A, 7));
    cpu.set_flag(Flags::N, opc.get_bit(cpu.A, 6));
    cpu.A = cpu.A << 1;
    opc.check_and_set_z(cpu.A, cpu);
    cpu.cycle += 2;
}

pub fn asl_0e(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { mut operand, address, high, low } = opc.absolute(cpu);
    opc.current_operation.push_str(format!("ASL ${:02X}{:02X}", high.unwrap(), low).as_str());
    sl(opc, cpu, operand, address);
    cpu.cycle += 6;
}

pub fn asl_1e(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { mut operand, address, high, low } = opc.absolute_indexed(cpu.X as u16, cpu);
    opc.current_operation.push_str(format!("ASL ${:02X}{:02X}, X", high.unwrap(), low).as_str());
    sl(opc, cpu, operand, address);
    cpu.cycle += 7;
}

pub fn asl_06(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { mut operand, address, high, low } = opc.zero_page(cpu);
    opc.current_operation.push_str(format!("ASL ${:02X}", low).as_str());
    sl(opc, cpu, operand, address);
    cpu.cycle += 5;
}

pub fn asl_16(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { mut operand, address, high, low } = opc.zero_page_indexed(cpu.X, cpu);
    opc.current_operation.push_str(format!("ASL ${:02X}, X", low).as_str());
    sl(opc, cpu, operand, address);
    cpu.cycle += 6;
}

pub fn lsr_4a(opc: &mut Opcode, cpu: &mut MOS6510) {
    opc.current_operation.push_str(format!("LSR A").as_str());
    cpu.set_flag(Flags::N, 0);
    cpu.set_flag(Flags::C, opc.get_bit(cpu.A, 0));
    cpu.A = cpu.A >> 1;
    opc.check_and_set_z(cpu.A, cpu);
    cpu.cycle += 2;
}

pub fn lsr_4e(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { mut operand, address, high, low } = opc.absolute(cpu);
    opc.current_operation.push_str(format!("LSR ${:02X}{:02X}", high.unwrap(), low).as_str());
    sr(opc, cpu, operand, address);
    cpu.cycle += 6;
}

pub fn lsr_5e(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { mut operand, address, high, low } = opc.absolute_indexed(cpu.X as u16, cpu);
    opc.current_operation.push_str(format!("LSR ${:02X}{:02X}, X", high.unwrap(), low).as_str());
    sr(opc, cpu, operand, address);
    cpu.cycle += 7;
}

pub fn lsr_46(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { mut operand, address, high, low } = opc.zero_page(cpu);
    opc.current_operation.push_str(format!("LSR ${:02X}", low).as_str());
    sr(opc, cpu, operand, address);
    cpu.cycle += 5;
}

pub fn lsr_56(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { mut operand, address, high, low } = opc.zero_page_indexed(cpu.X, cpu);
    opc.current_operation.push_str(format!("LSR ${:02X}, X", low).as_str());
    sr(opc, cpu, operand, address);
    cpu.cycle += 6;
}

pub fn rol_2a(opc: &mut Opcode, cpu: &mut MOS6510) {
    opc.current_operation.push_str(format!("ROL A").as_str());
    cpu.set_flag(Flags::N, opc.get_bit(cpu.A, 6));
    let input_bit_7: u8 = opc.get_bit(cpu.A, 7);
    cpu.A = cpu.A << 1;
    if cpu.get_flag(Flags::C) { cpu.A = cpu.A | 0b0000_0001 }
    cpu.set_flag(Flags::C, input_bit_7);
    opc.check_and_set_z(cpu.A, cpu);
    cpu.cycle += 2;
}

pub fn rol_2e(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { mut operand, address, high, low } = opc.absolute(cpu);
    opc.current_operation.push_str(format!("ROL ${:02X}{:02X}", high.unwrap(), low).as_str());
    rol(opc, cpu, operand, address);
    cpu.cycle += 6;
}

pub fn rol_3e(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { mut operand, address, high, low } = opc.absolute_indexed(cpu.X as u16, cpu);
    opc.current_operation.push_str(format!("ROL ${:02X}{:02X}, X", high.unwrap(), low).as_str());
    rol(opc, cpu, operand, address);
    cpu.cycle += 7;
}

pub fn rol_26(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { mut operand, address, high, low } = opc.zero_page(cpu);
    opc.current_operation.push_str(format!("ROL ${:02X}", low).as_str());
    rol(opc, cpu, operand, address);
    cpu.cycle += 5;
}

pub fn rol_36(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { mut operand, address, high, low } = opc.zero_page_indexed(cpu.X, cpu);
    opc.current_operation.push_str(format!("ROL ${:02X}, X", low).as_str());
    rol(opc, cpu, operand, address);
    cpu.cycle += 6;
}

pub fn ror_6a(opc: &mut Opcode, cpu: &mut MOS6510) {
    opc.current_operation.push_str(format!("ROR A").as_str());
    if cpu.get_flag(Flags::C) { cpu.set_flag(Flags::N, 1) } else { cpu.set_flag(Flags::N, 0) }
    let input_bit_0: u8 = opc.get_bit(cpu.A, 0);
    cpu.A = cpu.A >> 1;
    if cpu.get_flag(Flags::C) { cpu.A = cpu.A | 0b1000_0000 }
    cpu.set_flag(Flags::C, input_bit_0);
    opc.check_and_set_z(cpu.A, cpu);
    cpu.cycle += 2;
}

pub fn ror_6e(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { mut operand, address, high, low } = opc.absolute(cpu);
    opc.current_operation.push_str(format!("ROR ${:02X}{:02X}", high.unwrap(), low).as_str());
    ror(opc, cpu, operand, address);
    cpu.cycle += 6;
}

pub fn ror_7e(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { mut operand, address, high, low } = opc.absolute_indexed(cpu.X as u16, cpu);
    opc.current_operation.push_str(format!("ROR ${:02X}{:02X}, X", high.unwrap(), low).as_str());
    ror(opc, cpu, operand, address);
    cpu.cycle += 7;
}

pub fn ror_66(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { mut operand, address, high, low } = opc.zero_page(cpu);
    opc.current_operation.push_str(format!("ROR ${:02X}", low).as_str());
    ror(opc, cpu, operand, address);
    cpu.cycle += 5;
}

pub fn ror_76(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { mut operand, address, high, low } = opc.zero_page_indexed(cpu.X, cpu);
    opc.current_operation.push_str(format!("ROR ${:02X}, X", low).as_str());
    ror(opc, cpu, operand, address);
    cpu.cycle += 6;
}
