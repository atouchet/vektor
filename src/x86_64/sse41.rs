#![allow(unused_imports)]
use crate::myarch::*;
use crate::simd::*;

/// Extract an 64-bit integer from `a` selected with `imm8`
#[inline]
#[target_feature(enable = "sse4.1")]
// TODO: Add test for Windows
#[cfg_attr(test, assert_instr(pextrq, imm8 = 1))]
#[rustc_args_required_const(1)]
pub unsafe fn _mm_extract_epi64(a: i64x2, imm8: i32) -> i64 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_extract_epi64(::mem::transmute(a), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm8, call))
}

/// Return a copy of `a` with the 64-bit integer from `i` inserted at a
/// location specified by `imm8`.
#[inline]
#[target_feature(enable = "sse4.1")]
#[cfg_attr(test, assert_instr(pinsrq, imm8 = 0))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm_insert_epi64(a: i64x2, i: i64, imm8: i32) -> i64x2 {

    macro_rules! call {
        ($imm8:expr) => {
            ::myarch::_mm_insert_epi64(::mem::transmute(a), ::mem::transmute(i), $imm8)
        };
    }

    ::mem::transmute(constify_imm8!(imm8, call))
}

