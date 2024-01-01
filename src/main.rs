#![allow(unused)]

mod cpu;
use self::cpu::MOS6510;

fn main() {

    // for this to work i needed to:
    // make a .bashrc/.zshrc with two env variables:
    // export SFML_INCLUDE_DIR=/opt/homebrew/Cellar/sfml/2.5.1_2/include
	// export SFML_LIBS_DIR=/opt/homebrew/Cellar/sfml/2.5.1_2/lib
	// i got the path of the brew-installed sfml with: brew info sfml.
	// then to make them available i needed to run the following command:
	// 		source ~/.bashrc

    let mut c64 : MOS6510 = MOS6510::new();
    c64.init();
    c64.cycle();
}
