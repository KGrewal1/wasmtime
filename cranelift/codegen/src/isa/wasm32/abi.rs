use crate::isa::wasm32::{inst::*, settings as wasm32_settings};
use crate::machinst::{
    ABIArg, ABIArgSlot, ABIMachineSpec, ArgsAccumulator, ArgsOrRets, CallArgList, CallDest,
    CallRetList, FrameLayout, SmallInstVec, StackAMode,
};
use crate::{ir, isa, settings, CodegenResult, Reg, Writable};
use smallvec::smallvec;

// pub(crate) type Wasm32CallSite = CallSite<Wasm32MachineDeps>;

pub struct Wasm32MachineDeps;

impl ABIMachineSpec for Wasm32MachineDeps {
    type I = Inst;

    type F = wasm32_settings::Flags;

    fn word_bits() -> u32 {
        32
    }

    fn stack_align(_call_conv: isa::CallConv) -> u32 {
        todo!()
    }

    fn compute_arg_locs(
        _call_conv: isa::CallConv,
        _flags: &settings::Flags,
        params: &[ir::AbiParam],
        _args_or_rets: ArgsOrRets,
        _add_ret_area_ptr: bool,
        mut args: ArgsAccumulator,
    ) -> CodegenResult<(u32, Option<usize>)> {
        for param in params {
            args.push(ABIArg::Slots {
                slots: smallvec![ABIArgSlot::Reg {
                    reg: todo!(),
                    ty: todo!(),
                    extension: todo!()
                }],
                purpose: param.purpose,
            });
        }
        // todo!()
        Ok((0, None))
    }

    fn gen_load_stack(_mem: StackAMode, _into_reg: Writable<Reg>, _ty: ir::Type) -> Self::I {
        todo!()
    }

    fn gen_store_stack(_mem: StackAMode, _from_reg: Reg, _ty: ir::Type) -> Self::I {
        todo!()
    }

    fn gen_move(_to_reg: Writable<Reg>, _from_reg: Reg, _ty: ir::Type) -> Self::I {
        todo!()
    }

    fn gen_extend(
        _to_reg: Writable<Reg>,
        _from_reg: Reg,
        _is_signed: bool,
        _from_bits: u8,
        _to_bits: u8,
    ) -> Self::I {
        todo!()
    }

    fn gen_args(_args: alloc::vec::Vec<crate::machinst::ArgPair>) -> Self::I {
        todo!()
    }

    fn gen_rets(_rets: alloc::vec::Vec<crate::machinst::RetPair>) -> Self::I {
        todo!()
    }

    fn gen_add_imm(
        _call_conv: isa::CallConv,
        _into_reg: Writable<Reg>,
        _from_reg: Reg,
        _imm: u32,
    ) -> SmallInstVec<Self::I> {
        todo!()
    }

    fn gen_stack_lower_bound_trap(_limit_reg: Reg) -> SmallInstVec<Self::I> {
        todo!()
    }

    fn gen_get_stack_addr(_mem: StackAMode, _into_reg: Writable<Reg>) -> Self::I {
        todo!()
    }

    fn get_stacklimit_reg(_call_conv: isa::CallConv) -> Reg {
        todo!()
    }

    fn gen_load_base_offset(
        _into_reg: Writable<Reg>,
        _base: Reg,
        _offset: i32,
        _ty: ir::Type,
    ) -> Self::I {
        todo!()
    }

    fn gen_store_base_offset(_base: Reg, _offset: i32, _from_reg: Reg, _ty: ir::Type) -> Self::I {
        todo!()
    }

    fn gen_sp_reg_adjust(_amount: i32) -> SmallInstVec<Self::I> {
        todo!()
    }

    fn compute_frame_layout(
        _call_conv: isa::CallConv,
        _flags: &settings::Flags,
        _sig: &ir::Signature,
        _regs: &[Writable<crate::RealReg>],
        _is_leaf: bool,
        _incoming_args_size: u32,
        _tail_args_size: u32,
        _fixed_frame_storage_size: u32,
        _outgoing_args_size: u32,
    ) -> FrameLayout {
        todo!()
    }

    fn gen_prologue_frame_setup(
        _call_conv: isa::CallConv,
        _flags: &settings::Flags,
        _isa_flags: &Self::F,
        _frame_layout: &FrameLayout,
    ) -> SmallInstVec<Self::I> {
        todo!()
    }

    fn gen_epilogue_frame_restore(
        _call_conv: isa::CallConv,
        _flags: &settings::Flags,
        _isa_flags: &Self::F,
        _frame_layout: &FrameLayout,
    ) -> SmallInstVec<Self::I> {
        todo!()
    }

    fn gen_return(
        _call_conv: isa::CallConv,
        _isa_flags: &Self::F,
        _frame_layout: &FrameLayout,
    ) -> SmallInstVec<Self::I> {
        todo!()
    }

    fn gen_probestack(_insts: &mut SmallInstVec<Self::I>, _frame_size: u32) {
        todo!()
    }

    fn gen_inline_probestack(
        _insts: &mut SmallInstVec<Self::I>,
        _call_conv: isa::CallConv,
        _frame_size: u32,
        _guard_size: u32,
    ) {
        todo!()
    }

    fn gen_clobber_save(
        _call_conv: isa::CallConv,
        _flags: &settings::Flags,
        _frame_layout: &FrameLayout,
    ) -> smallvec::SmallVec<[Self::I; 16]> {
        todo!()
    }

    fn gen_clobber_restore(
        _call_conv: isa::CallConv,
        _flags: &settings::Flags,
        _frame_layout: &FrameLayout,
    ) -> smallvec::SmallVec<[Self::I; 16]> {
        todo!()
    }

    fn gen_call(
        _dest: &CallDest,
        _uses: CallArgList,
        _defs: CallRetList,
        _clobbers: regalloc2::PRegSet,
        _tmp: Writable<Reg>,
        _callee_conv: isa::CallConv,
        _caller_conv: isa::CallConv,
        _callee_pop_size: u32,
    ) -> smallvec::SmallVec<[Self::I; 2]> {
        todo!()
    }

    fn gen_memcpy<F: FnMut(ir::Type) -> Writable<Reg>>(
        _call_conv: isa::CallConv,
        _dst: Reg,
        _src: Reg,
        _size: usize,
        _alloc_tmp: F,
    ) -> smallvec::SmallVec<[Self::I; 8]> {
        todo!()
    }

    fn get_number_of_spillslots_for_value(
        _rc: crate::machinst::RegClass,
        _target_vector_bytes: u32,
        _isa_flags: &Self::F,
    ) -> u32 {
        todo!()
    }

    fn get_machine_env(
        _flags: &settings::Flags,
        _call_conv: isa::CallConv,
    ) -> &regalloc2::MachineEnv {
        todo!()
    }

    fn get_regs_clobbered_by_call(_call_conv_of_callee: isa::CallConv) -> regalloc2::PRegSet {
        todo!()
    }

    fn get_ext_mode(
        _call_conv: isa::CallConv,
        _specified: ir::ArgumentExtension,
    ) -> ir::ArgumentExtension {
        todo!()
    }
}
