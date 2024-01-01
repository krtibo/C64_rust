use super::cpu::MOS6510;
use super::cpu::Flags;

#[test]
fn cpu_set_flag() {
    let mut cpu: MOS6510 = MOS6510::new();
    cpu.set_flag(Flags::N, 1);
    assert_eq!(cpu.get_flag(Flags::N), true);

    cpu.set_flag(Flags::V, 1);
    assert_eq!(cpu.get_flag(Flags::V), true);

    cpu.set_flag(Flags::B, 1);
    assert_eq!(cpu.get_flag(Flags::B), true);

    cpu.set_flag(Flags::D, 1);
    assert_eq!(cpu.get_flag(Flags::D), true);

    cpu.set_flag(Flags::I, 1);
    assert_eq!(cpu.get_flag(Flags::I), true);

    cpu.set_flag(Flags::Z, 1);
    assert_eq!(cpu.get_flag(Flags::Z), true);

    cpu.set_flag(Flags::C, 1);
    assert_eq!(cpu.get_flag(Flags::C), true);

    cpu.set_flag(Flags::N, 0);
    assert_eq!(cpu.get_flag(Flags::N), false);

    cpu.set_flag(Flags::V, 0);
    assert_eq!(cpu.get_flag(Flags::V), false);

    cpu.set_flag(Flags::B, 0);
    assert_eq!(cpu.get_flag(Flags::B), false);

    cpu.set_flag(Flags::D, 0);
    assert_eq!(cpu.get_flag(Flags::D), false);

    cpu.set_flag(Flags::I, 0);
    assert_eq!(cpu.get_flag(Flags::I), false);

    cpu.set_flag(Flags::Z, 0);
    assert_eq!(cpu.get_flag(Flags::Z), false);

    cpu.set_flag(Flags::C, 0);
    assert_eq!(cpu.get_flag(Flags::C), false);
}

#[test]
fn cpu_get_flag() {
    let mut cpu: MOS6510 = MOS6510::new();
    cpu.P = 0b1000_0000;
    assert_eq!(cpu.get_flag(Flags::N), true);
    cpu.P = 0b0100_0000;
    assert_eq!(cpu.get_flag(Flags::V), true);
    cpu.P = 0b0001_0000;
    assert_eq!(cpu.get_flag(Flags::B), true);
    cpu.P = 0b0000_1000;
    assert_eq!(cpu.get_flag(Flags::D), true);
    cpu.P = 0b0000_0100;
    assert_eq!(cpu.get_flag(Flags::I), true);
    cpu.P = 0b0000_0010;
    assert_eq!(cpu.get_flag(Flags::Z), true);
    cpu.P = 0b0000_0001;
    assert_eq!(cpu.get_flag(Flags::C), true);
    cpu.P = 0;
    assert_eq!(cpu.get_flag(Flags::N), false);
    assert_eq!(cpu.get_flag(Flags::V), false);
    assert_eq!(cpu.get_flag(Flags::B), false);
    assert_eq!(cpu.get_flag(Flags::D), false);
    assert_eq!(cpu.get_flag(Flags::I), false);
    assert_eq!(cpu.get_flag(Flags::Z), false);
    assert_eq!(cpu.get_flag(Flags::C), false);
}
