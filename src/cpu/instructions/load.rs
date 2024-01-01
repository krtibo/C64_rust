use super::Opcode;
use super::AddrReturn;
use super::super::MOS6510;

pub enum Register {
    A,
    X,
    Y
}

pub fn ld(opc: &mut Opcode, cpu: &mut MOS6510, operand: u8, register: Register) {
    match register {
        Register::A => {
            cpu.A = operand;
            opc.check_and_set_n(cpu.A, cpu);
            opc.check_and_set_z(cpu.A, cpu);
        },
        Register::X => {
            cpu.X = operand;
            opc.check_and_set_n(cpu.X, cpu);
            opc.check_and_set_z(cpu.X, cpu);
        },
        Register::Y => {
            cpu.Y = operand;
            opc.check_and_set_n(cpu.Y, cpu);
            opc.check_and_set_z(cpu.Y, cpu);
        },
    }
}

pub fn lda_a9(opc: &mut Opcode, cpu: &mut MOS6510) {
    let operand: u8 = opc.fetch(cpu);
    opc.current_operation.push_str(format!("LDA #${:02X}", operand).as_str());
    ld(opc, cpu, operand, Register::A);
    cpu.cycle += 2;
}

pub fn lda_ad(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.absolute(cpu);
    opc.current_operation.push_str(format!("LDA ${:02X}{:02X}", high.unwrap(), low).as_str());
    ld(opc, cpu, operand, Register::A);
    cpu.cycle += 4;
}

pub fn lda_bd(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.absolute_indexed(cpu.X as u16, cpu);
    opc.current_operation.push_str(format!("LDA ${:02X}{:02X}, X", high.unwrap(), low).as_str());
    ld(opc, cpu, operand, Register::A);
    cpu.cycle += 4;
    // TODO: cycle is 4+1 if page is crossed
}

pub fn lda_b9(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.absolute_indexed(cpu.Y as u16, cpu);
    opc.current_operation.push_str(format!("LDA ${:02X}{:02X}, Y", high.unwrap(), low).as_str());
    ld(opc, cpu, operand, Register::A);
    cpu.cycle += 4;
    // TODO: cycle is 4+1 if page is crossed
}

pub fn lda_a5(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.zero_page(cpu);
    opc.current_operation.push_str(format!("LDA ${:02X}", low).as_str());
    ld(opc, cpu, operand, Register::A);
    cpu.cycle += 3;
}

pub fn lda_b5(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.zero_page_indexed(cpu.X, cpu);
    opc.current_operation.push_str(format!("LDA ${:02X}, X", low).as_str());
    ld(opc, cpu, operand, Register::A);
    cpu.cycle += 4;
}

pub fn lda_a1(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.zero_page_indexed_indirect(cpu.X, cpu);
    opc.current_operation.push_str(format!("LDA (${:02X}, X)", low).as_str());
    ld(opc, cpu, operand, Register::A);
    cpu.cycle += 6;
}

pub fn lda_b1(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.zero_page_indirect_indexed(cpu.X, cpu);
    opc.current_operation.push_str(format!("LDA (${:02X}), Y", low).as_str());
    ld(opc, cpu, operand, Register::A);
    cpu.cycle += 5;
    // TODO: cycle is 5+1 if page is crossed
}

pub fn ldx_a2(opc: &mut Opcode, cpu: &mut MOS6510) {
    let operand: u8 = opc.fetch(cpu);
    opc.current_operation.push_str(format!("LDX #${:02X}", operand).as_str());
    ld(opc, cpu, operand, Register::X);
    cpu.cycle += 2;
}

pub fn ldx_ae(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.absolute(cpu);
    opc.current_operation.push_str(format!("LDX ${:02X}{:02X}", high.unwrap(), low).as_str());
    ld(opc, cpu, operand, Register::X);
    cpu.cycle += 4;
}

pub fn ldx_be(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.absolute_indexed(cpu.Y as u16, cpu);
    opc.current_operation.push_str(format!("LDX ${:02X}{:02X}, Y", high.unwrap(), low).as_str());
    ld(opc, cpu, operand, Register::X);
    cpu.cycle += 4;
    // TODO: cycle is 4+1 if page is crossed
}

pub fn ldx_a6(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.zero_page(cpu);
    opc.current_operation.push_str(format!("LDX ${:02X}", low).as_str());
    ld(opc, cpu, operand, Register::X);
    cpu.cycle += 3;
}

pub fn ldx_b6(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.zero_page_indexed(cpu.Y, cpu);
    opc.current_operation.push_str(format!("LDX ${:02X}, Y", low).as_str());
    ld(opc, cpu, operand, Register::X);
    cpu.cycle += 4;
}

pub fn ldy_a0(opc: &mut Opcode, cpu: &mut MOS6510) {
    let operand: u8 = opc.fetch(cpu);
    opc.current_operation.push_str(format!("LDY #${:02X}", operand).as_str());
    ld(opc, cpu, operand, Register::Y);
    cpu.cycle += 2;
}

pub fn ldy_ac(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.absolute(cpu);
    opc.current_operation.push_str(format!("LDY ${:02X}{:02X}", high.unwrap(), low).as_str());
    ld(opc, cpu, operand, Register::Y);
    cpu.cycle += 4;
}

pub fn ldy_bc(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.absolute_indexed(cpu.X as u16, cpu);
    opc.current_operation.push_str(format!("LDY ${:02X}{:02X}, X", high.unwrap(), low).as_str());
    ld(opc, cpu, operand, Register::Y);
    cpu.cycle += 4;
    // TODO: cycle is 4+1 if page is crossed
}

pub fn ldy_a4(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.zero_page(cpu);
    opc.current_operation.push_str(format!("LDY ${:02X}", low).as_str());
    ld(opc, cpu, operand, Register::Y);
    cpu.cycle += 3;
}

pub fn ldy_b4(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.zero_page_indexed(cpu.X, cpu);
    opc.current_operation.push_str(format!("LDY ${:02X}, X", low).as_str());
    ld(opc, cpu, operand, Register::Y);
    cpu.cycle += 4;
}

pub fn sta_8d(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.absolute(cpu);
    opc.current_operation.push_str(format!("STA ${:02X}{:02X}", high.unwrap(), low).as_str());
    cpu.mmu.write(cpu.A, address);
    cpu.cycle += 4;
}

pub fn sta_9d(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.absolute_indexed(cpu.X as u16, cpu);
    opc.current_operation.push_str(format!("STA ${:02X}{:02X}, X", high.unwrap(), low).as_str());
    cpu.mmu.write(cpu.A, address);
    cpu.cycle += 5;
}

pub fn sta_99(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.absolute_indexed(cpu.Y as u16, cpu);
    opc.current_operation.push_str(format!("STA ${:02X}{:02X}, Y", high.unwrap(), low).as_str());
    cpu.mmu.write(cpu.A, address);
    cpu.cycle += 5;
}

pub fn sta_85(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.zero_page(cpu);
    opc.current_operation.push_str(format!("STA ${:02X}", low).as_str());
    cpu.mmu.write(cpu.A, address);
    cpu.cycle += 3;
}

pub fn sta_95(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.zero_page_indexed(cpu.X, cpu);
    opc.current_operation.push_str(format!("STA ${:02X}, X", low).as_str());
    cpu.mmu.write(cpu.A, address);
    cpu.cycle += 4;
}

pub fn sta_81(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.zero_page_indexed_indirect(cpu.X, cpu);
    opc.current_operation.push_str(format!("STA (${:02X}, X)", low).as_str());
    cpu.mmu.write(cpu.A, address);
    cpu.cycle += 6;
}

pub fn sta_91(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.zero_page_indirect_indexed(cpu.X, cpu);
    opc.current_operation.push_str(format!("STA (${:02X}), Y", low).as_str());
    cpu.mmu.write(cpu.A, address);
    cpu.cycle += 6;
}

pub fn stx_8e(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.absolute(cpu);
    opc.current_operation.push_str(format!("STX ${:02X}{:02X}", high.unwrap(), low).as_str());
    cpu.mmu.write(cpu.X, address);
    cpu.cycle += 4;
}

pub fn stx_86(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.zero_page(cpu);
    opc.current_operation.push_str(format!("STX ${:02X}", low).as_str());
    cpu.mmu.write(cpu.X, address);
    cpu.cycle += 3;
}

pub fn stx_96(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.zero_page_indexed(cpu.Y, cpu);
    opc.current_operation.push_str(format!("STX ${:02X}, Y", low).as_str());
    cpu.mmu.write(cpu.X, address);
    cpu.cycle += 4;
}

pub fn sty_8c(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.absolute(cpu);
    opc.current_operation.push_str(format!("STY ${:02X}{:02X}", high.unwrap(), low).as_str());
    cpu.mmu.write(cpu.Y, address);
    cpu.cycle += 4;
}

pub fn sty_84(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.zero_page(cpu);
    opc.current_operation.push_str(format!("STY ${:02X}", low).as_str());
    cpu.mmu.write(cpu.Y, address);
    cpu.cycle += 3;
}

pub fn sty_94(opc: &mut Opcode, cpu: &mut MOS6510) {
    let AddrReturn { operand, address, high, low } = opc.zero_page_indexed(cpu.X, cpu);
    opc.current_operation.push_str(format!("STY ${:02X}, X", low).as_str());
    cpu.mmu.write(cpu.Y, address);
    cpu.cycle += 4;
}
