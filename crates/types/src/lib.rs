//! Internal dependency of Wasmtime and Cranelift that defines types for
//! WebAssembly.

use smallvec::SmallVec;
pub use wasmparser;

use cranelift_entity::entity_impl;
use serde_derive::{Deserialize, Serialize};
use std::{fmt, ops::Range};

mod error;
pub use error::*;

/// A trait for things that can trace all type-to-type edges, aka all type
/// indices within this thing.
pub trait TypeTrace {
    /// Visit each edge.
    ///
    /// The function can break out of tracing by returning `Err(E)`.
    fn trace<F, E>(&self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(EngineOrModuleTypeIndex) -> Result<(), E>;

    /// Visit each edge, mutably.
    ///
    /// Allows updating edges.
    ///
    /// The function can break out of tracing by returning `Err(E)`.
    fn trace_mut<F, E>(&mut self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(&mut EngineOrModuleTypeIndex) -> Result<(), E>;

    /// Trace all `VMSharedTypeIndex` edges, ignoring other edges.
    fn trace_engine_indices<F, E>(&self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(VMSharedTypeIndex) -> Result<(), E>,
    {
        self.trace(&mut |idx| match idx {
            EngineOrModuleTypeIndex::Engine(idx) => func(idx),
            EngineOrModuleTypeIndex::Module(_) | EngineOrModuleTypeIndex::RecGroup(_) => Ok(()),
        })
    }

    /// Canonicalize `self` by rewriting all type references inside `self` from
    /// module-level interned type indices to engine-level interned type
    /// indices.
    ///
    /// This produces types that are suitable for usage by the runtime (only
    /// contains `VMSharedTypeIndex` type references).
    ///
    /// This does not produce types that are suitable for hash consing types
    /// (must have recgroup-relative indices for type indices referencing other
    /// types in the same recgroup).
    fn canonicalize_for_runtime_usage<F>(&mut self, module_to_engine: &mut F)
    where
        F: FnMut(ModuleInternedTypeIndex) -> VMSharedTypeIndex,
    {
        self.trace_mut::<_, ()>(&mut |idx| match idx {
            EngineOrModuleTypeIndex::Engine(_) => Ok(()),
            EngineOrModuleTypeIndex::Module(module_index) => {
                let engine_index = module_to_engine(*module_index);
                *idx = EngineOrModuleTypeIndex::Engine(engine_index);
                Ok(())
            }
            EngineOrModuleTypeIndex::RecGroup(_) => {
                panic!("should not already be canonicalized for hash consing")
            }
        })
        .unwrap()
    }

    /// Is this type canonicalized for runtime usage?
    fn is_canonicalized_for_runtime_usage(&self) -> bool {
        self.trace(&mut |idx| match idx {
            EngineOrModuleTypeIndex::Engine(_) => Ok(()),
            EngineOrModuleTypeIndex::Module(_) | EngineOrModuleTypeIndex::RecGroup(_) => Err(()),
        })
        .is_ok()
    }

    /// Canonicalize `self` by rewriting all type references inside `self` from
    /// module-level interned type indices to either engine-level interned type
    /// indices or recgroup-relative indices.
    ///
    /// This produces types that are suitable for hash consing and deduplicating
    /// recgroups (types may have recgroup-relative indices for references to
    /// other types within the same recgroup).
    ///
    /// This does *not* produce types that are suitable for usage by the runtime
    /// (only contain `VMSharedTypeIndex` type references).
    fn canonicalize_for_hash_consing<F>(
        &mut self,
        rec_group_range: Range<ModuleInternedTypeIndex>,
        module_to_engine: &mut F,
    ) where
        F: FnMut(ModuleInternedTypeIndex) -> VMSharedTypeIndex,
    {
        self.trace_mut::<_, ()>(&mut |idx| match *idx {
            EngineOrModuleTypeIndex::Engine(_) => Ok(()),
            EngineOrModuleTypeIndex::Module(module_index) => {
                *idx = if rec_group_range.start <= module_index {
                    // Any module index within the recursion group gets
                    // translated into a recgroup-relative index.
                    debug_assert!(module_index < rec_group_range.end);
                    let relative = module_index.as_u32() - rec_group_range.start.as_u32();
                    let relative = RecGroupRelativeTypeIndex::from_u32(relative);
                    EngineOrModuleTypeIndex::RecGroup(relative)
                } else {
                    // Cross-group indices are translated directly into
                    // `VMSharedTypeIndex`es.
                    debug_assert!(module_index < rec_group_range.start);
                    EngineOrModuleTypeIndex::Engine(module_to_engine(module_index))
                };
                Ok(())
            }
            EngineOrModuleTypeIndex::RecGroup(_) => {
                panic!("should not already be canonicalized for hash consing")
            }
        })
        .unwrap()
    }

    /// Is this type canonicalized for hash consing?
    fn is_canonicalized_for_hash_consing(&self) -> bool {
        self.trace(&mut |idx| match idx {
            EngineOrModuleTypeIndex::Engine(_) | EngineOrModuleTypeIndex::RecGroup(_) => Ok(()),
            EngineOrModuleTypeIndex::Module(_) => Err(()),
        })
        .is_ok()
    }
}

/// WebAssembly value type -- equivalent of `wasmparser::ValType`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WasmValType {
    /// I32 type
    I32,
    /// I64 type
    I64,
    /// F32 type
    F32,
    /// F64 type
    F64,
    /// V128 type
    V128,
    /// Reference type
    Ref(WasmRefType),
}

impl fmt::Display for WasmValType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WasmValType::I32 => write!(f, "i32"),
            WasmValType::I64 => write!(f, "i64"),
            WasmValType::F32 => write!(f, "f32"),
            WasmValType::F64 => write!(f, "f64"),
            WasmValType::V128 => write!(f, "v128"),
            WasmValType::Ref(rt) => write!(f, "{rt}"),
        }
    }
}

impl TypeTrace for WasmValType {
    fn trace<F, E>(&self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(EngineOrModuleTypeIndex) -> Result<(), E>,
    {
        match self {
            WasmValType::Ref(r) => r.trace(func),
            WasmValType::I32
            | WasmValType::I64
            | WasmValType::F32
            | WasmValType::F64
            | WasmValType::V128 => Ok(()),
        }
    }

    fn trace_mut<F, E>(&mut self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(&mut EngineOrModuleTypeIndex) -> Result<(), E>,
    {
        match self {
            WasmValType::Ref(r) => r.trace_mut(func),
            WasmValType::I32
            | WasmValType::I64
            | WasmValType::F32
            | WasmValType::F64
            | WasmValType::V128 => Ok(()),
        }
    }
}

impl WasmValType {
    /// Is this a type that is represented as a `VMGcRef`?
    pub fn is_vmgcref_type(&self) -> bool {
        match self {
            WasmValType::Ref(r) => r.is_vmgcref_type(),
            _ => false,
        }
    }

    /// Is this a type that is represented as a `VMGcRef` and is additionally
    /// not an `i31`?
    ///
    /// That is, is this a a type that actually refers to an object allocated in
    /// a GC heap?
    pub fn is_vmgcref_type_and_not_i31(&self) -> bool {
        match self {
            WasmValType::Ref(r) => r.is_vmgcref_type_and_not_i31(),
            _ => false,
        }
    }
}

/// WebAssembly reference type -- equivalent of `wasmparser`'s RefType
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WasmRefType {
    pub nullable: bool,
    pub heap_type: WasmHeapType,
}

impl TypeTrace for WasmRefType {
    fn trace<F, E>(&self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(EngineOrModuleTypeIndex) -> Result<(), E>,
    {
        self.heap_type.trace(func)
    }

    fn trace_mut<F, E>(&mut self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(&mut EngineOrModuleTypeIndex) -> Result<(), E>,
    {
        self.heap_type.trace_mut(func)
    }
}

impl WasmRefType {
    pub const EXTERNREF: WasmRefType = WasmRefType {
        nullable: true,
        heap_type: WasmHeapType::Extern,
    };
    pub const FUNCREF: WasmRefType = WasmRefType {
        nullable: true,
        heap_type: WasmHeapType::Func,
    };

    /// Is this a type that is represented as a `VMGcRef`?
    pub fn is_vmgcref_type(&self) -> bool {
        self.heap_type.is_vmgcref_type()
    }

    /// Is this a type that is represented as a `VMGcRef` and is additionally
    /// not an `i31`?
    ///
    /// That is, is this a a type that actually refers to an object allocated in
    /// a GC heap?
    pub fn is_vmgcref_type_and_not_i31(&self) -> bool {
        self.heap_type.is_vmgcref_type_and_not_i31()
    }
}

impl fmt::Display for WasmRefType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::FUNCREF => write!(f, "funcref"),
            Self::EXTERNREF => write!(f, "externref"),
            _ => {
                if self.nullable {
                    write!(f, "(ref null {})", self.heap_type)
                } else {
                    write!(f, "(ref {})", self.heap_type)
                }
            }
        }
    }
}

/// An interned type index, either at the module or engine level.
///
/// Roughly equivalent to `wasmparser::UnpackedIndex`, although doesn't have to
/// concern itself with recursion-group-local indices.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EngineOrModuleTypeIndex {
    /// An index within an engine, canonicalized among all modules that can
    /// interact with each other.
    Engine(VMSharedTypeIndex),

    /// An index within the current Wasm module, canonicalized within just this
    /// current module.
    Module(ModuleInternedTypeIndex),

    /// An index within the containing type's rec group. This is only used when
    /// hashing and canonicalizing rec groups, and should never appear outside
    /// of the engine's type registry.
    RecGroup(RecGroupRelativeTypeIndex),
}

impl From<ModuleInternedTypeIndex> for EngineOrModuleTypeIndex {
    fn from(i: ModuleInternedTypeIndex) -> Self {
        Self::Module(i)
    }
}

impl fmt::Display for EngineOrModuleTypeIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Engine(i) => write!(f, "(engine {})", i.bits()),
            Self::Module(i) => write!(f, "(module {})", i.as_u32()),
            Self::RecGroup(i) => write!(f, "(recgroup {})", i.as_u32()),
        }
    }
}

impl EngineOrModuleTypeIndex {
    /// Is this an engine-level type index?
    pub fn is_engine_type_index(self) -> bool {
        matches!(self, Self::Engine(_))
    }

    /// Get the underlying engine-level type index, if any.
    pub fn as_engine_type_index(self) -> Option<VMSharedTypeIndex> {
        match self {
            Self::Engine(e) => Some(e),
            Self::RecGroup(_) | Self::Module(_) => None,
        }
    }

    /// Get the underlying engine-level type index, or panic.
    pub fn unwrap_engine_type_index(self) -> VMSharedTypeIndex {
        self.as_engine_type_index()
            .unwrap_or_else(|| panic!("`unwrap_engine_type_index` on {self:?}"))
    }

    /// Is this an module-level type index?
    pub fn is_module_type_index(self) -> bool {
        matches!(self, Self::Module(_))
    }

    /// Get the underlying module-level type index, if any.
    pub fn as_module_type_index(self) -> Option<ModuleInternedTypeIndex> {
        match self {
            Self::Module(e) => Some(e),
            Self::RecGroup(_) | Self::Engine(_) => None,
        }
    }

    /// Get the underlying module-level type index, or panic.
    pub fn unwrap_module_type_index(self) -> ModuleInternedTypeIndex {
        self.as_module_type_index()
            .unwrap_or_else(|| panic!("`unwrap_module_type_index` on {self:?}"))
    }

    /// Is this an recgroup-level type index?
    pub fn is_rec_group_type_index(self) -> bool {
        matches!(self, Self::RecGroup(_))
    }

    /// Get the underlying recgroup-level type index, if any.
    pub fn as_rec_group_type_index(self) -> Option<RecGroupRelativeTypeIndex> {
        match self {
            Self::RecGroup(r) => Some(r),
            Self::Module(_) | Self::Engine(_) => None,
        }
    }

    /// Get the underlying module-level type index, or panic.
    pub fn unwrap_rec_group_type_index(self) -> RecGroupRelativeTypeIndex {
        self.as_rec_group_type_index()
            .unwrap_or_else(|| panic!("`unwrap_rec_group_type_index` on {self:?}"))
    }
}

/// WebAssembly heap type -- equivalent of `wasmparser`'s HeapType
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WasmHeapType {
    Extern,
    Func,
    ConcreteFunc(EngineOrModuleTypeIndex),
    NoFunc,
    Any,
    I31,
    None,
}

impl fmt::Display for WasmHeapType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Extern => write!(f, "extern"),
            Self::Func => write!(f, "func"),
            Self::ConcreteFunc(i) => write!(f, "func {i}"),
            Self::NoFunc => write!(f, "nofunc"),
            Self::Any => write!(f, "any"),
            Self::I31 => write!(f, "i31"),
            Self::None => write!(f, "none"),
        }
    }
}

impl TypeTrace for WasmHeapType {
    fn trace<F, E>(&self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(EngineOrModuleTypeIndex) -> Result<(), E>,
    {
        match *self {
            Self::ConcreteFunc(i) => func(i),
            _ => Ok(()),
        }
    }

    fn trace_mut<F, E>(&mut self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(&mut EngineOrModuleTypeIndex) -> Result<(), E>,
    {
        match self {
            Self::ConcreteFunc(i) => func(i),
            _ => Ok(()),
        }
    }
}

impl WasmHeapType {
    /// Is this a type that is represented as a `VMGcRef`?
    pub fn is_vmgcref_type(&self) -> bool {
        match self {
            // All `t <: (ref null any)` and `t <: (ref null extern)` are
            // represented as `VMGcRef`s.
            Self::Extern | Self::Any | Self::I31 | Self::None => true,

            // All `t <: (ref null func)` are not.
            Self::Func | Self::ConcreteFunc(_) | Self::NoFunc => false,
        }
    }

    /// Is this a type that is represented as a `VMGcRef` and is additionally
    /// not an `i31`?
    ///
    /// That is, is this a a type that actually refers to an object allocated in
    /// a GC heap?
    pub fn is_vmgcref_type_and_not_i31(&self) -> bool {
        self.is_vmgcref_type() && *self != Self::I31
    }
}

/// WebAssembly function type -- equivalent of `wasmparser`'s FuncType.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct WasmFuncType {
    params: Box<[WasmValType]>,
    non_i31_gc_ref_params_count: usize,
    returns: Box<[WasmValType]>,
    non_i31_gc_ref_returns_count: usize,
}

impl TypeTrace for WasmFuncType {
    fn trace<F, E>(&self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(EngineOrModuleTypeIndex) -> Result<(), E>,
    {
        for p in self.params.iter() {
            p.trace(func)?;
        }
        for r in self.returns.iter() {
            r.trace(func)?;
        }
        Ok(())
    }

    fn trace_mut<F, E>(&mut self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(&mut EngineOrModuleTypeIndex) -> Result<(), E>,
    {
        for p in self.params.iter_mut() {
            p.trace_mut(func)?;
        }
        for r in self.returns.iter_mut() {
            r.trace_mut(func)?;
        }
        Ok(())
    }
}

impl WasmFuncType {
    #[inline]
    pub fn new(params: Box<[WasmValType]>, returns: Box<[WasmValType]>) -> Self {
        let non_i31_gc_ref_params_count = params
            .iter()
            .filter(|p| p.is_vmgcref_type_and_not_i31())
            .count();
        let non_i31_gc_ref_returns_count = returns
            .iter()
            .filter(|r| r.is_vmgcref_type_and_not_i31())
            .count();
        WasmFuncType {
            params,
            non_i31_gc_ref_params_count,
            returns,
            non_i31_gc_ref_returns_count,
        }
    }

    /// Function params types.
    #[inline]
    pub fn params(&self) -> &[WasmValType] {
        &self.params
    }

    /// How many `externref`s are in this function's params?
    #[inline]
    pub fn non_i31_gc_ref_params_count(&self) -> usize {
        self.non_i31_gc_ref_params_count
    }

    /// Returns params types.
    #[inline]
    pub fn returns(&self) -> &[WasmValType] {
        &self.returns
    }

    /// How many `externref`s are in this function's returns?
    #[inline]
    pub fn non_i31_gc_ref_returns_count(&self) -> usize {
        self.non_i31_gc_ref_returns_count
    }
}

/// A function, array, or struct type.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum WasmCompositeType {
    Func(WasmFuncType),
    //
    // TODO: Array and struct types.
}

impl WasmCompositeType {
    #[inline]
    pub fn is_func(&self) -> bool {
        matches!(self, Self::Func(_))
    }

    #[inline]
    pub fn as_func(&self) -> Option<&WasmFuncType> {
        match self {
            WasmCompositeType::Func(f) => Some(f),
        }
    }

    #[inline]
    pub fn unwrap_func(&self) -> &WasmFuncType {
        self.as_func().unwrap()
    }
}

impl TypeTrace for WasmCompositeType {
    fn trace<F, E>(&self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(EngineOrModuleTypeIndex) -> Result<(), E>,
    {
        match self {
            WasmCompositeType::Func(f) => f.trace(func),
        }
    }

    fn trace_mut<F, E>(&mut self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(&mut EngineOrModuleTypeIndex) -> Result<(), E>,
    {
        match self {
            WasmCompositeType::Func(f) => f.trace_mut(func),
        }
    }
}

/// A concrete, user-defined (or host-defined) Wasm type.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct WasmSubType {
    // TODO: is_final, supertype
    //
    /// The array, function, or struct that is defined.
    pub composite_type: WasmCompositeType,
}

impl WasmSubType {
    #[inline]
    pub fn is_func(&self) -> bool {
        self.composite_type.is_func()
    }

    #[inline]
    pub fn as_func(&self) -> Option<&WasmFuncType> {
        self.composite_type.as_func()
    }

    #[inline]
    pub fn unwrap_func(&self) -> &WasmFuncType {
        self.composite_type.unwrap_func()
    }
}

impl TypeTrace for WasmSubType {
    fn trace<F, E>(&self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(EngineOrModuleTypeIndex) -> Result<(), E>,
    {
        self.composite_type.trace(func)
    }

    fn trace_mut<F, E>(&mut self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(&mut EngineOrModuleTypeIndex) -> Result<(), E>,
    {
        self.composite_type.trace_mut(func)
    }
}

/// A recursive type group.
///
/// Types within a recgroup can have forward references to each other, which
/// allows for cyclic types, for example a function `$f` that returns a
/// reference to a function `$g` which returns a reference to a function `$f`:
///
/// ```ignore
/// (rec (type (func $f (result (ref null $g))))
///      (type (func $g (result (ref null $f)))))
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct WasmRecGroup {
    /// The types inside of this recgroup.
    pub types: Box<[WasmSubType]>,
}

impl TypeTrace for WasmRecGroup {
    fn trace<F, E>(&self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(EngineOrModuleTypeIndex) -> Result<(), E>,
    {
        for ty in self.types.iter() {
            ty.trace(func)?;
        }
        Ok(())
    }

    fn trace_mut<F, E>(&mut self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(&mut EngineOrModuleTypeIndex) -> Result<(), E>,
    {
        for ty in self.types.iter_mut() {
            ty.trace_mut(func)?;
        }
        Ok(())
    }
}

/// Index type of a function (imported or defined) inside the WebAssembly module.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct FuncIndex(u32);
entity_impl!(FuncIndex);

/// Index type of a defined function inside the WebAssembly module.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct DefinedFuncIndex(u32);
entity_impl!(DefinedFuncIndex);

/// Index type of a defined table inside the WebAssembly module.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct DefinedTableIndex(u32);
entity_impl!(DefinedTableIndex);

/// Index type of a defined memory inside the WebAssembly module.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct DefinedMemoryIndex(u32);
entity_impl!(DefinedMemoryIndex);

/// Index type of a defined memory inside the WebAssembly module.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct OwnedMemoryIndex(u32);
entity_impl!(OwnedMemoryIndex);

/// Index type of a defined global inside the WebAssembly module.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct DefinedGlobalIndex(u32);
entity_impl!(DefinedGlobalIndex);

/// Index type of a table (imported or defined) inside the WebAssembly module.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct TableIndex(u32);
entity_impl!(TableIndex);

/// Index type of a global variable (imported or defined) inside the WebAssembly module.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct GlobalIndex(u32);
entity_impl!(GlobalIndex);

/// Index type of a linear memory (imported or defined) inside the WebAssembly module.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct MemoryIndex(u32);
entity_impl!(MemoryIndex);

/// Index type of a canonicalized recursive type group inside a WebAssembly
/// module (as opposed to canonicalized within the whole engine).
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct ModuleInternedRecGroupIndex(u32);
entity_impl!(ModuleInternedRecGroupIndex);

/// Index type of a canonicalized recursive type group inside the whole engine
/// (as opposed to canonicalized within just a single Wasm module).
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct EngineInternedRecGroupIndex(u32);
entity_impl!(EngineInternedRecGroupIndex);

/// Index type of a type (imported or defined) inside the WebAssembly module.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct TypeIndex(u32);
entity_impl!(TypeIndex);

/// A canonicalized type index referencing a type within a single recursion
/// group from another type within that same recursion group.
///
/// This is only suitable for use when hash consing and deduplicating rec
/// groups.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct RecGroupRelativeTypeIndex(u32);
entity_impl!(RecGroupRelativeTypeIndex);

/// A canonicalized type index for a type within a single WebAssembly module.
///
/// Note that this is deduplicated only at the level of a single WebAssembly
/// module, not at the level of a whole store or engine. This means that these
/// indices are only unique within the context of a single Wasm module, and
/// therefore are not suitable for runtime type checks (which, in general, may
/// involve entities defined in different modules).
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct ModuleInternedTypeIndex(u32);
entity_impl!(ModuleInternedTypeIndex);

/// A canonicalized type index into an engine's shared type registry.
///
/// This is canonicalized/deduped at the level of a whole engine, across all the
/// modules loaded into that engine, not just at the level of a single
/// particular module. This means that `VMSharedTypeIndex` is usable for
/// e.g. checking that function signatures match during an indirect call
/// (potentially to a function defined in a different module) at runtime.
#[repr(transparent)] // Used directly by JIT code.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct VMSharedTypeIndex(u32);
entity_impl!(VMSharedTypeIndex);

impl VMSharedTypeIndex {
    /// Create a new `VMSharedTypeIndex`.
    #[inline]
    pub fn new(value: u32) -> Self {
        assert_ne!(
            value,
            u32::MAX,
            "u32::MAX is reserved for the default value"
        );
        Self(value)
    }

    /// Returns the underlying bits of the index.
    #[inline]
    pub fn bits(&self) -> u32 {
        self.0
    }
}

impl Default for VMSharedTypeIndex {
    #[inline]
    fn default() -> Self {
        Self(u32::MAX)
    }
}

/// Index type of a passive data segment inside the WebAssembly module.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct DataIndex(u32);
entity_impl!(DataIndex);

/// Index type of a passive element segment inside the WebAssembly module.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct ElemIndex(u32);
entity_impl!(ElemIndex);

/// Index type of an event inside the WebAssembly module.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct TagIndex(u32);
entity_impl!(TagIndex);

/// Index into the global list of modules found within an entire component.
///
/// Module translations are saved on the side to get fully compiled after
/// the original component has finished being translated.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct StaticModuleIndex(u32);
entity_impl!(StaticModuleIndex);

/// An index of an entity.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub enum EntityIndex {
    /// Function index.
    Function(FuncIndex),
    /// Table index.
    Table(TableIndex),
    /// Memory index.
    Memory(MemoryIndex),
    /// Global index.
    Global(GlobalIndex),
}

impl From<FuncIndex> for EntityIndex {
    fn from(idx: FuncIndex) -> EntityIndex {
        EntityIndex::Function(idx)
    }
}

impl From<TableIndex> for EntityIndex {
    fn from(idx: TableIndex) -> EntityIndex {
        EntityIndex::Table(idx)
    }
}

impl From<MemoryIndex> for EntityIndex {
    fn from(idx: MemoryIndex) -> EntityIndex {
        EntityIndex::Memory(idx)
    }
}

impl From<GlobalIndex> for EntityIndex {
    fn from(idx: GlobalIndex) -> EntityIndex {
        EntityIndex::Global(idx)
    }
}

/// A type of an item in a wasm module where an item is typically something that
/// can be exported.
#[allow(missing_docs)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EntityType {
    /// A global variable with the specified content type
    Global(Global),
    /// A linear memory with the specified limits
    Memory(Memory),
    /// An event definition.
    Tag(Tag),
    /// A table with the specified element type and limits
    Table(Table),
    /// A function type where the index points to the type section and records a
    /// function signature.
    Function(EngineOrModuleTypeIndex),
}

impl TypeTrace for EntityType {
    fn trace<F, E>(&self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(EngineOrModuleTypeIndex) -> Result<(), E>,
    {
        match self {
            Self::Global(g) => g.trace(func),
            Self::Table(t) => t.trace(func),
            Self::Function(idx) => func(*idx),
            Self::Memory(_) | Self::Tag(_) => Ok(()),
        }
    }

    fn trace_mut<F, E>(&mut self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(&mut EngineOrModuleTypeIndex) -> Result<(), E>,
    {
        match self {
            Self::Global(g) => g.trace_mut(func),
            Self::Table(t) => t.trace_mut(func),
            Self::Function(idx) => func(idx),
            Self::Memory(_) | Self::Tag(_) => Ok(()),
        }
    }
}

impl EntityType {
    /// Assert that this entity is a global
    pub fn unwrap_global(&self) -> &Global {
        match self {
            EntityType::Global(g) => g,
            _ => panic!("not a global"),
        }
    }

    /// Assert that this entity is a memory
    pub fn unwrap_memory(&self) -> &Memory {
        match self {
            EntityType::Memory(g) => g,
            _ => panic!("not a memory"),
        }
    }

    /// Assert that this entity is a tag
    pub fn unwrap_tag(&self) -> &Tag {
        match self {
            EntityType::Tag(g) => g,
            _ => panic!("not a tag"),
        }
    }

    /// Assert that this entity is a table
    pub fn unwrap_table(&self) -> &Table {
        match self {
            EntityType::Table(g) => g,
            _ => panic!("not a table"),
        }
    }

    /// Assert that this entity is a function
    pub fn unwrap_func(&self) -> EngineOrModuleTypeIndex {
        match self {
            EntityType::Function(g) => *g,
            _ => panic!("not a func"),
        }
    }
}

/// A WebAssembly global.
///
/// Note that we record both the original Wasm type and the Cranelift IR type
/// used to represent it. This is because multiple different kinds of Wasm types
/// might be represented with the same Cranelift IR type. For example, both a
/// Wasm `i64` and a `funcref` might be represented with a Cranelift `i64` on
/// 64-bit architectures, and when GC is not required for func refs.
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Global {
    /// The Wasm type of the value stored in the global.
    pub wasm_ty: crate::WasmValType,
    /// A flag indicating whether the value may change at runtime.
    pub mutability: bool,
}

impl TypeTrace for Global {
    fn trace<F, E>(&self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(EngineOrModuleTypeIndex) -> Result<(), E>,
    {
        let Global {
            wasm_ty,
            mutability: _,
        } = self;
        wasm_ty.trace(func)
    }

    fn trace_mut<F, E>(&mut self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(&mut EngineOrModuleTypeIndex) -> Result<(), E>,
    {
        let Global {
            wasm_ty,
            mutability: _,
        } = self;
        wasm_ty.trace_mut(func)
    }
}

/// A constant expression.
///
/// These are used to initialize globals, table elements, etc...
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ConstExpr {
    ops: SmallVec<[ConstOp; 2]>,
}

impl ConstExpr {
    /// Create a new const expression from the given opcodes.
    ///
    /// Does not do any validation that the const expression is well-typed.
    ///
    /// Panics if given zero opcodes.
    pub fn new(ops: impl IntoIterator<Item = ConstOp>) -> Self {
        let ops = ops.into_iter().collect::<SmallVec<[ConstOp; 2]>>();
        assert!(!ops.is_empty());
        ConstExpr { ops }
    }

    /// Create a new const expression from a `wasmparser` const expression.
    ///
    /// Returns the new const expression as well as the escaping function
    /// indices that appeared in `ref.func` instructions, if any.
    pub fn from_wasmparser(
        expr: wasmparser::ConstExpr<'_>,
    ) -> WasmResult<(Self, SmallVec<[FuncIndex; 1]>)> {
        let mut iter = expr
            .get_operators_reader()
            .into_iter_with_offsets()
            .peekable();

        let mut ops = SmallVec::<[ConstOp; 2]>::new();
        let mut escaped = SmallVec::<[FuncIndex; 1]>::new();
        while let Some(res) = iter.next() {
            let (op, offset) = res?;

            // If we reach an `end` instruction, and there are no more
            // instructions after that, then we are done reading this const
            // expression.
            if matches!(op, wasmparser::Operator::End) && iter.peek().is_none() {
                break;
            }

            // Track any functions that appear in `ref.func` so that callers can
            // make sure to flag them as escaping.
            if let wasmparser::Operator::RefFunc { function_index } = &op {
                escaped.push(FuncIndex::from_u32(*function_index));
            }

            ops.push(ConstOp::from_wasmparser(op, offset)?);
        }
        Ok((Self { ops }, escaped))
    }

    /// Get the opcodes that make up this const expression.
    pub fn ops(&self) -> &[ConstOp] {
        &self.ops
    }
}

/// The subset of Wasm opcodes that are constant.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum ConstOp {
    I32Const(i32),
    I64Const(i64),
    F32Const(u32),
    F64Const(u64),
    V128Const(u128),
    GlobalGet(GlobalIndex),
    RefI31,
    RefNull,
    RefFunc(FuncIndex),
}

impl ConstOp {
    /// Convert a `wasmparser::Operator` to a `ConstOp`.
    pub fn from_wasmparser(op: wasmparser::Operator<'_>, offset: usize) -> WasmResult<Self> {
        use wasmparser::Operator as O;
        Ok(match op {
            O::I32Const { value } => Self::I32Const(value),
            O::I64Const { value } => Self::I64Const(value),
            O::F32Const { value } => Self::F32Const(value.bits()),
            O::F64Const { value } => Self::F64Const(value.bits()),
            O::V128Const { value } => Self::V128Const(u128::from_le_bytes(*value.bytes())),
            O::RefNull { hty: _ } => Self::RefNull,
            O::RefFunc { function_index } => Self::RefFunc(FuncIndex::from_u32(function_index)),
            O::GlobalGet { global_index } => Self::GlobalGet(GlobalIndex::from_u32(global_index)),
            O::RefI31 => Self::RefI31,
            op => {
                return Err(wasm_unsupported!(
                    "unsupported opcode in const expression at offset {offset:#x}: {op:?}",
                ));
            }
        })
    }
}

/// WebAssembly table.
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Table {
    /// The table elements' Wasm type.
    pub wasm_ty: WasmRefType,
    /// The minimum number of elements in the table.
    pub minimum: u32,
    /// The maximum number of elements in the table.
    pub maximum: Option<u32>,
}

impl TypeTrace for Table {
    fn trace<F, E>(&self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(EngineOrModuleTypeIndex) -> Result<(), E>,
    {
        let Table {
            wasm_ty,
            minimum: _,
            maximum: _,
        } = self;
        wasm_ty.trace(func)
    }

    fn trace_mut<F, E>(&mut self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(&mut EngineOrModuleTypeIndex) -> Result<(), E>,
    {
        let Table {
            wasm_ty,
            minimum: _,
            maximum: _,
        } = self;
        wasm_ty.trace_mut(func)
    }
}

/// WebAssembly linear memory.
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Memory {
    /// The minimum number of pages in the memory.
    pub minimum: u64,
    /// The maximum number of pages in the memory.
    pub maximum: Option<u64>,
    /// Whether the memory may be shared between multiple threads.
    pub shared: bool,
    /// Whether or not this is a 64-bit memory
    pub memory64: bool,
}

impl From<wasmparser::MemoryType> for Memory {
    fn from(ty: wasmparser::MemoryType) -> Memory {
        Memory {
            minimum: ty.initial,
            maximum: ty.maximum,
            shared: ty.shared,
            memory64: ty.memory64,
        }
    }
}

/// WebAssembly event.
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// The event signature type.
    pub ty: TypeIndex,
}

impl From<wasmparser::TagType> for Tag {
    fn from(ty: wasmparser::TagType) -> Tag {
        match ty.kind {
            wasmparser::TagKind::Exception => Tag {
                ty: TypeIndex::from_u32(ty.func_type_idx),
            },
        }
    }
}

/// Helpers used to convert a `wasmparser` type to a type in this crate.
pub trait TypeConvert {
    /// Converts a wasmparser table type into a wasmtime type
    fn convert_global_type(&self, ty: &wasmparser::GlobalType) -> Global {
        Global {
            wasm_ty: self.convert_valtype(ty.content_type),
            mutability: ty.mutable,
        }
    }

    /// Converts a wasmparser table type into a wasmtime type
    fn convert_table_type(&self, ty: &wasmparser::TableType) -> Table {
        Table {
            wasm_ty: self.convert_ref_type(ty.element_type),
            minimum: ty.initial,
            maximum: ty.maximum,
        }
    }

    fn convert_sub_type(&self, ty: &wasmparser::SubType) -> WasmResult<WasmSubType> {
        if ty.supertype_idx.is_some() {
            return Err(wasm_unsupported!("wasm gc: explicit subtyping"));
        }
        let composite_type = self.convert_composite_type(&ty.composite_type)?;
        Ok(WasmSubType { composite_type })
    }

    fn convert_composite_type(
        &self,
        ty: &wasmparser::CompositeType,
    ) -> WasmResult<WasmCompositeType> {
        match ty {
            wasmparser::CompositeType::Func(f) => {
                Ok(WasmCompositeType::Func(self.convert_func_type(f)))
            }
            wasmparser::CompositeType::Array(_) => Err(wasm_unsupported!("wasm gc: array types")),
            wasmparser::CompositeType::Struct(_) => Err(wasm_unsupported!("wasm gc: struct types")),
        }
    }

    /// Converts a wasmparser function type to a wasmtime type
    fn convert_func_type(&self, ty: &wasmparser::FuncType) -> WasmFuncType {
        let params = ty
            .params()
            .iter()
            .map(|t| self.convert_valtype(*t))
            .collect();
        let results = ty
            .results()
            .iter()
            .map(|t| self.convert_valtype(*t))
            .collect();
        WasmFuncType::new(params, results)
    }

    /// Converts a wasmparser value type to a wasmtime type
    fn convert_valtype(&self, ty: wasmparser::ValType) -> WasmValType {
        match ty {
            wasmparser::ValType::I32 => WasmValType::I32,
            wasmparser::ValType::I64 => WasmValType::I64,
            wasmparser::ValType::F32 => WasmValType::F32,
            wasmparser::ValType::F64 => WasmValType::F64,
            wasmparser::ValType::V128 => WasmValType::V128,
            wasmparser::ValType::Ref(t) => WasmValType::Ref(self.convert_ref_type(t)),
        }
    }

    /// Converts a wasmparser reference type to a wasmtime type
    fn convert_ref_type(&self, ty: wasmparser::RefType) -> WasmRefType {
        WasmRefType {
            nullable: ty.is_nullable(),
            heap_type: self.convert_heap_type(ty.heap_type()),
        }
    }

    /// Converts a wasmparser heap type to a wasmtime type
    fn convert_heap_type(&self, ty: wasmparser::HeapType) -> WasmHeapType {
        match ty {
            wasmparser::HeapType::Extern => WasmHeapType::Extern,
            wasmparser::HeapType::Func => WasmHeapType::Func,
            wasmparser::HeapType::NoFunc => WasmHeapType::NoFunc,
            wasmparser::HeapType::Concrete(i) => self.lookup_heap_type(i),
            wasmparser::HeapType::Any => WasmHeapType::Any,
            wasmparser::HeapType::I31 => WasmHeapType::I31,
            wasmparser::HeapType::None => WasmHeapType::None,

            wasmparser::HeapType::Exn
            | wasmparser::HeapType::NoExn
            | wasmparser::HeapType::NoExtern
            | wasmparser::HeapType::Eq
            | wasmparser::HeapType::Struct
            | wasmparser::HeapType::Array => {
                unimplemented!("unsupported heap type {ty:?}");
            }
        }
    }

    /// Converts the specified type index from a heap type into a canonicalized
    /// heap type.
    fn lookup_heap_type(&self, index: wasmparser::UnpackedIndex) -> WasmHeapType;
}
