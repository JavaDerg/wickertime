use crate::decl_types;
use crate::NumType;

// Good resource: https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/
decl_types! {
    Sign => :enum {
        U,
        S,
    },
    Bits => :enum {
        _64,
        _32,
    },
    Inst => :enum {
        Const(NumType),

        // Unops
        IClz(Bits),
        ICtz(Bits),
        IPopCnt(Bits),

        FAbs(Bits),
        FNeg(Bits),
        FSqrt(Bits),
        FCeil(Bits),
        FFloor(Bits),
        FTrunc(Bits),
        FNearest(Bits),

        // Binops
        IAdd(Bits),
        ISub(Bits),
        IMul(Bits),
        IDiv(Bits, Sign),
        IRem(Bits, Sign),
        IAnd(Bits),
        IOr(Bits),
        IXor(Bits),
        IShl(Bits),
        IShr(Bits, Sign),
        IRotl(Bits),
        IRotr(Bits),

        FAdd(Bits),
        FSub(Bits),
        FMul(Bits),
        FDiv(Bits),
        Fmin(Bits),
        FMax(Bits),
        FCopySign(Bits),

        // itestop
        IEqz(Bits),

        // irelop
        IEq(Bits),
        INe(Bits),
        ILt(Bits, Sign),
        IGt(Bits, Sign),
        ILe(Bits, Sign),
        IGe(Bits, Sign),

        // frelop
        FEq(Bits),
        FNe(Bits),
        FLt(Bits),
        FGt(Bits),
        FLe(Bits),
        FGe(Bits),

        IExtend8S(Bits),
        IExtend16S(Bits),
        I64Extend32S(Bits),
        I32WrapI64(Bits),
        /// Unlike the other Extend operations, they assume that they are extending from T -> T where T maybe i32 or i64
        /// Wasm only has i32 & i64, so bytes will be represented as i32
        /// This will perform I64 -> I32
        I64ExtendI32(Sign),

        Trunc(Bits, Bits, Sign),
        TruncSat(Bits, Bits, Sign),
        F32DemoteF64,
        F64PromoteF32,

        FConvertI(Bits, Bits, Sign),
        IReinterpretF(Bits, Bits),
        FReinterpretI(Bits, Bits),

        // to be done: all the othe ones :>
    },
}
