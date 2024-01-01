use super::Opcode;
use super::AddrReturn;
use super::super::MOS6510;
use super::super::Flags;

pub fn clc_18(opc: &mut Opcode, cpu: &mut MOS6510) {
    opc.current_operation.push_str("CLC");
    cpu.set_flag(Flags::C, 0);
    cpu.cycle += 2;
}

pub fn cld_d8(opc: &mut Opcode, cpu: &mut MOS6510) {
    opc.current_operation.push_str("CLD");
    cpu.set_flag(Flags::D, 0);
    cpu.cycle += 2;
}

pub fn cli_58(opc: &mut Opcode, cpu: &mut MOS6510) {
    opc.current_operation.push_str("CLI");
    cpu.set_flag(Flags::I, 0);
    cpu.cycle += 2;
}

pub fn clv_b8(opc: &mut Opcode, cpu: &mut MOS6510) {
    opc.current_operation.push_str("CLV");
    cpu.set_flag(Flags::V, 0);
    cpu.cycle += 2;
}

pub fn sec_38(opc: &mut Opcode, cpu: &mut MOS6510) {
    opc.current_operation.push_str("SEC");
    cpu.set_flag(Flags::C, 1);
    cpu.cycle += 2;
}

pub fn sed_f8(opc: &mut Opcode, cpu: &mut MOS6510) {
    opc.current_operation.push_str("SED");
    cpu.set_flag(Flags::D, 1);
    cpu.cycle += 2;
}

pub fn sei_78(opc: &mut Opcode, cpu: &mut MOS6510) {
    opc.current_operation.push_str("SEI");
    cpu.set_flag(Flags::I, 1);
    cpu.cycle += 2;
}
