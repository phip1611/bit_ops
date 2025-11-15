# bit_ops

Common bit-oriented operations on primitive integer types with a focus on
`no_std` and `const` compatibility. Unlike other crates that provide tooling to
create sophisticated high-level types with bitfields, the focus of `bit_ops` is
on raw primitive integer types.

# Documentation

See <https://docs.rs/bit_ops>.

# Example

<!-- copied from lib.rs -->
```rust
fn main() {
    // PREREQUISITES: Some Definitions

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

    use x86_ioapic::*;

    // ACTUAL LIBRARY USAGE BEGINS HERE

    let redirection_entry = bit_ops::bitops_u64::set_bits_exact_n(
        0,
        &[
            (7, VECTOR_BITS, VECTOR_SHIFT),
            (0b111 /* ExtInt */, DELIVERY_MODE_BITS, DELIVERY_MODE_SHIFT),
            (0 /* physical */, DESTINATION_MODE_BITS, DESTINATION_MODE_SHIFT),
            (1 /* low-active */, PIN_POLARITY_BITS, PIN_POLARITY_SHIFT),
            (1 /* level-triggered */, TRIGGER_MODE_BITS, TRIGGER_MODE_SHIFT),
            (1 /* masked */, MASKED_BITS, MASKED_SHIFT),
            (13 /* APIC ID */, DESTINATION_BITS, DESTINATION_SHIFT),
        ],
    );
    assert_eq!(redirection_entry, 0xd0000000001a707);
}
```

## MSRV

<!-- Keep in sync with lib.rs and Cargo.toml!  -->
1.85 stable

## Benchmarks

Using `$ cargo bench`, you can execute benchmarks.

### BitmapIter

The `BitmapIter` is an iterator over set bits in (large) bitmaps, i.e.,
collection of unsigned integers.

| % 1 Bits | bit_ops::BitmapIter | [Julian's Version][jv] | BitVec   |
|----------|---------------------|------------------------|----------|
| 0.0      | 2,7 µs              | 2.8 µs                 | 3.4 µs   |
| 0.1      | 27,0 µs             | 38,9 µs                | 69,9 µs  |
| 1.0      | 27,5 µs             | 38,6 µs                | 71,0 µs  |
| 5.0      | 60,8 µs             | 93,6 µs                | 251,5 µs |
| 99.9     | 301,6 µs            | 933,3 µs               | 4,0 ms   |

In this benchmark, a `&[u64; 10000]` slice was iterated which corresponds to 78
KiB. `BitmapIter` from this crate clearly outperforms the other solutions.
The data was collected on a i5-10600k but shows very similar behavior on modern
x86 microarchitectures.

[jv]: https://github.com/cyberus-technology/cloud-hypervisor/pull/36/files

## License

MIT License. See [LICENSE](./LICENSE) file.
