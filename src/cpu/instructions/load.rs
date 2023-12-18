use super::Opcode;
use super::AddrReturn;
use super::super::MOS6510;

pub fn lda_a9(opc: &mut Opcode, cpu: &mut MOS6510) {
    let operand: u8 = opc.fetch(cpu);
    opc.current_operation.push_str(format!("LDA #${:02X}", operand).as_str());
    cpu.A = operand;
    opc.check_and_set_n(cpu.A, cpu);
    opc.check_and_set_z(cpu.A, cpu);
    cpu.cycle += 2;
}

pub fn lda_bd(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.absolute_indexed(cpu.X as u16, cpu);
    opc.current_operation.push_str(format!("LDA ${:02X}{:02X}, X", high.unwrap(), low).as_str());
    cpu.A = operand;
    opc.check_and_set_n(cpu.A, cpu);
    opc.check_and_set_z(cpu.A, cpu);
    cpu.cycle += 4;
    // TODO: cycle is 4+1 if page is crossed
}

pub fn tester(a: u8, b: u8) -> u8 {
    a+b
}
