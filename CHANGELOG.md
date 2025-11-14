# Changelog of `bit_ops`

## Unreleased


## v0.2.2 - 2025-11-13

- Improved documentation


## v0.2.1 - 2025-11-13

- Improved implementation of `BitmapIter` significantly (50% performance gain)


## v0.2.0 - 2025-11-13

- **Breaking**: Rust edition 2024, MSRV is now 1.85.1
- Added `BitsIter` and `BitmapIter` to iterate set bits in unsigned integers
  and bitmaps.
- Documentation fixes


## v0.1.16 - 2025-06-01

- Added `set_bit_exact` which allows setting a bit to an exact value.
  The existing `set_bit` can only set a bit to `1`.


## v0.1.15 - 2025-01-07

- Improved documentation


## v0.1.14 - 2025-01-07

- Improved documentation


## v0.1.13 - 2024-08-24

- Metadata changes


## v0.1.12 - 2024-08-07

- Added the `#![no_std]` attribute to the crate. During early development, I
  probably removed this for easier debugging and then this embarrassing mistake
  happened.. 🤦😅  Now the crate is really `no_std` - as promised!


## v0.1.11 - 2024-07-30

- Functions `set_bits_n` and `set_bits_exact_n` are now `const` compatible


## v0.1.9, v0.1.10 - 2024-07-30

- Removed `create_shifted_mask` as `create_mask() << 5` is a trivial replacement
- Renamed `toggle` to `toggle_bit`
- Added `toggle_bits`


## v0.1.8 - 2024-07-15

- Added `get_bit` function that is like `is_set` but returns an integer instead
  of a boolean


## v0.1.1 - v0.1.7 - 2024-07-13

- Doc fixes
- Trait API: add `#[inline]` and `#[must_use]` to each function impl


## v0.1.0 - 2024-07-13

Initial release.
