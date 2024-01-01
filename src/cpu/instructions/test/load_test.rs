use super::super::load::*;
use super::{ setup, TestSetup };

#[test]
fn test_ld() {
    let TestSetup { mut cpu, mut opc } = setup();
    ld(&mut opc, &mut cpu, 0x05, Register::A);

    assert_eq!(cpu.A, 0x05);
}
