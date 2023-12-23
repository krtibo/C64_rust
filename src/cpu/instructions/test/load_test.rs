use super::super::load::*;
use super::{ setup, TestSetup };

#[test]
fn test_lda_a9() {
    let TestSetup { mut cpu, mut opc } = setup();
    cpu.mmu.write(0xFF, 0x0000);
    lda_a9(&mut opc, &mut cpu);

    assert_eq!(cpu.A, 0xFF);
}

#[test]
fn test_lda_bd() {
    let TestSetup { mut cpu, mut opc } = setup();
    cpu.X = 1;
    cpu.mmu.write(0xFF, 0x0001);
    cpu.mmu.write(0xFF, 0xFF01);
    lda_bd(&mut opc, &mut cpu);

    assert_eq!(cpu.A, 0xFF);
}
