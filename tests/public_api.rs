//! This is just a simple test ensuring the public API works as expected.

use bit_ops::{BitOps, bitops_u64};

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
fn const_compatible() {
    const fn compiles() {
        use bitops_u64::*;
        let _ = set_bit(0, 0);
        let _ = set_bit_exact(0, 0, false);
        let _ = clear_bit(0, 0);
        let _ = is_set(0, 0);
        let _ = toggle_bit(0, 0);
        let _ = toggle_bits(0, 0, 0);
        let _ = set_bits(0, 0, 0, 0);
        let _ = set_bits_n(0, &[]);
        let _ = set_bits_exact(0, 0, 0, 0);
        let _ = set_bits_exact_n(0, &[]);
        let _ = clear_bits(0, 0);
        let _ = highest_bit(0);
        let _ = lowest_bit(0);
        let _ = get_bits(0, 0, 0);
        let _ = create_mask(0);
    }
    compiles();
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

    let raw = 0b10101010_u8
        .set_bit_exact(0, true)
        .set_bit_exact(1, false)
        .set_bit_exact(7, false);
    assert_eq!(raw, 0b00101001);
}
