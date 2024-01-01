use crate::cpu::Flags;
use super::super::shift::*;
use super::{ setup, TestSetup };

#[test]
fn test_sl() {
    let TestSetup { mut cpu, mut opc } = setup();
    let operand: u8 = 0b0000_0010;
    let address: u16 = 0xFFF0;
    sl(&mut opc, &mut cpu, operand, address);

    assert_eq!(cpu.mmu.read(address), 0b0000_0100);
}

#[test]
fn test_sr() {
    let TestSetup { mut cpu, mut opc } = setup();
    let operand: u8 = 0b0000_0010;
    let address: u16 = 0xFFF0;
    sr(&mut opc, &mut cpu, operand, address);

    assert_eq!(cpu.mmu.read(address), 0b0000_0001);
}

#[test]
fn test_rol() {
    let TestSetup { mut cpu, mut opc } = setup();
    let operand: u8 = 0b0000_0010;
    let address: u16 = 0xFFF0;
    cpu.set_flag(Flags::C, 1);
    rol(&mut opc, &mut cpu, operand, address);

    assert_eq!(cpu.mmu.read(address), 0b0000_0101);
}

#[test]
fn test_rol_zero_carry() {
    let TestSetup { mut cpu, mut opc } = setup();
    let operand: u8 = 0b0000_0010;
    let address: u16 = 0xFFF0;
    cpu.set_flag(Flags::C, 0);
    rol(&mut opc, &mut cpu, operand, address);

    assert_eq!(cpu.mmu.read(address), 0b0000_0100);
}

#[test]
fn test_ror() {
    let TestSetup { mut cpu, mut opc } = setup();
    let operand: u8 = 0b0000_0010;
    let address: u16 = 0xFFF0;
    cpu.set_flag(Flags::C, 1);
    ror(&mut opc, &mut cpu, operand, address);

    assert_eq!(cpu.mmu.read(address), 0b1000_0001);
}

#[test]
fn test_ror_zero_carry() {
    let TestSetup { mut cpu, mut opc } = setup();
    let operand: u8 = 0b0000_0010;
    let address: u16 = 0xFFF0;
    cpu.set_flag(Flags::C, 0);
    ror(&mut opc, &mut cpu, operand, address);

    assert_eq!(cpu.mmu.read(address), 0b0000_0001);
}
