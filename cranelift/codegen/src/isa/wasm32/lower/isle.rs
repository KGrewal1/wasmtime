//! ISLE integration glue code for wasm32 lowering.

// Pull in the ISLE generated code.
pub mod generated_code;
pub use generated_code::MInst;

use crate::{ir::Inst, isa::wasm32::Wasm32Backend, machinst::isle::*};
use generated_code::Context;

/// The main entry point for lowering with ISLE.
pub(crate) fn lower(
    lower_ctx: &mut Lower<MInst>,
    backend: &Wasm32Backend,
    inst: Inst,
) -> Option<InstOutput> {
    // TODO: reuse the ISLE context across lowerings so we can reuse its
    // internal heap allocations.
    let mut isle_ctx = IsleContext { lower_ctx, backend };
    generated_code::constructor_lower(&mut isle_ctx, inst)
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
    generated_code::constructor_lower_branch(&mut isle_ctx, branch, targets)
}

impl Context for IsleContext<'_, '_, MInst, Wasm32Backend> {
    isle_lower_prelude_methods!();
    // isle_prelude_caller_methods!(
    //     crate::isa::aarch64::abi::AArch64MachineDeps,
    //     AArch64CallSite
    // );
}
