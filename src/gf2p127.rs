//#![allow(non_snake_case)]

extern crate std;
extern crate core;

use self::core::ops::BitXor;

#[unstable(feature = "core")]
#[simd]
#[derive(PartialEq, Eq, Copy, Debug)]
#[repr(C)]
struct Gf2p127(pub i64, pub i64);

#[inline(always)]
pub fn zero() -> Gf2p127 {
    return Gf2p127(0, 0);
}

#[inline(always)]
pub fn from_i64(a: i64) -> Gf2p127 {
    return Gf2p127(0, a);
}

#[inline(always)]
pub fn add(a: Gf2p127, b: Gf2p127) -> Gf2p127 {
    return Gf2p127(a.0.bitxor(b.0), a.1.bitxor(b.1));
}

#[inline(always)]
pub fn mul_00(_: Gf2p127) -> Gf2p127 {
    return zero();
}

#[inline(always)]
pub fn mul_01(a: Gf2p127) -> Gf2p127 {
    return a;
}

#[inline(always)]
pub fn mul_10(a: Gf2p127) -> Gf2p127 {
    unsafe {
        let sl = slli_epi64(a, 1);
        let one = srli_epi64(Gf2p127(a.1, sl.0), 63);
        let lo = Gf2p127(one.1, one.1);
        let x127x63 = slli_epi64(lo, 63);
        return add(add(sl, one), x127x63);
    }
}

#[test]
fn mul_10_test() {
  assert_eq!(hex(mul_10(Gf2p127(0, 0))), "00000000000000000000000000000000");
  assert_eq!(hex(mul_10(Gf2p127(0, 1))), "00000000000000000000000000000002");
  assert_eq!(hex(mul_10(Gf2p127(0, 1 << 63))), "00000000000000010000000000000000");
  assert_eq!(hex(mul_10(Gf2p127(1 << 62, 0))), "00000000000000008000000000000001");
  assert_eq!(hex(mul_10(Gf2p127(1 << 62, 1 << 63))), "00000000000000018000000000000001");
}

#[inline(always)]
pub fn mul_11(a: Gf2p127) -> Gf2p127 {
    return add(mul_01(a), mul_10(a));
}

pub fn hex(a: Gf2p127) -> String {
  return std::fmt::format(format_args!("{:016x}{:016x}", a.0, a.1));
}

#[test]
fn hex_test() {
  assert_eq!(hex(zero()), "00000000000000000000000000000000");
  assert_eq!(hex(from_i64(1)), "00000000000000000000000000000001");
}

extern {
    #[link_name = "llvm.x86.sse2.pslli.q"]
    fn slli_epi64(a: Gf2p127, b: i32) -> Gf2p127;
    #[link_name = "llvm.x86.sse2.psrli.q"]
    fn srli_epi64(a: Gf2p127, b: i32) -> Gf2p127;
}
