use super::Opcode;
use super::AddrReturn;
use super::super::MOS6510;
use super::super::Flags;

pub fn brk_00(opc: &mut Opcode, cpu: &mut MOS6510) {
    opc.current_operation.push_str("BRK");
    let pc: [u8; 2] = opc.u16_to_u8s(cpu.PC.wrapping_add(2));
    cpu.push_on_stack(pc[0]);
    cpu.push_on_stack(pc[1]);
    cpu.push_on_stack(cpu.P);
    cpu.PC = opc.u8s_to_u16(cpu.mmu.read(0xFFFF), cpu.mmu.read(0xFFFE));
    cpu.set_flag(Flags::I, 1);
    cpu.set_flag(Flags::B, 1);
    cpu.cycle += 7;
    opc.omit_fetch = true;
}

pub fn nop_ea(opc: &mut Opcode, cpu: &mut MOS6510) {
    opc.current_operation.push_str("NOP");
    cpu.cycle += 2;
}

pub fn jmp_4c(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.absolute(cpu);
    opc.current_operation.push_str(format!("JMP ${:02X}{:02X}", high.unwrap(), low).as_str());
    cpu.PC = address;
    cpu.cycle += 3;
    opc.omit_fetch = true;
}

pub fn jmp_6c(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.absolute_indirect(cpu);
    opc.current_operation.push_str(format!("JMP (${:02X}{:02X})", high.unwrap(), low).as_str());
    cpu.PC = address;
    cpu.cycle += 5;
    opc.omit_fetch = true;
}

pub fn jsr_20(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.absolute(cpu);
    opc.current_operation.push_str(format!("JSR ${:02X}{:02X}", high.unwrap(), low).as_str());
    let pc_bytes = opc.u16_to_u8s(cpu.PC);
    cpu.push_on_stack(pc_bytes[0]);
    cpu.push_on_stack(pc_bytes[1]);
    cpu.PC = address;
    cpu.cycle += 6;
    opc.omit_fetch = true;
}

pub fn rti_40(opc: &mut Opcode, cpu: &mut MOS6510) {
    let status: u8 = cpu.pull_from_stack();
    let low: u8 = cpu.pull_from_stack();
    let high: u8 = cpu.pull_from_stack();
    opc.current_operation.push_str(format!("RTI flags: {:08b} PC: {:02X}{:02X}", status, high, low).as_str());
    cpu.PC = opc.u8s_to_u16(high, low);
    cpu.P = status;
    cpu.cycle += 6;
    opc.omit_fetch = true;
}

pub fn rts_60(opc: &mut Opcode, cpu: &mut MOS6510) {
    opc.current_operation.push_str("RTS");
    let pc_low = cpu.pull_from_stack();
    let pc_high = cpu.pull_from_stack();
    cpu.PC = opc.u8s_to_u16(pc_high, pc_low) + 1;
    cpu.cycle += 4;
    opc.omit_fetch = true;
}
