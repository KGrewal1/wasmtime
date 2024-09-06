use crate::isa::wasm32::{inst::*, settings as wasm32_settings};
use crate::machinst::{
    ABIMachineSpec, ArgsAccumulator, ArgsOrRets, CallArgList, CallDest, CallRetList, CallSite,
    FrameLayout, SmallInstVec, StackAMode,
};
use crate::{ir, isa, settings, Reg, Writable};

pub(crate) type Wasm32CallSite = CallSite<Wasm32MachineDeps>;

pub struct Wasm32MachineDeps;

impl ABIMachineSpec for Wasm32MachineDeps {
    type I = Inst;

    type F = wasm32_settings::Flags;

    fn word_bits() -> u32 {
        32
    }

    fn stack_align(call_conv: isa::CallConv) -> u32 {
        todo!()
    }

    fn compute_arg_locs(
        call_conv: isa::CallConv,
        flags: &settings::Flags,
        params: &[ir::AbiParam],
        args_or_rets: ArgsOrRets,
        add_ret_area_ptr: bool,
        args: ArgsAccumulator,
    ) -> crate::CodegenResult<(u32, Option<usize>)> {
        todo!()
    }

    fn gen_load_stack(mem: StackAMode, into_reg: Writable<Reg>, ty: ir::Type) -> Self::I {
        todo!()
    }

    fn gen_store_stack(mem: StackAMode, from_reg: Reg, ty: ir::Type) -> Self::I {
        todo!()
    }

    fn gen_move(to_reg: Writable<Reg>, from_reg: Reg, ty: ir::Type) -> Self::I {
        todo!()
    }

    fn gen_extend(
        to_reg: Writable<Reg>,
        from_reg: Reg,
        is_signed: bool,
        from_bits: u8,
        to_bits: u8,
    ) -> Self::I {
        todo!()
    }

    fn gen_args(args: alloc::vec::Vec<crate::machinst::ArgPair>) -> Self::I {
        todo!()
    }

    fn gen_rets(rets: alloc::vec::Vec<crate::machinst::RetPair>) -> Self::I {
        todo!()
    }

    fn gen_add_imm(
        call_conv: isa::CallConv,
        into_reg: Writable<Reg>,
        from_reg: Reg,
        imm: u32,
    ) -> SmallInstVec<Self::I> {
        todo!()
    }

    fn gen_stack_lower_bound_trap(limit_reg: Reg) -> SmallInstVec<Self::I> {
        todo!()
    }

    fn gen_get_stack_addr(mem: StackAMode, into_reg: Writable<Reg>) -> Self::I {
        todo!()
    }

    fn get_stacklimit_reg(call_conv: isa::CallConv) -> Reg {
        todo!()
    }

    fn gen_load_base_offset(
        into_reg: Writable<Reg>,
        base: Reg,
        offset: i32,
        ty: ir::Type,
    ) -> Self::I {
        todo!()
    }

    fn gen_store_base_offset(base: Reg, offset: i32, from_reg: Reg, ty: ir::Type) -> Self::I {
        todo!()
    }

    fn gen_sp_reg_adjust(amount: i32) -> SmallInstVec<Self::I> {
        todo!()
    }

    fn compute_frame_layout(
        call_conv: isa::CallConv,
        flags: &settings::Flags,
        sig: &ir::Signature,
        regs: &[Writable<crate::RealReg>],
        is_leaf: bool,
        incoming_args_size: u32,
        tail_args_size: u32,
        fixed_frame_storage_size: u32,
        outgoing_args_size: u32,
    ) -> FrameLayout {
        todo!()
    }

    fn gen_prologue_frame_setup(
        call_conv: isa::CallConv,
        flags: &settings::Flags,
        isa_flags: &Self::F,
        frame_layout: &FrameLayout,
    ) -> SmallInstVec<Self::I> {
        todo!()
    }

    fn gen_epilogue_frame_restore(
        call_conv: isa::CallConv,
        flags: &settings::Flags,
        isa_flags: &Self::F,
        frame_layout: &FrameLayout,
    ) -> SmallInstVec<Self::I> {
        todo!()
    }

    fn gen_return(
        call_conv: isa::CallConv,
        isa_flags: &Self::F,
        frame_layout: &FrameLayout,
    ) -> SmallInstVec<Self::I> {
        todo!()
    }

    fn gen_probestack(insts: &mut SmallInstVec<Self::I>, frame_size: u32) {
        todo!()
    }

    fn gen_inline_probestack(
        insts: &mut SmallInstVec<Self::I>,
        call_conv: isa::CallConv,
        frame_size: u32,
        guard_size: u32,
    ) {
        todo!()
    }

    fn gen_clobber_save(
        call_conv: isa::CallConv,
        flags: &settings::Flags,
        frame_layout: &FrameLayout,
    ) -> smallvec::SmallVec<[Self::I; 16]> {
        todo!()
    }

    fn gen_clobber_restore(
        call_conv: isa::CallConv,
        flags: &settings::Flags,
        frame_layout: &FrameLayout,
    ) -> smallvec::SmallVec<[Self::I; 16]> {
        todo!()
    }

    fn gen_call(
        dest: &CallDest,
        uses: CallArgList,
        defs: CallRetList,
        clobbers: regalloc2::PRegSet,
        tmp: Writable<Reg>,
        callee_conv: isa::CallConv,
        caller_conv: isa::CallConv,
        callee_pop_size: u32,
    ) -> smallvec::SmallVec<[Self::I; 2]> {
        todo!()
    }

    fn gen_memcpy<F: FnMut(ir::Type) -> Writable<Reg>>(
        call_conv: isa::CallConv,
        dst: Reg,
        src: Reg,
        size: usize,
        alloc_tmp: F,
    ) -> smallvec::SmallVec<[Self::I; 8]> {
        todo!()
    }

    fn get_number_of_spillslots_for_value(
        rc: crate::machinst::RegClass,
        target_vector_bytes: u32,
        isa_flags: &Self::F,
    ) -> u32 {
        todo!()
    }

    fn get_machine_env(
        flags: &settings::Flags,
        call_conv: isa::CallConv,
    ) -> &regalloc2::MachineEnv {
        todo!()
    }

    fn get_regs_clobbered_by_call(call_conv_of_callee: isa::CallConv) -> regalloc2::PRegSet {
        todo!()
    }

    fn get_ext_mode(
        call_conv: isa::CallConv,
        specified: ir::ArgumentExtension,
    ) -> ir::ArgumentExtension {
        todo!()
    }
}
