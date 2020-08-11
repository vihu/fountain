#![cfg_attr(
    all(
        feature = "arm-neon",
        any(target_arch = "arm", target_arch = "aarch64")
    ),
    feature(arm_target_feature),
    feature(stdsimd)
)]

// Computing the XOR of two byte slices, `lhs` & `rhs`.
// `lhs` is mutated in-place with the result
pub fn xor_bytes(lhs: &mut [u8], rhs: &[u8]) {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        if is_x86_feature_detected!("avx2") {
            return unsafe { xor_bytes_avx2(lhs, rhs) };
        } else if is_x86_feature_detected!("sse2") {
            return unsafe { xor_bytes_sse2(lhs, rhs) };
        }
    }

    #[cfg(all(
        feature = "arm-neon",
        any(target_arch = "arm", target_arch = "aarch64")
    ))]
    {
        if is_arm_feature_detected!("neon") {
            return unsafe { xor_bytes_neon(lhs, rhs) };
        }
    }

    xor_bytes_fallback(lhs, rhs);
}

#[inline(always)]
fn xor_bytes_fallback(lhs: &mut [u8], rhs: &[u8]) {
    for (l, r) in lhs.iter_mut().zip(rhs) {
        *l ^= *r;
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "avx2")]
unsafe fn xor_bytes_avx2(lhs: &mut [u8], rhs: &[u8]) {
    xor_bytes_fallback(lhs, rhs);
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "sse2")]
unsafe fn xor_bytes_sse2(lhs: &mut [u8], rhs: &[u8]) {
    xor_bytes_fallback(lhs, rhs);
}

#[cfg(all(
    feature = "arm-neon",
    any(target_arch = "arm", target_arch = "aarch64")
))]
#[target_feature(enable = "neon")]
unsafe fn xor_bytes_neon(lhs: &mut [u8], rhs: &[u8]) {
    xor_bytes_fallback(lhs, rhs);
}
