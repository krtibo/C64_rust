use super::super::inc::*;
use super::{ setup, TestSetup };

#[test]
fn test_dec() {
    let TestSetup { mut cpu, mut opc } = setup();
    let operand: u8 = 0x01;
    let address: u16 = 0xFFF0;
    cpu.mmu.write(operand, address);
    dec(&mut opc, &mut cpu, operand, address);

    assert_eq!(cpu.mmu.read(address), 0x00);
}

#[test]
fn test_inc() {
    let TestSetup { mut cpu, mut opc } = setup();
    let operand: u8 = 0x00;
    let address: u16 = 0xFFF0;
    cpu.mmu.write(operand, address);
    inc(&mut opc, &mut cpu, operand, address);

    assert_eq!(cpu.mmu.read(address), 0x01);
}
