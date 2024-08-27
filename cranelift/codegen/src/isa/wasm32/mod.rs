use crate::dominator_tree::DominatorTree;
use crate::ir::{Function, Type};

use crate::isa::{Builder as IsaBuilder, FunctionAlignment, TargetIsa};
use crate::machinst::{
    compile, CompiledCode, CompiledCodeStencil, MachInst, MachTextSectionBuilder, Reg, SigSet,
    TextSectionBuilder, VCode,
};
use crate::result::CodegenResult;
use crate::settings as shared_settings;
use alloc::{boxed::Box, vec::Vec};
use core::fmt;
use cranelift_control::ControlPlane;
use target_lexicon::{Architecture, Triple};
// New backend:
mod abi;
pub(crate) mod inst;
mod lower;
mod settings;

/// A wasm32 backend.
pub struct Wasm32Backend {
    triple: Triple,
    flags: shared_settings::Flags,
    isa_flags: wasm32_settings::Flags,
}

impl Wasm32Backend {
    /// Create a new wasm32 backend with the given (shared) flags.
    pub fn new_with_flags(
        triple: Triple,
        flags: shared_settings::Flags,
        isa_flags: wasm32_settings::Flags,
    ) -> Self {
        Wasm32Backend {
            triple,
            flags,
            isa_flags,
        }
    }

    /// This performs lowering to VCode, register-allocates the code, computes block layout and
    /// finalizes branches. The result is ready for binary emission.
    fn compile_vcode(
        &self,
        func: &Function,
        domtree: &DominatorTree,
        ctrl_plane: &mut ControlPlane,
    ) -> ! {
        todo!();
        // CodegenResult<(VCode<inst::Inst>, regalloc2::Output)> {
        // let emit_info = EmitInfo::new(self.isa_flags.clone());
        // let sigs = SigSet::new::<abi::Wasm32MachineDeps>(func, &self.flags)?;
        // let abi = abi::Wasm32Callee::new(func, self, &self.isa_flags, &sigs)?;
        // compile::compile::<Wasm32Backend>(func, domtree, self, abi, emit_info, sigs, ctrl_plane)
    }
}

impl core::fmt::Display for Wasm32Backend {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Wasm32Backend")
            .field("name", &self.name())
            .field("triple", &self.triple())
            .field("flags", &format!("{}", self.flags()))
            .finish()
    }
}

impl TargetIsa for Wasm32Backend {
    fn name(&self) -> &'static str {
        "wasm32"
    }

    fn triple(&self) -> &Triple {
        &self.triple
    }

    fn flags(&self) -> &shared_settings::Flags {
        &self.flags
    }

    fn isa_flags(&self) -> Vec<shared_settings::Value> {
        self.isa_flags.iter().collect()
    }

    fn dynamic_vector_bytes(&self, dynamic_ty: crate::ir::Type) -> u32 {
        todo!()
    }

    fn compile_function(
        &self,
        func: &Function,
        domtree: &DominatorTree,
        want_disasm: bool,
        ctrl_plane: &mut ControlPlane,
    ) -> CodegenResult<CompiledCodeStencil> {
        todo!()
    }

    fn emit_unwind_info(
        &self,
        result: &CompiledCode,
        kind: super::unwind::UnwindInfoKind,
    ) -> CodegenResult<Option<crate::isa::unwind::UnwindInfo>> {
        todo!()
    }

    fn text_section_builder(&self, num_labeled_funcs: usize) -> Box<dyn TextSectionBuilder> {
        todo!()
    }

    fn function_alignment(&self) -> FunctionAlignment {
        todo!()
    }

    fn page_size_align_log2(&self) -> u8 {
        todo!()
    }

    fn has_native_fma(&self) -> bool {
        true
    }

    fn has_x86_blendv_lowering(&self, ty: Type) -> bool {
        false
    }

    fn has_x86_pshufb_lowering(&self) -> bool {
        false
    }

    fn has_x86_pmulhrsw_lowering(&self) -> bool {
        false
    }

    fn has_x86_pmaddubsw_lowering(&self) -> bool {
        false
    }
}
