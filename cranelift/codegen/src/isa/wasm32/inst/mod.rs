pub mod args;
pub mod emit;
pub use crate::isa::wasm32::lower::isle::generated_code::MInst as Inst;
use crate::{
    binemit::{Addend, CodeOffset, Reloc},
    ir::Type,
    isa::FunctionAlignment,
    machinst::{MachInstLabelUse, MachTerminator, OperandVisitor, RegClass},
    settings::Flags,
    MachInst, MachLabel, Reg, Writable,
};

use super::abi::Wasm32MachineDeps;

impl MachInst for Inst {
    type ABIMachineSpec = Wasm32MachineDeps;
    type LabelUse = LabelUse;

    // "CLIF" in hex, to make the trap recognizable during
    // debugging.
    const TRAP_OPCODE: &'static [u8] = &[0, 0];

    fn get_operands(&mut self, _collector: &mut impl OperandVisitor) {
        todo!()
    }

    fn is_move(&self) -> Option<(Writable<Reg>, Reg)> {
        todo!()
    }

    fn is_term(&self) -> MachTerminator {
        todo!()
    }

    fn is_trap(&self) -> bool {
        todo!()
    }

    fn is_args(&self) -> bool {
        todo!()
    }

    fn is_included_in_clobbers(&self) -> bool {
        todo!()
    }

    fn is_mem_access(&self) -> bool {
        todo!()
    }

    fn gen_move(_to_reg: Writable<Reg>, _from_reg: Reg, _ty: Type) -> Self {
        todo!()
    }

    fn gen_dummy_use(_reg: Reg) -> Self {
        todo!()
    }

    fn rc_for_type(_ty: Type) -> crate::CodegenResult<(&'static [RegClass], &'static [Type])> {
        todo!()
    }

    fn canonical_type_for_rc(_rc: RegClass) -> Type {
        todo!()
    }

    fn gen_jump(_target: MachLabel) -> Self {
        todo!()
    }

    fn gen_nop(_preferred_size: usize) -> Self {
        todo!()
    }

    fn worst_case_size() -> CodeOffset {
        todo!()
    }

    fn ref_type_regclass(_flags: &Flags) -> RegClass {
        todo!()
    }

    fn is_safepoint(&self) -> bool {
        todo!()
    }

    fn function_alignment() -> FunctionAlignment {
        FunctionAlignment {
            minimum: 1,
            preferred: 1,
        }
    }
}

//=============================================================================
// Label fixups and jump veneers.

/// Different forms of label references for different instruction formats.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LabelUse {}

impl MachInstLabelUse for LabelUse {
    const ALIGN: CodeOffset = 1;

    fn max_pos_range(self) -> CodeOffset {
        todo!()
    }

    fn max_neg_range(self) -> CodeOffset {
        todo!()
    }

    fn patch_size(self) -> CodeOffset {
        todo!()
    }

    fn patch(self, _buffer: &mut [u8], _use_offset: CodeOffset, _label_offset: CodeOffset) {
        todo!()
    }

    fn supports_veneer(self) -> bool {
        todo!()
    }

    fn veneer_size(self) -> CodeOffset {
        todo!()
    }

    fn worst_case_veneer_size() -> CodeOffset {
        todo!()
    }

    fn generate_veneer(self, _buffer: &mut [u8], _veneer_offset: CodeOffset) -> (CodeOffset, Self) {
        todo!()
    }

    fn from_reloc(_reloc: Reloc, _addend: Addend) -> Option<Self> {
        todo!()
    }
}
