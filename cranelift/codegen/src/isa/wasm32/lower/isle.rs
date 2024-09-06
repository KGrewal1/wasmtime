//! ISLE integration glue code for wasm32 lowering.

// Pull in the ISLE generated code.
pub mod generated_code;
pub use generated_code::MInst;

use crate::{
    ir::{
        condcodes::{FloatCC, IntCC},
        immediates::*,
        types::*,
        BlockCall, ExternalName, Inst, InstructionData, MemFlags, Opcode, TrapCode, Value,
        ValueList,
    },
    isa::wasm32::{abi::Wasm32CallSite, Wasm32Backend},
    machinst::{isle::*, InstOutput, MachInst, VCodeConstant, VCodeConstantData},
};
use generated_code::Context;
use regalloc2::PReg;
use std::boxed::Box;
use std::vec::Vec;

/// The main entry point for lowering with ISLE.
pub(crate) fn lower(
    lower_ctx: &mut Lower<MInst>,
    backend: &Wasm32Backend,
    inst: Inst,
) -> Option<InstOutput> {
    // TODO: reuse the ISLE context across lowerings so we can reuse its
    // internal heap allocations.
    let mut isle_ctx = IsleContext { lower_ctx, backend };
    todo!();
    // generated_code::constructor_lower(&mut isle_ctx, inst)
}

pub(crate) fn lower_branch(
    lower_ctx: &mut Lower<MInst>,
    backend: &Wasm32Backend,
    branch: Inst,
    targets: &[MachLabel],
) -> Option<()> {
    // TODO: reuse the ISLE context across lowerings so we can reuse its
    // internal heap allocations.
    let mut isle_ctx = IsleContext { lower_ctx, backend };
    todo!();
    // generated_code::constructor_lower_branch(&mut isle_ctx, branch, targets)
}

impl Context for IsleContext<'_, '_, MInst, Wasm32Backend> {
    isle_lower_prelude_methods!();
    // isle_prelude_caller_methods!(crate::isa::wasm32::abi::Wasm32MachineDeps, Wasm32CallSite);

    fn emit(&mut self, _arg0: &MInst) -> Unit {
        todo!()
    }

    fn gen_return_call(
        &mut self,
        _arg0: SigRef,
        _arg1: ExternalName,
        _arg2: RelocDistance,
        _arg3: ValueSlice,
    ) -> InstOutput {
        todo!()
    }

    fn gen_return_call_indirect(
        &mut self,
        _arg0: SigRef,
        _arg1: Value,
        _arg2: ValueSlice,
    ) -> InstOutput {
        todo!()
    }
}
