pub mod load_test;
pub mod arith_test;

use crate::MOS6510;
use super::super::Opcode;

pub fn setup() -> TestSetup {
    let mut cpu: MOS6510 = MOS6510::new(); 
    cpu.PC = 0x0000;
    let mut opc: Opcode = Opcode::new();
    TestSetup { cpu, opc }
}

pub struct TestSetup {
    pub cpu: MOS6510,
    pub opc: Opcode,
}
