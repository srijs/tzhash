//#![allow(non_snake_case)]

extern crate core;
extern crate simdty;

use self::core::ops::BitXor;

type Gf2p127 = simdty::i64x2;

fn zero() -> Gf2p127 {
    return simdty::i64x2(0, 0);
}

fn from_int(a: i64) -> Gf2p127 {
    return simdty::i64x2(0, a);
}

fn add(a: Gf2p127, b: Gf2p127) -> Gf2p127 {
    return simdty::i64x2(a.0.bitxor(b.0), a.1.bitxor(b.1));
}

fn mul_00(_: Gf2p127) -> Gf2p127 {
    return zero();
}

fn mul_01(a: Gf2p127) -> Gf2p127 {
    return a;
}

fn mul_10(a: Gf2p127) -> Gf2p127 {
    unsafe {
        let sl = slli_epi64(a, 1);
        let one = srli_epi64(alignr_epi8(a, sl, 8), 63);
        return sl;
    }
}

extern {
    #[link_name = "llvm.x86.sse2.pslli.q"]
    fn slli_epi64(a: simdty::i64x2, b: i32) -> simdty::i64x2;
    #[link_name = "llvm.x86.sse2.psrli.q"]
    fn srli_epi64(a: simdty::i64x2, b: i32) -> simdty::i64x2;
    #[link_name = "llvm.x86.ssse3.palign.r.128"]
    fn alignr_epi8(a: simdty::i64x2, b: simdty::i64x2, c: i32) -> simdty::i64x2;
}
