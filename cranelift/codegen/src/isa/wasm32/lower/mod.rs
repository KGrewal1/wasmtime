use crate::machinst::{Lower, LowerBackend};

use super::Wasm32Backend;

pub mod isle;

impl LowerBackend for Wasm32Backend {
    type MInst = isle::MInst;

    fn lower(
        &self,
        ctx: &mut Lower<Self::MInst>,
        inst: crate::ir::Inst,
    ) -> Option<crate::machinst::InstOutput> {
        isle::lower(ctx, self, inst)
    }

    fn lower_branch(
        &self,
        ctx: &mut Lower<Self::MInst>,
        inst: crate::ir::Inst,
        targets: &[crate::MachLabel],
    ) -> Option<()> {
        isle::lower_branch(ctx, self, inst, targets)
    }

    type FactFlowState = ();
}
