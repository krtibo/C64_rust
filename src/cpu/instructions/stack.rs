use super::Opcode;
use super::AddrReturn;
use super::super::MOS6510;

pub fn pha_48(opc: &mut Opcode, cpu: &mut MOS6510) {
    opc.current_operation.push_str("PHA");
    cpu.push_on_stack(cpu.A);
    cpu.cycle += 3;
}

pub fn php_08(opc: &mut Opcode, cpu: &mut MOS6510) {
    opc.current_operation.push_str("PHP");
    cpu.push_on_stack(cpu.P);
    cpu.cycle += 3;
}

pub fn pla_68(opc: &mut Opcode, cpu: &mut MOS6510) {
    opc.current_operation.push_str("PLA");
    cpu.A = cpu.pull_from_stack();
    opc.check_and_set_n(cpu.A, cpu);
    opc.check_and_set_z(cpu.A, cpu);
    cpu.cycle += 4;
}

pub fn plp_28(opc: &mut Opcode, cpu: &mut MOS6510) {
    opc.current_operation.push_str("PLP");
    cpu.P = cpu.pull_from_stack();
    cpu.cycle += 4;
}
