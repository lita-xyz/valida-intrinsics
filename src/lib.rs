#![feature(link_llvm_intrinsics)]

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(C, align(4))]
pub struct Secp256k1Point {
    pub x: [u8; 32],
    pub y: [u8; 32],
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(C, align(4))]
pub struct Secp256k1Scalar {
    pub value: [u8; 32],
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C, align(4))]
pub struct Secp256k1Comb {
    pub point: Secp256k1Point,
    pub scalar: Secp256k1Scalar,
}

extern "C" {
    #[link_name = "llvm.valida.muls.secp256k1"]
    fn intrinsic_muls_secp256k1(arg1: *const u8, arg2: *mut u8);
}

extern "C" {
    #[link_name = "llvm.valida.comb.secp256k1"]
    fn intrinsic_comb_secp256k1(arg1: *const u8, arg2: *mut u8);
}

extern "C" {
    #[link_name = "llvm.valida.smul.secp256k1"]
    fn intrinsic_smul_secp256k1(arg1: *const u8, arg2: *mut u8);
}

extern "C" {
    #[link_name = "llvm.valida.sinv.secp256k1"]
    fn intrinsic_sinv_secp256k1(arg: *mut u8);
}

#[inline(always)]
pub fn muls_secp256k1(x: &Secp256k1Scalar, y: &mut Secp256k1Scalar) {
    let x: *const u8 = x as *const Secp256k1Scalar as *const u8;
    let y: *mut u8 = y as *mut Secp256k1Scalar as *mut u8;

    unsafe { intrinsic_muls_secp256k1(x, y) }
}

#[inline(always)]
pub fn comb_secp256k1(x: &Secp256k1Comb, y: &mut Secp256k1Comb) {
    let x: *const u8 = x as *const Secp256k1Comb as *const u8;
    let y: *mut u8 = y as *mut Secp256k1Comb as *mut u8;

    unsafe { intrinsic_comb_secp256k1(x, y) }
}

#[inline(always)]
pub fn sinv_secp256k1(x: &mut Secp256k1Scalar) {
    let x: *mut u8 = x as *mut Secp256k1Scalar as *mut u8;

    unsafe { intrinsic_sinv_secp256k1(x) }
}

#[inline(always)]
pub fn smul_secp256k1(x: &Secp256k1Scalar, y: &mut Secp256k1Point) {
    let x: *const u8 = x as *const Secp256k1Scalar as *const u8;
    let y: *mut u8 = y as *mut Secp256k1Point as *mut u8;

    unsafe { intrinsic_smul_secp256k1(x, y) }
}
