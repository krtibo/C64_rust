use super::Opcode;
use super::AddrReturn;
use super::super::MOS6510;
use super::super::Flags;

pub fn and_29(opc: &mut Opcode, cpu : &mut MOS6510) {
    let operand: u8 = opc.fetch(cpu);
    opc.current_operation.push_str(format!("AND #${:02X}", operand).as_str());
    cpu.A = cpu.A & operand;
    opc.check_and_set_z(cpu.A, cpu);
    opc.check_and_set_n(cpu.A, cpu);
    cpu.cycle += 2;
}

pub fn and_2d(opc: &mut Opcode, cpu : &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.absolute(cpu);
    opc.current_operation.push_str(format!("AND ${:02X}{:02X}", high.unwrap(), low).as_str());
    cpu.A = cpu.A & operand;
    opc.check_and_set_z(cpu.A, cpu);
    opc.check_and_set_n(cpu.A, cpu);
    cpu.cycle += 4;
}

pub fn and_3d(opc: &mut Opcode, cpu : &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.absolute_indexed(cpu.X as u16, cpu);
    opc.current_operation.push_str(format!("AND ${:02X}{:02X}, X", high.unwrap(), low).as_str());
    cpu.A = cpu.A & operand;
    opc.check_and_set_z(cpu.A, cpu);
    opc.check_and_set_n(cpu.A, cpu);
    cpu.cycle += 4;
}

pub fn and_39(opc: &mut Opcode, cpu : &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.absolute_indexed(cpu.Y as u16, cpu);
    opc.current_operation.push_str(format!("AND ${:02X}{:02X}, Y", high.unwrap(), low).as_str());
    cpu.A = cpu.A & operand;
    opc.check_and_set_z(cpu.A, cpu);
    opc.check_and_set_n(cpu.A, cpu);
    cpu.cycle += 4;
}

pub fn and_25(opc: &mut Opcode, cpu : &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.zero_page(cpu);
    opc.current_operation.push_str(format!("AND ${:02X}", low).as_str());
    cpu.A = cpu.A & operand;
    opc.check_and_set_z(cpu.A, cpu);
    opc.check_and_set_n(cpu.A, cpu);
    cpu.cycle += 3;
}

pub fn and_35(opc: &mut Opcode, cpu : &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.zero_page_indexed(cpu.X, cpu);
    opc.current_operation.push_str(format!("AND ${:02X}, X", low).as_str());
    cpu.A = cpu.A & operand;
    opc.check_and_set_z(cpu.A, cpu);
    opc.check_and_set_n(cpu.A, cpu);
    cpu.cycle += 4;
}

pub fn and_21(opc: &mut Opcode, cpu : &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.zero_page_indexed_indirect(cpu.X, cpu);
    opc.current_operation.push_str(format!("AND (${:02X}, X)", operand).as_str());
    cpu.A = cpu.A & operand;    
    opc.check_and_set_n(cpu.A, cpu);
    opc.check_and_set_z(cpu.A, cpu);
    cpu.cycle += 6;
}

pub fn and_31(opc: &mut Opcode, cpu : &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.zero_page_indirect_indexed(cpu.X, cpu);
    opc.current_operation.push_str(format!("AND (${:02X}), Y", low).as_str());
    cpu.A = cpu.A & operand;
    opc.check_and_set_n(cpu.A, cpu);
    opc.check_and_set_z(cpu.A, cpu);
    cpu.cycle += 5;
}

pub fn ora_09(opc: &mut Opcode, cpu : &mut MOS6510) {
    let operand: u8 = opc.fetch(cpu);
    opc.current_operation.push_str(format!("ORA #${:02X}", operand).as_str());
    cpu.A = cpu.A | operand;
    opc.check_and_set_z(cpu.A, cpu);
    opc.check_and_set_n(cpu.A, cpu);
    cpu.cycle += 2;
}

pub fn ora_0d(opc: &mut Opcode, cpu : &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.absolute(cpu);
    opc.current_operation.push_str(format!("ORA ${:02X}{:02X}", high.unwrap(), low).as_str());
    cpu.A = cpu.A | operand;
    opc.check_and_set_z(cpu.A, cpu);
    opc.check_and_set_n(cpu.A, cpu);
    cpu.cycle += 4;
}

pub fn ora_1d(opc: &mut Opcode, cpu : &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.absolute_indexed(cpu.X as u16, cpu);
    opc.current_operation.push_str(format!("ORA ${:02X}{:02X}, X", high.unwrap(), low).as_str());
    cpu.A = cpu.A | operand;
    opc.check_and_set_z(cpu.A, cpu);
    opc.check_and_set_n(cpu.A, cpu);
    cpu.cycle += 4;
}

pub fn ora_19(opc: &mut Opcode, cpu : &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.absolute_indexed(cpu.Y as u16, cpu);
    opc.current_operation.push_str(format!("ORA ${:02X}{:02X}, Y", high.unwrap(), low).as_str());
    cpu.A = cpu.A | operand;
    opc.check_and_set_z(cpu.A, cpu);
    opc.check_and_set_n(cpu.A, cpu);
    cpu.cycle += 4;
}

pub fn ora_05(opc: &mut Opcode, cpu : &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.zero_page(cpu);
    opc.current_operation.push_str(format!("ORA ${:02X}", low).as_str());
    cpu.A = cpu.A | operand;
    opc.check_and_set_z(cpu.A, cpu);
    opc.check_and_set_n(cpu.A, cpu);
    cpu.cycle += 3;
}

pub fn ora_15(opc: &mut Opcode, cpu : &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.zero_page_indexed(cpu.X, cpu);
    opc.current_operation.push_str(format!("ORA ${:02X}, X", low).as_str());
    cpu.A = cpu.A | operand;
    opc.check_and_set_z(cpu.A, cpu);
    opc.check_and_set_n(cpu.A, cpu);
    cpu.cycle += 4;
}

pub fn ora_01(opc: &mut Opcode, cpu : &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.zero_page_indexed_indirect(cpu.X, cpu);
    opc.current_operation.push_str(format!("ORA (${:02X}, X)", low).as_str());
    cpu.A = cpu.A | operand;
    opc.check_and_set_n(cpu.A, cpu);
    opc.check_and_set_z(cpu.A, cpu);
    cpu.cycle += 6;
}

pub fn ora_11(opc: &mut Opcode, cpu : &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.zero_page_indirect_indexed(cpu.X, cpu);
    opc.current_operation.push_str(format!("ORA (${:02X}), Y", low).as_str());
    cpu.A = cpu.A | operand;
    opc.check_and_set_n(cpu.A, cpu);
    opc.check_and_set_z(cpu.A, cpu);
    cpu.cycle += 5;
}

pub fn eor_49(opc: &mut Opcode, cpu : &mut MOS6510) {
    let operand: u8 = opc.fetch(cpu);
    opc.current_operation.push_str(format!("EOR #${:02X}", operand).as_str());
    cpu.A = cpu.A ^ operand;
    opc.check_and_set_z(cpu.A, cpu);
    opc.check_and_set_n(cpu.A, cpu);
    cpu.cycle += 2;
}

pub fn eor_4d(opc: &mut Opcode, cpu : &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.absolute(cpu);
    opc.current_operation.push_str(format!("EOR ${:02X}{:02X}", high.unwrap(), low).as_str());
    cpu.A = cpu.A ^ operand;
    opc.check_and_set_z(cpu.A, cpu);
    opc.check_and_set_n(cpu.A, cpu);
    cpu.cycle += 4;
}

pub fn eor_5d(opc: &mut Opcode, cpu : &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.absolute_indexed(cpu.X as u16, cpu);
    opc.current_operation.push_str(format!("EOR ${:02X}{:02X}, X", high.unwrap(), low).as_str());
    cpu.A = cpu.A ^ operand;
    opc.check_and_set_z(cpu.A, cpu);
    opc.check_and_set_n(cpu.A, cpu);
    cpu.cycle += 4;
}

pub fn eor_59(opc: &mut Opcode, cpu : &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.absolute_indexed(cpu.Y as u16, cpu);
    opc.current_operation.push_str(format!("EOR ${:02X}{:02X}, Y", high.unwrap(), low).as_str());
    cpu.A = cpu.A ^ operand;
    opc.check_and_set_z(cpu.A, cpu);
    opc.check_and_set_n(cpu.A, cpu);
    cpu.cycle += 4;
}

pub fn eor_45(opc: &mut Opcode, cpu : &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.zero_page(cpu);
    opc.current_operation.push_str(format!("EOR ${:02X}", low).as_str());
    cpu.A = cpu.A ^ operand;
    opc.check_and_set_z(cpu.A, cpu);
    opc.check_and_set_n(cpu.A, cpu);
    cpu.cycle += 3;
}

pub fn eor_55(opc: &mut Opcode, cpu : &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.zero_page_indexed(cpu.X, cpu);
    opc.current_operation.push_str(format!("EOR ${:02X}, X", low).as_str());
    cpu.A = cpu.A ^ operand;
    opc.check_and_set_z(cpu.A, cpu);
    opc.check_and_set_n(cpu.A, cpu);
    cpu.cycle += 4;
}

pub fn eor_41(opc: &mut Opcode, cpu : &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.zero_page_indexed_indirect(cpu.X, cpu);
    opc.current_operation.push_str(format!("EOR (${:02X}, X)", low).as_str());
    cpu.A = cpu.A ^ operand;
    opc.check_and_set_n(cpu.A, cpu);
    opc.check_and_set_z(cpu.A, cpu);
    cpu.cycle += 6;
}

pub fn eor_51(opc: &mut Opcode, cpu : &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.zero_page_indirect_indexed(cpu.X, cpu);
    opc.current_operation.push_str(format!("EOR (${:02X}), Y", low).as_str());
    cpu.A = cpu.A ^ operand;
    opc.check_and_set_n(cpu.A, cpu);
    opc.check_and_set_z(cpu.A, cpu);
    cpu.cycle += 5;
}

pub fn bit_2c(opc: &mut Opcode, cpu : &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.absolute(cpu);
    opc.current_operation.push_str(format!("BIT ${:02X}{:02X}", high.unwrap(), low).as_str());
    let result = cpu.A & operand;
    if opc.get_bit(operand, 6) == 1 { cpu.set_flag(Flags::V, 1) } else { cpu.set_flag(Flags::V, 0) }
    opc.check_and_set_n(operand, cpu);
    opc.check_and_set_z(result, cpu);
    cpu.cycle += 4;
}

pub fn bit_24(opc: &mut Opcode, cpu : &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.zero_page(cpu);
    opc.current_operation.push_str(format!("BIT ${:02X}", low).as_str());
    let result = cpu.A & operand;
    if opc.get_bit(operand, 6) == 1 { cpu.set_flag(Flags::V, 1) } else { cpu.set_flag(Flags::V, 0) }
    opc.check_and_set_n(operand, cpu);
    opc.check_and_set_z(result, cpu);
    cpu.cycle += 3;
}
