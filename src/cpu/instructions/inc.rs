use super::Opcode;
use super::AddrReturn;
use super::super::MOS6510;

pub fn dec_ce(opc: &mut Opcode, cpu : &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.absolute(cpu);
    opc.current_operation.push_str(format!("DEC ${:02X}{:02X}", high.unwrap(), low).as_str());
    let value = operand.wrapping_sub(1);
    cpu.mmu.write(value, address);
    opc.check_and_set_n(value, cpu);
    opc.check_and_set_z(value, cpu);
    cpu.cycle += 6;
}

pub fn dec_de(opc: &mut Opcode, cpu : &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.absolute_indexed(cpu.X as u16, cpu);
    opc.current_operation.push_str(format!("DEC ${:02X}{:02X}, X", high.unwrap(), low).as_str());
    let value = operand.wrapping_sub(1);
    cpu.mmu.write(value, address);
    opc.check_and_set_n(value, cpu);
    opc.check_and_set_z(value, cpu);
    cpu.cycle += 7;
}

pub fn dec_c6(opc: &mut Opcode, cpu : &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.zero_page(cpu);
    opc.current_operation.push_str(format!("DEC ${:02X}", low).as_str());
    let value = operand.wrapping_sub(1);
    cpu.mmu.write(value, address);
    opc.check_and_set_n(value, cpu);
    opc.check_and_set_z(value, cpu);
    cpu.cycle += 5;
}

pub fn dec_d6(opc: &mut Opcode, cpu : &mut MOS6510) {
    let AddrReturn { mut operand, address, high, low } = opc.zero_page_indexed(cpu.X, cpu);
    opc.current_operation.push_str(format!("DEC ${:02X}, X", low).as_str());
    let value = operand.wrapping_sub(1);
    cpu.mmu.write(value, address);
    opc.check_and_set_n(value, cpu);
    opc.check_and_set_z(value, cpu);
    cpu.cycle += 6;
}

pub fn inc_ee(opc: &mut Opcode, cpu : &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.absolute(cpu);
    opc.current_operation.push_str(format!("INC ${:02X}{:02X}", high.unwrap(), low).as_str());
    let value = operand.wrapping_add(1);
    cpu.mmu.write(value, address);
    opc.check_and_set_n(value, cpu);
    opc.check_and_set_z(value, cpu);
    cpu.cycle += 6;
}

pub fn inc_fe(opc: &mut Opcode, cpu : &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.absolute_indexed(cpu.X as u16, cpu);
    opc.current_operation.push_str(format!("INC ${:02X}{:02X}, X", high.unwrap(), low).as_str());
    let value = operand.wrapping_add(1);
    cpu.mmu.write(value, address);
    opc.check_and_set_n(value, cpu);
    opc.check_and_set_z(value, cpu);
    cpu.cycle += 7;
}

pub fn inc_e6(opc: &mut Opcode, cpu : &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.zero_page(cpu);
    opc.current_operation.push_str(format!("INC ${:02X}", low).as_str());
    let value = operand.wrapping_add(1);
    cpu.mmu.write(value, address);
    opc.check_and_set_n(value, cpu);
    opc.check_and_set_z(value, cpu);
    cpu.cycle += 5;
}

pub fn inc_f6(opc: &mut Opcode, cpu : &mut MOS6510) {
    let AddrReturn { mut operand, address, high, low } = opc.zero_page_indexed(cpu.X, cpu);
    opc.current_operation.push_str(format!("INC ${:02X}, X", low).as_str());
    let value = operand.wrapping_add(1);
    cpu.mmu.write(value, address);
    opc.check_and_set_n(value, cpu);
    opc.check_and_set_z(value, cpu);
    cpu.cycle += 6;
}

pub fn iny_c8(opc: &mut Opcode, cpu : &mut MOS6510) {
    opc.current_operation.push_str("INY");
    cpu.Y = cpu.Y.wrapping_add(1);
    opc.check_and_set_n(cpu.Y, cpu);
    opc.check_and_set_z(cpu.Y, cpu);
    cpu.cycle += 2;
}

pub fn dex_ca(opc: &mut Opcode, cpu : &mut MOS6510) {
    opc.current_operation.push_str("DEX");
    cpu.X -= 1;
    opc.check_and_set_n(cpu.X, cpu);
    opc.check_and_set_z(cpu.X, cpu);
    cpu.cycle += 2;
}

pub fn dey_88(opc: &mut Opcode, cpu : &mut MOS6510) {
    opc.current_operation.push_str("DEY");
    cpu.Y = cpu.Y.wrapping_sub(1);
    opc.check_and_set_n(cpu.Y, cpu);
    opc.check_and_set_z(cpu.Y, cpu);
    cpu.cycle += 2;
}

pub fn inx_e8(opc: &mut Opcode, cpu : &mut MOS6510) {
    opc.current_operation.push_str("INX");
    cpu.X = cpu.X.wrapping_add(1);
    opc.check_and_set_n(cpu.X, cpu);
    opc.check_and_set_z(cpu.X, cpu);
    cpu.cycle += 2;
}
