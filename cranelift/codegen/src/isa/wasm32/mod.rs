use crate::dominator_tree::DominatorTree;
use crate::ir::{Function, Type};

use crate::isa::{Builder as IsaBuilder, FunctionAlignment, TargetIsa};
use crate::machinst::{CompiledCode, CompiledCodeStencil, TextSectionBuilder};
use crate::result::CodegenResult;
use crate::settings as shared_settings;
use alloc::{boxed::Box, vec::Vec};
use cranelift_control::ControlPlane;
pub use settings::Flags as Wasm32Flags;
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
    isa_flags: settings::Flags,
}

impl Wasm32Backend {
    /// Create a new wasm32 backend with the given (shared) flags.
    pub fn new_with_flags(
        triple: Triple,
        flags: shared_settings::Flags,
        isa_flags: settings::Flags,
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
        _func: &Function,
        _domtree: &DominatorTree,
        _ctrl_plane: &mut ControlPlane,
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

    fn dynamic_vector_bytes(&self, _dynamic_ty: crate::ir::Type) -> u32 {
        todo!()
    }

    fn compile_function(
        &self,
        _func: &Function,
        _domtree: &DominatorTree,
        _want_disasm: bool,
        _ctrl_plane: &mut ControlPlane,
    ) -> CodegenResult<CompiledCodeStencil> {
        todo!()
    }

    fn emit_unwind_info(
        &self,
        _result: &CompiledCode,
        _kind: super::unwind::UnwindInfoKind,
    ) -> CodegenResult<Option<crate::isa::unwind::UnwindInfo>> {
        todo!()
    }

    fn text_section_builder(&self, _num_labeled_funcs: usize) -> Box<dyn TextSectionBuilder> {
        todo!()
    }

    fn function_alignment(&self) -> FunctionAlignment {
        todo!()
    }

    fn page_size_align_log2(&self) -> u8 {
        todo!()
    }

    fn has_native_fma(&self) -> bool {
        false
    }

    fn has_x86_blendv_lowering(&self, _ty: Type) -> bool {
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

/// Create a new Pulley ISA builder.
pub fn isa_builder(triple: Triple) -> IsaBuilder {
    assert!(matches!(triple.architecture, Architecture::Wasm32));

    IsaBuilder {
        triple,
        setup: self::settings::builder(),
        constructor: |triple, shared_flags, builder| {
            let isa_flags = Wasm32Flags::new(&shared_flags, builder);
            let backend = Wasm32Backend::new_with_flags(triple, shared_flags, isa_flags);
            Ok(backend.wrapped())
        },
    }
}
