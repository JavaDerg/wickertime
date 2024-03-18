use crate::decl_types;
use crate::NumType;
use crate::VecType;

// Good resource: https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/
// https://pengowray.github.io/wasm-ops/
decl_types! {
    Sign => :enum {
        U,
        S,
    },
    Bits => :enum {
        _64,
        _32,
    },
    IShape => :enum {
        I8X16,
        I16X8,
        I32X4,
        I64X2,
    },
    FShape => :enum {
        F32X4,
        F64X2,
    },
    Shape => :enum {
        I(IShape),
        F(FShape),
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

        // Vector instrunctions
        VConst(VecType),
        VNot,
        VAnd,
        VAndNot,
        VOr,
        VXor,
        VBitSelect,
        VAnyTrue,

        VI8X16Shuffle([u8; 16]),
        VI8X16Swizzle,
        VSplat(Shape),
        VI8X16ExtractLane(Bits, u8),
        VI16X8ExtractLane(Bits, u8),
        VI32X4ExtractLine(u8),
        VI64X2ExtractLine(u8),
        VExtractLane(FShape, u8),
        VReplaceLane(Shape, u8),
        VI8X16Eq,
        VI8X16Ne,
        VI8X16Lt(Sign),
        VI8X16Gt(Sign),
        VI8X16Le(Sign),
        VI8X16Ge(Sign),
        VI16X8Eq,
        VI16X8Ne,
        VI16X8Lt(Sign),
        VI16X8Gt(Sign),
        VI16X8Le(Sign),
        VI16X8Ge(Sign),
        VI32X4Eq,
        VI32X4Ne,
        VI32X4Lt(Sign),
        VI32X4Gt(Sign),
        VI32X4Le(Sign),
        VI32X4Ge(Sign),
        VI64X2Eq,
        VI64X2Ne,
        VI64X2Lt,
        VI64X2Gt,
        VI64X2Le,
        VI64X2Ge,
        VFEq(FShape),
        VFNe(FShape),
        VFLt(FShape),
        VFGt(FShape),
        VFLe(FShape),
        VFGe(FShape),
        VIAbs(IShape),
        VINeg(IShape),
        VI8X16PopCnt,
        VI16X8Q15MulRSatS,
        VI32X4DotI16X8S,
        VFAbs(FShape),
        VFNeg(FShape),
        VFSqrt(FShape),
        VFCeil(FShape),
        VFFloor(FShape),
        VFTrunc(FShape),
        VFNearest(FShape),
        VIAllTrue(IShape),
        VIBitMask(IShape),

        // TODO: More stuff
    },
}
