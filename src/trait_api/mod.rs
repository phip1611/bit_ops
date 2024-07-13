#[macro_use]
mod macros;

/// Common bitwise operations to manipulate the bits in raw integers.
pub trait BitOps: Copy + Sized {
    /// Sets the given bit to `1`.
    ///
    /// The bit position starts at `0`.
    #[must_use]
    fn set_bit(self, bit: Self) -> Self;

    /// Clears the given bit by setting it to `0`.
    ///
    /// The bit position starts at `0`.
    #[must_use]
    fn clear_bit(self, bit: Self) -> Self;

    /// Returns whether the given bit is set.
    ///
    /// The bit position starts at `0`.
    #[must_use]
    fn is_set(self, bit: Self) -> bool;

    /// Toggles (flips) the given bit.
    ///
    /// The bit position starts at `0`.
    #[must_use]
    fn toggle(self, bit: Self) -> Self;

    /// Sets the bits of `value` in `base` without clearing already set bits.
    ///
    /// # Parameters
    /// - `value`: New value/bits to be set in `base`, but unshifted.
    /// - `value_bits`: Amount of bits of `value` that are relevant, starting from the right.
    /// - `value_shit`: Position of `value` inside `self`.
    #[must_use]
    fn set_bits(self, value: Self, value_bits: Self, value_shift: Self) -> Self;

    /// Version of [`Self::set_bits`] that applies a list of multiple values to
    /// the base.
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
    #[must_use]
    fn set_bits_exact(self, value: Self, value_bits: Self, value_shift: Self) -> Self;

    /// Combination of [`Self::set_bits_exact`] and [`Self::set_bits_n`].
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
    /// - `clear_mask`: Bitmask with bits to clear.
    #[must_use]
    fn clear_bits(self, clear_mask: Self) -> Self;

    /// Returns the highest bit that is set.
    ///
    /// The bit position starts at `0`.
    #[must_use]
    fn highest_bit(self) -> Option<Self>;

    /// Returns the lowest bit that is set.
    ///
    /// The bit position starts at `0`.
    #[must_use]
    fn lowest_bit(self) -> Option<Self>;

    /// Get the requested bits as new integer.
    ///
    /// # Parameters
    /// - `value_bits`: Amount of bits of `value` that are relevant, starting
    ///                 from the right.
    /// - `value_shit`: Position of `value` inside `self`.
    #[must_use]
    fn get_bits(self, value_bits: Self, value_shift: Self) -> Self;

    /// Creates a bitmask (`1`s) with the given amount of bits.
    #[must_use]
    fn create_mask(bits: Self) -> Self;

    /// Like [`Self::create_mask`] but shifts the mask.
    #[must_use]
    fn create_shifted_mask(bits: Self, lshift: Self) -> Self;
}

impl_trait!(u8);
impl_trait!(u16);
impl_trait!(u32);
impl_trait!(u64);
impl_trait!(usize);

mod tests {}
