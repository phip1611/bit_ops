# Unreleased

# v0.1.9, v0.1.10  - 2024-07-30

- Removed `create_shifted_mask` as `create_mask() << 5` is a trivial replacement
- Renamed `toggle` to `toggle_bit`
- Added `toggle_bits`

# v0.1.8 - 2024-07-15

- Added `get_bit` function that is like `is_set` but returns an integer instead
  of a boolean

# v0.1.1 - v0.1.7 - 2024-07-13

- Doc fixes
- Trait API: add `#[inline]` and `#[must_use]` to each function impl

# v0.1.0 - 2024-07-13

Initial release.
