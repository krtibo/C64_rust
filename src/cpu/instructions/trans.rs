use super::Opcode;
use super::AddrReturn;
use super::super::MOS6510;

pub fn tax_aa(opc: &mut Opcode, cpu: &mut MOS6510) {
    opc.current_operation.push_str("TAX");
    cpu.X = cpu.A;
    opc.check_and_set_n(cpu.X, cpu);
    opc.check_and_set_z(cpu.X, cpu);
    cpu.cycle += 2;
}

pub fn tay_a8(opc: &mut Opcode, cpu: &mut MOS6510) {
    opc.current_operation.push_str("TAY");
    cpu.Y = cpu.A;
    opc.check_and_set_n(cpu.Y, cpu);
    opc.check_and_set_z(cpu.Y, cpu);
    cpu.cycle += 2;
}

pub fn tsx_ba(opc: &mut Opcode, cpu: &mut MOS6510) {
    opc.current_operation.push_str("TSX");
    cpu.X = cpu.S;
    opc.check_and_set_n(cpu.X, cpu);
    opc.check_and_set_z(cpu.X, cpu);
    cpu.cycle += 2;
}

pub fn txa_8a(opc: &mut Opcode, cpu: &mut MOS6510) {
    opc.current_operation.push_str("TXA");
    cpu.A = cpu.X;
    opc.check_and_set_n(cpu.A, cpu);
    opc.check_and_set_z(cpu.A, cpu);
    cpu.cycle += 2;
}

pub fn txs_9a(opc: &mut Opcode, cpu: &mut MOS6510) {
    opc.current_operation.push_str("TXS");
    cpu.S = cpu.X;
    opc.check_and_set_n(cpu.S, cpu);
    opc.check_and_set_z(cpu.S, cpu);
    cpu.cycle += 2;
}

pub fn tya_98(opc: &mut Opcode, cpu: &mut MOS6510) {
    opc.current_operation.push_str("TYA");
    cpu.A = cpu.Y;
    opc.check_and_set_n(cpu.A, cpu);
    opc.check_and_set_z(cpu.A, cpu);
    cpu.cycle += 2;
}
