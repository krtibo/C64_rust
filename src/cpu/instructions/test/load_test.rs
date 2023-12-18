// use super::super::super::MOS6510;
use crate::MOS6510;
use super::super::Opcode;
use super::super::*;

fn setup() -> TestSetup {
    let mut cpu: MOS6510 = MOS6510::new(); 
    let mut opc: Opcode = Opcode::new();
    TestSetup { cpu, opc }
}

struct TestSetup {
    pub cpu: MOS6510,
    pub opc: Opcode,
}

#[test]
fn test_lda_a9() {
    let TestSetup { mut cpu, mut opc } = setup();
    cpu.mmu.write(0xFF, 0xA000);
    lda_a9(&mut opc, &mut cpu);

    assert_eq!(cpu.A, 0xFF);
}

#[test]
fn test_lda_bd() {
    let TestSetup { mut cpu, mut opc } = setup();
    cpu.X = 1;
    cpu.mmu.write(0xFF, 0xA001);
    cpu.mmu.write(0xFF, 0xFF01);
    lda_bd(&mut opc, &mut cpu);

    assert_eq!(cpu.A, 0xFF);
}
