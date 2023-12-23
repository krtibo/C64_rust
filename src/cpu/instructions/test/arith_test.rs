use crate::cpu::Flags;
use super::super::arith::*;
use super::{ setup, TestSetup };

#[test]
fn adc_non_decimal() {
    let TestSetup { mut cpu, mut opc } = setup();
    cpu.set_flag(Flags::D, 0);
    cpu.set_flag(Flags::C, 1);
    cpu.A = 10;
    let operand: u8 = 5;
    adc(&mut opc, &mut cpu, operand);

    assert_eq!(cpu.A, 16);
}

#[test]
fn adc_non_decimal_carry() {
    let TestSetup { mut cpu, mut opc } = setup();
    cpu.set_flag(Flags::D, 0);
    cpu.set_flag(Flags::C, 1);
    cpu.A = 254;
    let operand: u8 = 1;
    adc(&mut opc, &mut cpu, operand);

    assert_eq!(cpu.A, 0);
    assert_eq!(cpu.get_flag(Flags::C), true);
}

#[test]
fn adc_non_decimal_overflow() {
    let TestSetup { mut cpu, mut opc } = setup();
    cpu.set_flag(Flags::D, 0);
    cpu.set_flag(Flags::C, 1);
    cpu.A = 127;
    let operand: u8 = 1;
    adc(&mut opc, &mut cpu, operand);

    assert_eq!(cpu.A, 129);
    assert_eq!(cpu.get_flag(Flags::V), true);
}


#[test]
fn adc_decimal() {
    let TestSetup { mut cpu, mut opc } = setup();
    cpu.set_flag(Flags::D, 1);
    cpu.set_flag(Flags::C, 1);
    cpu.A = 0x25;
    let operand: u8 = 5;
    adc(&mut opc, &mut cpu, operand);

    assert_eq!(cpu.A, 31);
}

#[test]
fn cmp_is_null() {
    let TestSetup { mut cpu, mut opc } = setup();
    cmp(&mut opc, &mut cpu, 5, 5);

    assert_eq!(cpu.get_flag(Flags::Z), true);
}

#[test]
fn cmp_is_less() {
    let TestSetup { mut cpu, mut opc } = setup();
    cmp(&mut opc, &mut cpu, 3, 5);

    assert_eq!(cpu.get_flag(Flags::C), true);
}

#[test]
fn cmp_is_negative() {
    let TestSetup { mut cpu, mut opc } = setup();
    cmp(&mut opc, &mut cpu, 8, 5);

    assert_eq!(cpu.get_flag(Flags::N), true);
}

#[test]
fn sbc_non_decimal() {
}
