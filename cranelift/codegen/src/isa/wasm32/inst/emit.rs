//! Wasm32 ISA: binary code emission.

use cranelift_control::ControlPlane;

use crate::ir::{self, types::*};
use crate::isa::wasm32::abi::Wasm32MachineDeps;
use crate::isa::wasm32::inst::*;
use crate::machinst::{Callee, FrameLayout};
use crate::{settings, trace, MachBuffer, MachInstEmit, MachInstEmitState};

/// State carried between emissions of a sequence of instructions.
#[derive(Default, Clone, Debug)]
pub struct EmitState {
    /// The user stack map for the upcoming instruction, as provided to
    /// `pre_safepoint()`.
    user_stack_map: Option<ir::UserStackMap>,

    /// Only used during fuzz-testing. Otherwise, it is a zero-sized struct and
    /// optimized away at compiletime. See [cranelift_control].
    ctrl_plane: ControlPlane,

    frame_layout: FrameLayout,
}

impl MachInstEmitState<Inst> for EmitState {
    fn new(abi: &Callee<Wasm32MachineDeps>, ctrl_plane: ControlPlane) -> Self {
        EmitState {
            user_stack_map: None,
            ctrl_plane,
            frame_layout: abi.frame_layout().clone(),
        }
    }

    fn pre_safepoint(&mut self, user_stack_map: Option<ir::UserStackMap>) {
        self.user_stack_map = user_stack_map;
    }

    fn ctrl_plane_mut(&mut self) -> &mut ControlPlane {
        &mut self.ctrl_plane
    }

    fn take_ctrl_plane(self) -> ControlPlane {
        self.ctrl_plane
    }

    fn frame_layout(&self) -> &FrameLayout {
        &self.frame_layout
    }
}

impl EmitState {
    fn take_stack_map(&mut self) -> Option<ir::UserStackMap> {
        self.user_stack_map.take()
    }

    fn clear_post_insn(&mut self) {
        self.user_stack_map = None;
    }
}

/// Constant state used during function compilation.
pub struct EmitInfo;

impl EmitInfo {
    /// Create a constant state for emission of instructions.
    pub fn new() -> Self {
        Self
    }
}

impl MachInstEmit for Inst {
    type State = EmitState;

    type Info = EmitInfo;

    fn emit(&self, code: &mut MachBuffer<Self>, info: &Self::Info, state: &mut Self::State) {
        todo!()
    }

    fn pretty_print_inst(&self, state: &mut Self::State) -> alloc::string::String {
        todo!()
    }
}
