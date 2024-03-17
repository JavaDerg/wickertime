#![no_std]

mod inst;

use alloc::vec::Vec;
pub use inst::Inst;

extern crate alloc;

macro_rules! decl_types {
    {
        $(
            $type_name:ident =>
                $($type_path:ty)?
                $(:enum {$($type_block_enum:tt)*})?
                $(:($($type_tuple:ty),*$(,)?))?
                $({$($type_block:tt)*})?
        ),*
        $(,)?
    } => {
        $(
            decl_types!{
                _ $type_name =>
                    $(wrap, $type_path)?
                    $(tuple, ($($type_tuple),*))?
                    $(block, {$($type_block)*})?
                    $(enum {$($type_block_enum)*})?
            }
        )*
    };
    {
        _ $type_name:ident => wrap, $type:path
    } => {
        #[derive(Clone, Debug, PartialEq)]
        pub struct $type_name (pub $type);

        impl ::core::ops::Deref for $type_name {
            type Target = $type;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl ::core::ops::DerefMut for $type_name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
    {
        _ $type_name:ident => tuple, ($($type:ty),*)
    } => {
        #[derive(Clone, Debug, PartialEq)]
        pub struct $type_name ($(pub $type),*);
    };
    {
        _ $type_name:ident => block, {$(
            $name:ident: $type:ty
        ),*$(,)?}
    } => {
        #[derive(Clone, Debug, PartialEq)]
        pub struct $type_name {$(
            pub $name: $type,
        )*}
    };
    {
        _ $type_name:ident => enum {$($tt:tt)*}
    } => {
        #[derive(Clone, Debug, PartialEq)]
        pub enum $type_name {$(
            $tt
        )*}
    };
}
pub(crate) use decl_types;

decl_types! {
    TypeId => u32,
    FuncId => u32,
    TableId => u32,
    MemId => u32,
    GlobalId => u32,
    ElemId => u32,
    DataId => u32,
    LocalId => u32,
    LabelId => u32,
    Module => {
        types: Vec<FuncType>,
        funcs: Vec<Func>,
    },
    Func => {
        r#type: TypeId,
        locals: Vec<ValType>,
        body: Expr,
    },
    Expr => Vec<Inst>,
    FuncType => {
        from: ResultType,
        to: ResultType,
    },
    ResultType => Vec<ValType>,
    ValType => :enum {
        Num(NumType),
        Vec(VecType),
        Ref(RefType),
    },
    RefType => :enum {
        Func(u32),
        Extern(u32),
    },
    Limits => {
        min: u32,
        max: Option<u32>,
    },
    VecType => i128,
    NumType => :enum {
        I(NumIType),
        F(NumFType),
    },
    NumIType => :enum {
        I32(i32),
        I64(i64),
    },
    NumFType => :enum {
        F32(f32),
        F64(f64),
    },
    MemType => Limits,
    TableType => :(Limits, RefType),
    Table => {
        r#type: TableType,
    },
    Mem => {
        r#type: MemType,
    },
    Mut => :enum {
        Const,
        Var,
    },
    Global => {
        r#type: GlobalType,
        init: Expr,
    },
    GlobalType => :(Mut, ValType),
    Elem => {
        r#type: RefType,
        init: Vec<Expr>,
        mode: ElemMode,
    },
    ExternType => :enum {
        Func(FuncType),
        Table(TableType),
        Mem(MemType),
        Global(GlobalType),
    },
}
