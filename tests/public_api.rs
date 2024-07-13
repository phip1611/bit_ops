//! This is just a simple test ensuring the public API works as expected.

use bit_ops::{bitops_u64, BitOps};

#[test]
fn test_public_function_api() {
    let raw = bitops_u64::set_bits_exact(0, 0b100, 3, 0);
    let raw = bitops_u64::set_bits_exact(raw, 0xf, 4, 60);
    assert_eq!(raw, 0xf000_0000_0000_0004);
    let raw = bitops_u64::set_bits(raw, 0b1001, 4, 60);
    assert_eq!(raw, 0xf000_0000_0000_0004); // unchanged
    let raw = bitops_u64::set_bits_exact(raw, 0b1001, 4, 60);
    assert_eq!(raw, 0x9000_0000_0000_0004);
}

#[test]
fn test_public_trait_api() {
    let raw = 0_u64.set_bits_exact(0b100, 3, 0);
    let raw = raw.set_bits_exact(0xf, 4, 60);
    assert_eq!(raw, 0xf000_0000_0000_0004);
    let raw = raw.set_bits(0b1001, 4, 60);
    assert_eq!(raw, 0xf000_0000_0000_0004); // unchanged
    let raw = raw.set_bits_exact(0b1001, 4, 60);
    assert_eq!(raw, 0x9000_0000_0000_0004);

    let raw = 0_u64.set_bit(0).set_bit(1).set_bit(4);
    assert_eq!(raw, 0b10011);
}
