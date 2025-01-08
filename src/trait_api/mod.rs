/*
MIT License

Copyright (c) 2025 Philipp Schuster

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/
#[macro_use]
mod macros;

/// Common bitwise operations to manipulate the bits in raw integers.
pub trait BitOps: Copy + Sized {
    /// Sets the given bit to `1`.
    ///
    /// The bit position starts at `0`.
    ///
    /// # Parameters
    ///
    /// - `bit`: Bit to set, starting at position `0`.
    #[must_use]
    fn set_bit(self, bit: Self) -> Self;

    /// Clears the given bit by setting it to `0`.
    ///
    /// The bit position starts at `0`.
    ///
    /// # Parameters
    ///
    /// - `bit`: Bit to clear, starting at position `0`.
    #[must_use]
    fn clear_bit(self, bit: Self) -> Self;

    /// Returns whether the given bit is set.
    ///
    /// The bit position starts at `0`.
    ///
    /// # Parameters
    ///
    /// - `bit`: Bit to check, starting at position `0`.
    #[must_use]
    fn is_set(self, bit: Self) -> bool;

    /// Returns the integer value of the given bit (`0` or `1`).
    ///
    /// The bit position starts at `0`.
    ///
    /// # Parameters
    ///
    /// - `bit`: Bit to get, starting at position `0`.
    #[must_use]
    fn get_bit(self, bit: Self) -> Self;

    /// Toggles (flips) the given bit.
    ///
    /// The bit position starts at `0`.
    ///
    /// # Parameters
    ///
    /// - `bit`: Bit to toggle, starting at position `0`.
    #[must_use]
    fn toggle_bit(self, bit: Self) -> Self;

    /// Toggles (flips) the specified contiguous bits.
    ///
    /// # Parameters
    ///
    /// - `bits`: Amount of bits of `value` that are relevant.
    /// - `shift`: Relevant position of bits inside `value`, starting from the
    ///            right/LSB (`0`).
    #[must_use]
    fn toggle_bits(self, bits: Self, shift: Self) -> Self;

    /// Sets the bits of `value` in `self` without clearing already set bits.
    ///
    /// # Parameters
    ///
    /// - `value`: New value/bits to be set in `base`.
    /// - `value_bits`: Amount of bits of `value` that are relevant.
    /// - `value_shift`: Position of `value` inside `self`, starting from the
    ///                  right/LSB (`0`).
    ///
    /// # Panics
    ///
    /// This function panics for overflowing shifts and bit positions that
    /// are outside the range of the underlying type.
    #[must_use]
    fn set_bits(self, value: Self, value_bits: Self, value_shift: Self) -> Self;

    /// Version of [`Self::set_bits`] that applies a list of multiple values to
    /// the base.
    ///
    /// # Parameters
    ///
    /// - `base`: Base value to alter.
    /// - `ops`: Tuple of (`value`, `value_bits`, `value_shift`) where each
    ///   tuple member corresponds to the parameter in [`set_bits`].
    ///
    /// # Panics
    ///
    /// This function panics for overflowing shifts and bit positions that
    /// are outside the range of the underlying type.
    #[must_use]
    fn set_bits_n(
        self,
        ops: &[(
            Self, /* value */
            Self, /* value_bits */
            Self, /* value_shift */
        )],
    ) -> Self;

    /// Like [`Self::set_bits`] but calls [`Self::clear_bits`] beforehand for
    /// the relevant bits.
    ///
    /// # Parameters
    ///
    /// - `value`: New value/bits to be set in `base`.
    /// - `value_bits`: Amount of bits of `value` that are relevant.
    /// - `value_shift`: Position of `value` inside `self`, starting from the
    ///                  right/LSB (`0`).
    ///
    /// # Panics
    ///
    /// This function panics for overflowing shifts and bit positions that
    /// are outside the range of the underlying type.
    #[must_use]
    fn set_bits_exact(self, value: Self, value_bits: Self, value_shift: Self) -> Self;

    /// Combination of [`Self::set_bits_exact`] and [`Self::set_bits_n`].
    ///
    /// # Parameters
    ///
    /// - `base`: Base value to alter.
    /// - `ops`: Tuple of (`value`, `value_bits`, `value_shift`) where each
    ///   tuple member corresponds to the parameter in [`set_bits`].
    ///
    /// # Panics
    ///
    /// This function panics for overflowing shifts and bit positions that
    /// are outside the range of the underlying type.
    #[must_use]
    fn set_bits_exact_n(
        self,
        ops: &[(
            Self, /* value */
            Self, /* value_bits */
            Self, /* value_shift */
        )],
    ) -> Self;

    /// Clears all bits specified in the mask by setting them to `0`.
    ///
    /// # Parameters
    ///
    /// - `clear_mask`: Bitmask with bits to clear.
    #[must_use]
    fn clear_bits(self, clear_mask: Self) -> Self;

    /// Returns the highest bit that is set, if any.
    ///
    /// The bit position starts at `0`.
    #[must_use]
    fn highest_bit(self) -> Option<Self>;

    /// Returns the lowest bit that is set, if any.
    ///
    /// The bit position starts at `0`.
    #[must_use]
    fn lowest_bit(self) -> Option<Self>;

    /// Returns the requested contiguous bits as new integer.
    ///
    /// # Parameters
    ///
    /// - `value_bits`: Amount of bits of `value` that are relevant.
    /// - `value_shift`: Position of `value` inside `self`, starting from the
    ///                  right/LSB (`0`).
    #[must_use]
    fn get_bits(self, value_bits: Self, value_shift: Self) -> Self;

    /// Creates a bitmask (`1`s) with the given amount of contiguous bits.
    ///
    /// # Parameters
    ///
    /// - `bits`: Amount of contiguous bits.
    #[must_use]
    fn create_mask(bits: Self) -> Self;
}

impl_trait!(u8);
impl_trait!(u16);
impl_trait!(u32);
impl_trait!(u64);
impl_trait!(u128);
impl_trait!(usize);
