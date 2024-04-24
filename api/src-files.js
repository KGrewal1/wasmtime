var srcIndex = new Map(JSON.parse('[\
["byte_array_literals",["",[],["lib.rs"]]],\
["clif_json",["",[],["clif-json.rs"]]],\
["clif_util",["",[],["bugpoint.rs","cat.rs","clif-util.rs","compile.rs","disasm.rs","interpret.rs","print_cfg.rs","run.rs","souper_harvest.rs","utils.rs"]]],\
["component_fuzz_util",["",[],["lib.rs"]]],\
["component_macro_test",["",[],["lib.rs"]]],\
["component_macro_test_helpers",["",[],["lib.rs"]]],\
["component_test_util",["",[],["lib.rs"]]],\
["cranelift",["",[],["lib.rs"]]],\
["cranelift_bforest",["",[],["lib.rs","map.rs","node.rs","path.rs","pool.rs","set.rs"]]],\
["cranelift_codegen",["",[["binemit",[],["mod.rs","stack_map.rs"]],["egraph",[],["cost.rs","elaborate.rs"]],["ir",[],["atomic_rmw_op.rs","builder.rs","condcodes.rs","constant.rs","dfg.rs","dynamic_type.rs","entities.rs","extfunc.rs","extname.rs","function.rs","globalvalue.rs","immediates.rs","instructions.rs","jumptable.rs","known_symbol.rs","layout.rs","libcall.rs","memflags.rs","memtype.rs","mod.rs","pcc.rs","progpoint.rs","sourceloc.rs","stackslot.rs","trapcode.rs","types.rs"]],["isa",[["aarch64",[["inst",[["unwind",[],["systemv.rs"]]],["args.rs","emit.rs","imms.rs","mod.rs","regs.rs","unwind.rs"]],["lower",[["isle",[],["generated_code.rs"]]],["isle.rs"]]],["abi.rs","lower.rs","mod.rs","pcc.rs","settings.rs"]],["riscv64",[["inst",[["unwind",[],["systemv.rs"]]],["args.rs","emit.rs","encode.rs","imms.rs","mod.rs","regs.rs","unwind.rs","vector.rs"]],["lower",[["isle",[],["generated_code.rs"]]],["isle.rs"]]],["abi.rs","lower.rs","mod.rs","settings.rs"]],["s390x",[["inst",[["unwind",[],["systemv.rs"]]],["args.rs","emit.rs","imms.rs","mod.rs","regs.rs","unwind.rs"]],["lower",[["isle",[],["generated_code.rs"]]],["isle.rs"]]],["abi.rs","lower.rs","mod.rs","settings.rs"]],["unwind",[],["systemv.rs","winx64.rs"]],["x64",[["encoding",[],["evex.rs","mod.rs","rex.rs","vex.rs"]],["inst",[["unwind",[],["systemv.rs","winx64.rs"]]],["args.rs","emit.rs","emit_state.rs","mod.rs","regs.rs","unwind.rs"]],["lower",[["isle",[],["generated_code.rs"]]],["isle.rs"]]],["abi.rs","lower.rs","mod.rs","pcc.rs","settings.rs"]]],["call_conv.rs","mod.rs","unwind.rs"]],["legalizer",[],["globalvalue.rs","mod.rs"]],["machinst",[],["abi.rs","blockorder.rs","buffer.rs","compile.rs","helpers.rs","inst_common.rs","isle.rs","lower.rs","mod.rs","pcc.rs","reg.rs","valueregs.rs","vcode.rs"]],["opts",[],["generated_code.rs"]],["verifier",[],["mod.rs"]]],["alias_analysis.rs","bitset.rs","cfg_printer.rs","constant_hash.rs","context.rs","ctxhash.rs","cursor.rs","data_value.rs","dbg.rs","dce.rs","dominator_tree.rs","egraph.rs","flowgraph.rs","fx.rs","incremental_cache.rs","inst_predicates.rs","isle_prelude.rs","iterators.rs","lib.rs","loop_analysis.rs","nan_canonicalization.rs","opts.rs","print_errors.rs","remove_constant_phis.rs","result.rs","scoped_hash_map.rs","settings.rs","souper_harvest.rs","timing.rs","unionfind.rs","unreachable_code.rs","value_label.rs","write.rs"]]],\
["cranelift_codegen_meta",["",[["cdsl",[],["formats.rs","instructions.rs","isa.rs","mod.rs","operands.rs","settings.rs","types.rs","typevar.rs"]],["isa",[],["arm64.rs","mod.rs","riscv64.rs","s390x.rs","x86.rs"]],["shared",[],["entities.rs","formats.rs","immediates.rs","instructions.rs","mod.rs","settings.rs","types.rs"]]],["constant_hash.rs","error.rs","gen_inst.rs","gen_settings.rs","gen_types.rs","isle.rs","lib.rs","srcgen.rs","unique_table.rs"]]],\
["cranelift_codegen_shared",["",[],["constant_hash.rs","constants.rs","lib.rs"]]],\
["cranelift_control",["",[],["lib.rs","zero_sized.rs"]]],\
["cranelift_entity",["",[],["boxed_slice.rs","iter.rs","keys.rs","lib.rs","list.rs","map.rs","packed_option.rs","primary.rs","set.rs","sparse.rs","unsigned.rs"]]],\
["cranelift_filetests",["",[],["concurrent.rs","function_runner.rs","lib.rs","match_directive.rs","runner.rs","runone.rs","subtest.rs","test_alias_analysis.rs","test_cat.rs","test_compile.rs","test_dce.rs","test_domtree.rs","test_interpret.rs","test_legalizer.rs","test_optimize.rs","test_print_cfg.rs","test_run.rs","test_safepoint.rs","test_unwind.rs","test_verifier.rs"]]],\
["cranelift_frontend",["",[],["frontend.rs","lib.rs","ssa.rs","switch.rs","variable.rs"]]],\
["cranelift_fuzzgen",["",[["passes",[],["fcvt.rs","int_divz.rs","mod.rs"]]],["config.rs","cranelift_arbitrary.rs","function_generator.rs","lib.rs","print.rs","target_isa_extras.rs"]]],\
["cranelift_interpreter",["",[],["address.rs","environment.rs","frame.rs","instruction.rs","interpreter.rs","lib.rs","state.rs","step.rs","value.rs"]]],\
["cranelift_isle",["",[],["ast.rs","codegen.rs","compile.rs","error.rs","lexer.rs","lib.rs","log.rs","overlap.rs","parser.rs","sema.rs","serialize.rs","trie_again.rs"]]],\
["cranelift_jit",["",[],["backend.rs","compiled_blob.rs","lib.rs","memory.rs"]]],\
["cranelift_module",["",[],["data_context.rs","lib.rs","module.rs","traps.rs"]]],\
["cranelift_native",["",[],["lib.rs"]]],\
["cranelift_object",["",[],["backend.rs","lib.rs"]]],\
["cranelift_reader",["",[],["error.rs","isaspec.rs","lexer.rs","lib.rs","parser.rs","run_command.rs","sourcemap.rs","testcommand.rs","testfile.rs"]]],\
["cranelift_wasm",["",[["code_translator",[],["bounds_checks.rs"]],["environ",[],["mod.rs","spec.rs"]]],["code_translator.rs","func_translator.rs","heap.rs","lib.rs","module_translator.rs","sections_translator.rs","state.rs","table.rs","translation_utils.rs"]]],\
["embedding",["",[],["allocator.rs","lib.rs"]]],\
["fib",["",[],["fib.rs"]]],\
["guest",["",[],["guest.rs"]]],\
["islec",["",[],["main.rs"]]],\
["min_platform_host",["",[],["main.rs"]]],\
["test_programs_artifacts",["",[],["lib.rs"]]],\
["tokio_wasi",["",[],["tokio-wasi.rs"]]],\
["verify_component_adapter",["",[],["main.rs"]]],\
["wasi",["",[],["wasi.rs"]]],\
["wasi_common",["",[["sched",[],["subscription.rs"]],["snapshots",[["preview_1",[],["error.rs"]]],["mod.rs","preview_0.rs","preview_1.rs"]],["sync",[["sched",[],["unix.rs"]]],["clocks.rs","dir.rs","file.rs","mod.rs","net.rs","sched.rs","stdio.rs"]]],["clocks.rs","ctx.rs","dir.rs","error.rs","file.rs","lib.rs","pipe.rs","random.rs","sched.rs","string_array.rs","table.rs"]]],\
["wasi_snapshot_preview1",["",[],["descriptors.rs","lib.rs","macros.rs"]]],\
["wasm_spec_interpreter",["",[],["lib.rs","without_library.rs"]]],\
["wasmtime",["",[["compile",[],["code_builder.rs","runtime.rs"]],["engine",[],["serialization.rs"]],["profiling_agent",[],["jitdump.rs","perfmap.rs","vtune.rs"]],["runtime",[["component",[["func",[],["host.rs","options.rs","typed.rs"]]],["component.rs","func.rs","instance.rs","linker.rs","matching.rs","mod.rs","resource_table.rs","resources.rs","storage.rs","store.rs","types.rs","values.rs"]],["externals",[],["global.rs","table.rs"]],["func",[],["typed.rs"]],["gc",[["enabled",[],["anyref.rs","externref.rs","i31.rs","rooting.rs"]]],["enabled.rs"]],["module",[],["registry.rs"]],["store",[],["context.rs","data.rs","func_refs.rs"]],["trampoline",[],["func.rs","global.rs","memory.rs","table.rs"]],["types",[],["matching.rs"]]],["code.rs","code_memory.rs","coredump.rs","debug.rs","externals.rs","func.rs","gc.rs","instance.rs","instantiate.rs","limits.rs","linker.rs","memory.rs","module.rs","profiling.rs","resources.rs","stack.rs","store.rs","trampoline.rs","trap.rs","type_registry.rs","types.rs","uninhabited.rs","unix.rs","v128.rs","values.rs"]]],["compile.rs","config.rs","engine.rs","lib.rs","profiling_agent.rs","runtime.rs"]]],\
["wasmtime_asm_macros",["",[],["lib.rs"]]],\
["wasmtime_bench_api",["",[],["lib.rs","unsafe_send_sync.rs"]]],\
["wasmtime_c_api",["",[["types",[],["export.rs","extern.rs","func.rs","global.rs","import.rs","memory.rs","table.rs","val.rs"]]],["async.rs","config.rs","engine.rs","error.rs","extern.rs","func.rs","global.rs","instance.rs","lib.rs","linker.rs","memory.rs","module.rs","profiling.rs","ref.rs","sharedmemory.rs","store.rs","table.rs","trap.rs","types.rs","val.rs","vec.rs","wasi.rs","wat2wasm.rs"]]],\
["wasmtime_c_api_macros",["",[],["lib.rs"]]],\
["wasmtime_cache",["",[],["config.rs","lib.rs","worker.rs"]]],\
["wasmtime_cli_flags",["",[],["lib.rs","opt.rs"]]],\
["wasmtime_component_macro",["",[],["bindgen.rs","component.rs","lib.rs"]]],\
["wasmtime_component_util",["",[],["lib.rs"]]],\
["wasmtime_cranelift",["",[["compiler",[],["component.rs"]],["debug",[["transform",[],["address_transform.rs","attr.rs","expression.rs","line_program.rs","mod.rs","range_info_builder.rs","refs.rs","simulate.rs","unit.rs","utils.rs"]]],["gc.rs","write_debuginfo.rs"]],["gc",[],["enabled.rs"]]],["builder.rs","compiled_function.rs","compiler.rs","debug.rs","func_environ.rs","gc.rs","isa_builder.rs","lib.rs","obj.rs"]]],\
["wasmtime_environ",["",[["compile",[],["address_map.rs","mod.rs","module_artifacts.rs","trap_encoding.rs"]],["component",[["translate",[],["adapt.rs","inline.rs"]],["types",[],["resources.rs"]]],["artifacts.rs","compiler.rs","dfg.rs","info.rs","translate.rs","types.rs","vmcomponent_offsets.rs"]],["fact",[],["core_types.rs","signature.rs","trampoline.rs","transcode.rs","traps.rs"]]],["address_map.rs","builtin.rs","component.rs","demangling.rs","fact.rs","gc.rs","lib.rs","module.rs","module_artifacts.rs","module_environ.rs","module_types.rs","obj.rs","ref_bits.rs","scopevec.rs","stack_map.rs","trap_encoding.rs","tunables.rs","vmoffsets.rs"]]],\
["wasmtime_explorer",["",[],["lib.rs"]]],\
["wasmtime_fiber",["",[["unix",[],["x86_64.rs"]]],["lib.rs","unix.rs"]]],\
["wasmtime_fuzzing",["",[["generators",[],["api.rs","async_config.rs","codegen_settings.rs","component_types.rs","config.rs","instance_allocation_strategy.rs","memory.rs","module.rs","pooling_config.rs","single_inst_module.rs","stacks.rs","table_ops.rs","value.rs","wast_test.rs"]],["oracles",[],["diff_spec.rs","diff_v8.rs","diff_wasmi.rs","diff_wasmtime.rs","dummy.rs","engine.rs","stacks.rs"]]],["generators.rs","lib.rs","mutators.rs","oracles.rs","single_module_fuzzer.rs"]]],\
["wasmtime_jit_debug",["",[],["gdb_jit_int.rs","lib.rs","perf_jitdump.rs"]]],\
["wasmtime_jit_icache_coherence",["",[],["lib.rs","libc.rs"]]],\
["wasmtime_runtime",["",[["arch",[],["mod.rs","x86_64.rs"]],["component",[],["libcalls.rs","resources.rs"]],["gc",[["enabled",[],["drc.rs","externref.rs","free_list.rs"]]],["enabled.rs","gc_ref.rs","gc_runtime.rs","host_data.rs","i31.rs"]],["instance",[["allocator",[["pooling",[],["gc_heap_pool.rs","generic_stack_pool.rs","index_allocator.rs","memory_pool.rs","table_pool.rs","unix_stack_pool.rs"]]],["on_demand.rs","pooling.rs"]]],["allocator.rs"]],["mpk",[],["enabled.rs","mod.rs","pkru.rs","sys.rs"]],["sys",[["unix",[],["mmap.rs","mod.rs","signals.rs","unwind.rs","vm.rs"]]],["mod.rs"]],["threads",[],["mod.rs","parking_spot.rs","shared_memory.rs"]],["traphandlers",[],["backtrace.rs","coredump_enabled.rs"]],["vmcontext",[],["vm_host_func_context.rs"]]],["async_yield.rs","component.rs","const_expr.rs","cow.rs","debug_builtins.rs","export.rs","gc.rs","imports.rs","instance.rs","lib.rs","libcalls.rs","memory.rs","mmap.rs","mmap_vec.rs","module_id.rs","send_sync_ptr.rs","store_box.rs","table.rs","traphandlers.rs","vmcontext.rs"]]],\
["wasmtime_slab",["",[],["lib.rs"]]],\
["wasmtime_types",["",[],["error.rs","lib.rs"]]],\
["wasmtime_versioned_export_macros",["",[],["lib.rs"]]],\
["wasmtime_wasi",["",[["clocks",[],["host.rs"]],["host",[["filesystem",[],["sync.rs"]]],["clocks.rs","env.rs","exit.rs","filesystem.rs","instance_network.rs","io.rs","mod.rs","network.rs","random.rs","tcp.rs","tcp_create_socket.rs","udp.rs","udp_create_socket.rs"]],["stdio",[],["worker_thread_stdin.rs"]]],["bindings.rs","clocks.rs","ctx.rs","error.rs","filesystem.rs","ip_name_lookup.rs","lib.rs","network.rs","pipe.rs","poll.rs","preview0.rs","preview1.rs","random.rs","runtime.rs","stdio.rs","stream.rs","tcp.rs","udp.rs","write_stream.rs"]]],\
["wasmtime_wasi_http",["",[],["body.rs","error.rs","http_impl.rs","io.rs","lib.rs","proxy.rs","types.rs","types_impl.rs"]]],\
["wasmtime_wasi_nn",["",[["backend",[],["mod.rs","openvino.rs"]],["registry",[],["in_memory.rs","mod.rs"]]],["ctx.rs","lib.rs","testing.rs","wit.rs","witx.rs"]]],\
["wasmtime_wasi_threads",["",[],["lib.rs"]]],\
["wasmtime_wast",["",[],["component.rs","core.rs","lib.rs","spectest.rs","wast.rs"]]],\
["wasmtime_winch",["",[],["builder.rs","compiler.rs","lib.rs"]]],\
["wasmtime_wit_bindgen",["",[],["lib.rs","rust.rs","source.rs","types.rs"]]],\
["wasmtime_wmemcheck",["",[],["lib.rs"]]],\
["wiggle",["",[],["borrow.rs","error.rs","guest_type.rs","lib.rs","region.rs","wasmtime.rs"]]],\
["wiggle_generate",["",[["types",[],["error.rs","flags.rs","handle.rs","mod.rs","record.rs","variant.rs"]]],["codegen_settings.rs","config.rs","funcs.rs","lib.rs","lifetimes.rs","module_trait.rs","names.rs","wasmtime.rs"]]],\
["wiggle_macro",["",[],["lib.rs"]]],\
["wiggle_test",["",[],["lib.rs"]]],\
["winch_codegen",["",[["abi",[],["local.rs","mod.rs"]],["codegen",[],["bounds.rs","builtin.rs","call.rs","context.rs","control.rs","env.rs","mod.rs"]],["frame",[],["mod.rs"]],["isa",[["x64",[],["abi.rs","address.rs","asm.rs","masm.rs","mod.rs","regs.rs"]]],["mod.rs","reg.rs"]]],["lib.rs","masm.rs","regalloc.rs","regset.rs","stack.rs","visitor.rs"]]]\
]'));
createSrcSidebar();
