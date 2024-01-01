use super::Opcode;
use super::AddrReturn;
use super::super::MOS6510;
use super::super::Flags;

pub fn bcc_90(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.relative(cpu);
    opc.current_operation.push_str(format!("BCC ${:02X}{:02X}", high.unwrap(), low).as_str());
    if !cpu.get_flag(Flags::C) {
        cpu.PC = address;
        cpu.cycle += 1;
    }
    cpu.cycle += 2;
}

pub fn bcs_b0(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.relative(cpu);
    opc.current_operation.push_str(format!("BCS ${:02X}{:02X}", high.unwrap(), low).as_str());
    if cpu.get_flag(Flags::C) {
        cpu.PC = address;
        cpu.cycle += 1;
    }
    cpu.cycle += 2;
}

pub fn beq_f0(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.relative(cpu);
    opc.current_operation.push_str(format!("BEQ ${:02X}{:02X}", high.unwrap(), low).as_str());
    if cpu.get_flag(Flags::Z) {
        cpu.PC = address;
        cpu.cycle += 1;
    }
    cpu.cycle += 2;
}

pub fn bmi_30(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.relative(cpu);
    opc.current_operation.push_str(format!("BMI ${:02X}{:02X}", high.unwrap(), low).as_str());
    if cpu.get_flag(Flags::N) {
        cpu.PC = address;
        cpu.cycle += 1;
    }
    cpu.cycle += 2;
}

pub fn bne_d0(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.relative(cpu);
    opc.current_operation.push_str(format!("BNE ${:02X}{:02X}", high.unwrap(), low).as_str());
    if !cpu.get_flag(Flags::Z) {
        cpu.PC = address;
        cpu.cycle += 1;
    }
    cpu.cycle += 2;
}

pub fn bpl_10(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.relative(cpu);
    opc.current_operation.push_str(format!("BPL ${:02X}{:02X}", high.unwrap(), low).as_str());
    if !cpu.get_flag(Flags::N) {
        cpu.PC = address;
        cpu.cycle += 1;
    }
    cpu.cycle += 2;
}

pub fn bvc_50(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.relative(cpu);
    opc.current_operation.push_str(format!("BVC ${:02X}{:02X}", high.unwrap(), low).as_str());
    if !cpu.get_flag(Flags::V) {
        cpu.PC = address;
        cpu.cycle += 1;
    }
    cpu.cycle += 2;
}
pub fn bvs_70(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.relative(cpu);
    opc.current_operation.push_str(format!("BVS ${:02X}{:02X}", high.unwrap(), low).as_str());
    if cpu.get_flag(Flags::V) {
        cpu.PC = address;
        cpu.cycle += 1;
    }
    cpu.cycle += 2;
}

