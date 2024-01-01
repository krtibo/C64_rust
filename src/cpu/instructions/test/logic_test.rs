use super::super::logic::*;
use super::{ setup, TestSetup };

#[test]
fn test_and() {
    let TestSetup { mut cpu, mut opc } = setup();
    cpu.A = 0b0000_1111;
    let operand: u8 = 0b1111_0000;
    operation(&mut opc, &mut cpu, operand, Logic::And);

    assert_eq!(cpu.A, 0b0000_0000);
}

#[test]
fn test_or() {
    let TestSetup { mut cpu, mut opc } = setup();
    cpu.A = 0b0000_1111;
    let operand: u8 = 0b1111_0000;
    operation(&mut opc, &mut cpu, operand, Logic::Or);

    assert_eq!(cpu.A, 0b1111_1111);
}

#[test]
fn test_eor() {
    let TestSetup { mut cpu, mut opc } = setup();
    cpu.A = 0b0001_1111;
    let operand: u8 = 0b1111_0000;
    operation(&mut opc, &mut cpu, operand, Logic::Eor);

    assert_eq!(cpu.A, 0b1110_1111);
}
