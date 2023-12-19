pub mod opcode;
pub mod load;
pub mod trans;
pub mod stack;
pub mod shift;

pub mod test;

pub use opcode::Opcode;
pub use opcode::AddrReturn;
pub use load::*;
pub use trans::*;
pub use stack::*;
pub use shift::*;
