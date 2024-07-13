# bit_ops

Bitwise operations on primitive integer types, but `no_std` and `const`-compatible.
Provides a collection of typical bit manipulation operations that are primarily
required in low-level development. Unlike other crates that provide tooling to
create sophisticated high-level types with bitfields, the focus of bit_ops is
to work on raw integers. Thus, bit_ops chose a manual and more direct approach.

# Documentation

See <https://docs.rs/bit_ops>.

# Example

<!-- copied from lib.rs -->
```rust
fn main() {
    use bit_ops::bitops_u64;

    /// See specification of the x86 IOAPIC redirection entry for more details.
    mod x86_ioapic {
        pub const VECTOR_BITS: u64 = 8;
        pub const VECTOR_SHIFT: u64 = 0;
        pub const DELIVERY_MODE_BITS: u64 = 3;
        pub const DELIVERY_MODE_SHIFT: u64 = 8;
        pub const DESTINATION_MODE_BITS: u64 = 1;
        pub const DESTINATION_MODE_SHIFT: u64 = 11;
        pub const PIN_POLARITY_BITS: u64 = 1;
        pub const PIN_POLARITY_SHIFT: u64 = 13;
        pub const TRIGGER_MODE_BITS: u64 = 1;
        pub const TRIGGER_MODE_SHIFT: u64 = 15;
        pub const MASKED_BITS: u64 = 1;
        pub const MASKED_SHIFT: u64 = 16;
        pub const DESTINATION_BITS: u64 = 8;
        pub const DESTINATION_SHIFT: u64 = 56;
    }
    // example properties for an x86 IOAPIC redirection entry
    let vector = 7;
    let delivery_mode = 0b111; // ExtInt
    let destination_mode = 0; // physical
    let pin_polarity = 1; // low-active
    let trigger_mode = 1; // level-triggered
    let masked = 1;
    let destination = 13;
    //!
    use x86_ioapic::*;
    //!
    let redirection_entry = bitops_u64::set_bits_exact_n(
        0,
        &[
            (vector, VECTOR_BITS, VECTOR_SHIFT),
            (delivery_mode, DELIVERY_MODE_BITS, DELIVERY_MODE_SHIFT),
            (destination_mode, DESTINATION_MODE_BITS, DESTINATION_MODE_SHIFT),
            (pin_polarity, PIN_POLARITY_BITS, PIN_POLARITY_SHIFT),
            (trigger_mode, TRIGGER_MODE_BITS, TRIGGER_MODE_SHIFT),
            (masked, MASKED_BITS, MASKED_SHIFT),
            (destination, DESTINATION_BITS, DESTINATION_SHIFT),
        ],
    );
    assert_eq!(redirection_entry, 0xd0000000001a707);
}
```

## MSRV

1.57.0 stable
