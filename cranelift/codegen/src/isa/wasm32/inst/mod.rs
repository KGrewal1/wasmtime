pub mod args;
pub use self::args::*;
pub mod emit;
pub use crate::isa::wasm32::lower::isle::generated_code::MInst as Inst;
