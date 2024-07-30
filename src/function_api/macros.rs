/*
MIT License

Copyright (c) 2024 Philipp Schuster

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
/// Implements the relevant bit operations for the specified primitive type.
///
/// Note that the bit positions start at `0`. The highest `bit` position thus
/// is `BITS - 1`.
macro_rules! impl_bit_ops {
    ($primitive_ty:ty) => {
        /// Amount of bits for that type.
        const BIT_COUNT: $primitive_ty = <$primitive_ty>::BITS as $primitive_ty;

        #[track_caller]
        const fn assert_in_range(n: $primitive_ty, inclusive: bool) {
            if inclusive {
                assert!(
                    n <= BIT_COUNT,
                    "bit position starts at 0 and should be less than or equal to `bitcount(type)`"
                );
            } else {
                assert!(
                    n < BIT_COUNT,
                    "bit position starts at 0 and should be less than `bitcount(type)`"
                );
            }
        }

        /// Sets the given bit to `1`.
        ///
        /// The bit position starts at `0`.
        ///
        /// # Example
        ///
        /// ```rust
        #[doc = concat!("use bit_ops::bitops_", stringify!($primitive_ty), "::set_bit;")]
        ///
        /// let raw = set_bit(0, 7);
        /// assert_eq!(raw, 0b1000_0000);
        /// ```
        ///
        /// # Panics
        /// This function panics for bit positions that are outside the range of
        /// the underlying type.
        #[must_use]
        #[inline]
        pub const fn set_bit(val: $primitive_ty, bit: $primitive_ty) -> $primitive_ty {
            assert_in_range(bit, false);
            val | (1 << bit)
        }

        /// Clears the given bit by setting it to `0`.
        ///
        /// The bit position starts at `0`.
        ///
        /// # Example
        ///
        /// ```rust
        #[doc = concat!("use bit_ops::bitops_", stringify!($primitive_ty), "::clear_bit;")]
        ///
        /// let raw = clear_bit(0b1000_0000, 7);
        /// assert_eq!(raw, 0);
        /// ```
        ///
        /// # Panics
        ///
        /// This function panics for bit positions that are outside the range of
        /// the underlying type.
        #[must_use]
        #[inline]
        pub const fn clear_bit(val: $primitive_ty, bit: $primitive_ty) -> $primitive_ty {
            assert_in_range(bit, false);
            let negative_mask = !(1 << bit);
            val & negative_mask
        }

        /// Returns whether the given bit is set.
        ///
        /// The bit position starts at `0`.
        ///
        /// # Example
        ///
        /// ```rust
        #[doc = concat!("use bit_ops::bitops_", stringify!($primitive_ty), "::is_set;")]
        ///
        /// assert!(!is_set(0, 0));
        /// assert!(!is_set(2, 0));
        /// assert!(is_set(2, 1));
        /// ```
        ///
        /// # Panics
        /// This function panics for bit positions that are outside the range of
        /// the underlying type.
        #[must_use]
        #[inline]
        pub const fn is_set(val: $primitive_ty, bit: $primitive_ty) -> bool {
            get_bit(val, bit) == 1
        }

        /// Returns the integer value of the given bit (`0` or `1`).
        ///
        /// The bit position starts at `0`.
        ///
        /// # Example
        ///
        /// ```rust
        #[doc = concat!("use bit_ops::bitops_", stringify!($primitive_ty), "::get_bit;")]
        ///
        /// assert_eq!(get_bit(0, 0), 0);
        /// assert_eq!(get_bit(2, 0), 0);
        /// assert_eq!(get_bit(2, 1), 1);
        /// ```
        ///
        /// # Panics
        /// This function panics for bit positions that are outside the range of
        /// the underlying type.
        #[must_use]
        #[inline]
        pub const fn get_bit(val: $primitive_ty, bit: $primitive_ty) -> $primitive_ty {
            assert_in_range(bit, false);
            (val >> bit) & 1
        }

        /// Toggles (flips) the given bit.
        ///
        /// The bit position starts at `0`.
        ///
        /// # Example
        ///
        /// ```rust
        #[doc = concat!("use bit_ops::bitops_", stringify!($primitive_ty), "::toggle_bit;")]
        ///
        /// assert_eq!(toggle_bit(0, 0), 1);
        /// assert_eq!(toggle_bit(0, 1), 2);
        /// assert_eq!(toggle_bit(0, 2), 4);
        /// assert_eq!(toggle_bit(0, 7), 128);
        /// ```
        ///
        /// # Panics
        /// This function panics for bit positions that are outside the range of
        /// the underlying type.
        #[must_use]
        #[inline]
        pub const fn toggle_bit(val: $primitive_ty, bit: $primitive_ty) -> $primitive_ty {
            toggle_bits(val, 1, bit)
        }

        /// Toggles (flips) the specified contiguous bits.
        ///
        /// # Parameters
        ///
        /// - `value`: Base value to alter.
        /// - `bits`: Amount of bits of `value` that are relevant, starting from the right.
        /// - `shift`: Relevant position of bits inside `value`, starting from the right.
        ///
        /// # Example
        ///
        /// ```rust
        #[doc = concat!("use bit_ops::bitops_", stringify!($primitive_ty), "::toggle_bits;")]
        ///
        /// assert_eq!(toggle_bits(0, 1, 0), 1);
        /// assert_eq!(toggle_bits(0, 1, 1), 2);
        /// assert_eq!(toggle_bits(0, 1, 2), 4);
        /// assert_eq!(toggle_bits(0, 1, 7), 128);
        /// assert_eq!(toggle_bits(1, 2, 1), 0b111);
        /// assert_eq!(toggle_bits(0b1000_0100, 4, 2), 0b1011_1000);
        /// ```
        ///
        /// # Panics
        /// This function panics for overflowing shifts and bit positions that
        /// are outside the range of the underlying type.
        #[must_use]
        #[inline]
        pub const fn toggle_bits(
            value: $primitive_ty,
            bits: $primitive_ty,
            shift: $primitive_ty,
        ) -> $primitive_ty {
            assert_in_range(bits, true);
            assert_in_range(shift, true);
            let mask = create_mask(bits) << shift;
            value ^ mask
        }

        /// Sets the bits of `value` in `base` without clearing already set
        /// bits.
        ///
        /// # Parameters
        ///
        /// - `base`: Base value to set bits in.
        /// - `value`: New value/bits to be set in `base`, but unshifted.
        /// - `value_bits`: Amount of bits of `value` that are relevant, starting from the right.
        /// - `value_shit`: Position of `value` inside `base`, starting from the right.
        ///
        /// # Example
        ///
        /// ```rust
        #[doc = concat!("use bit_ops::bitops_", stringify!($primitive_ty), "::set_bits;")]
        ///
        /// // props of a fictional interrupt controller
        /// let delivery_mode = 0b111;
        /// let delivery_mode_bits = 3;
        /// let delivery_mode_shift = 3;
        /// assert_eq!(
        ///     set_bits(
        ///         0,
        ///         delivery_mode,
        ///         delivery_mode_bits,
        ///         delivery_mode_shift,
        ///     ),
        ///     0b11_1000
        /// );
        /// ```
        ///
        /// # Panics
        /// This function panics for overflowing shifts and bit positions that
        /// are outside the range of the underlying type.
        #[must_use]
        #[inline]
        pub const fn set_bits(
            base: $primitive_ty,
            value: $primitive_ty,       /* unshifted new bits */
            value_bits: $primitive_ty,  /* bits of value to use */
            value_shift: $primitive_ty, /* bits to left-shift value before updating base  */
        ) -> $primitive_ty {
            assert_in_range(value_bits, true);
            assert_in_range(value_shift, true);
            let value_mask = create_mask(value_bits);
            let value = value & value_mask;
            base | (value << value_shift)
        }

        /// Version of [`set_bits`] that applies a list of multiple values to
        /// the base.
        ///
        /// This function, unlike the others, is not `const` yet due to the
        /// inner loop.
        ///
        /// # Parameters
        ///
        /// - `base`: Base value to set bits in.
        /// - `ops`: Tuple of (`value`, `value_bits`, `value_shift`)
        ///
        /// # Example
        ///
        /// ```rust
        #[doc = concat!("use bit_ops::bitops_", stringify!($primitive_ty), "::set_bits_n;")]
        ///
        /// // props of a fictional interrupt controller
        /// let vector = 0b1_1101;
        /// let vector_bits = 5;
        /// let vector_shift = 0;
        /// let delivery_mode = 0b111;
        /// let delivery_mode_bits = 3;
        /// let delivery_mode_shift = 5;
        /// assert_eq!(
        ///     set_bits_n(
        ///         0,
        ///         &[
        ///             (vector, vector_bits, vector_shift),
        ///             (delivery_mode, delivery_mode_bits, delivery_mode_shift),
        ///         ],
        ///     ),
        ///     0b1111_1101
        /// );
        /// ```
        ///
        /// # Panics
        /// This function panics for overflowing shifts and bit positions that
        /// are outside the range of the underlying type.
        #[must_use]
        #[inline]
        // TODO make const as soon as for-loops can be in const fn
        pub fn set_bits_n(
            base: $primitive_ty,
            ops: &[(
                $primitive_ty, /* value */
                $primitive_ty, /* value_bits */
                $primitive_ty, /* value_shift */
            )],
        ) -> $primitive_ty {
            let mut base = base;
            for op in ops {
                base = set_bits(base, op.0, op.1, op.2);
            }
            base
        }

        /// Like [`set_bits`] but calls [`clear_bits`] beforehand for
        /// the relevant bits.
        #[must_use]
        #[inline]
        pub const fn set_bits_exact(
            base: $primitive_ty,
            value: $primitive_ty,
            value_bits: $primitive_ty,
            value_shift: $primitive_ty,
        ) -> $primitive_ty {
            let clear_mask = create_mask(value_bits) << value_shift;
            let base = clear_bits(base, clear_mask);
            set_bits(base, value, value_bits, value_shift)
        }

        /// Combination of [`set_bits_exact`] and [`set_bits_n`].
        ///
        /// This function, unlike the others, is not `const` yet due to the
        /// inner loop.
        #[must_use]
        #[inline]
        // TODO make const as soon as for-loops can be in const fn
        pub fn set_bits_exact_n(
            base: $primitive_ty,
            ops: &[(
                $primitive_ty, /* value */
                $primitive_ty, /* value_bits */
                $primitive_ty, /* value_shift */
            )],
        ) -> $primitive_ty {
            let mut base = base;
            for op in ops {
                base = set_bits_exact(base, op.0, op.1, op.2);
            }
            base
        }

        /// Clears all bits specified in the mask by setting them to `0`.
        ///
        /// # Parameters
        ///
        /// - `base`: Base value to set bits in.
        /// - `clear_mask`: Bitmask with bits to clear.
        ///
        /// # Example
        ///
        /// ```rust
        #[doc = concat!("use bit_ops::bitops_", stringify!($primitive_ty), "::clear_bits;")]
        ///
        /// assert_eq!(clear_bits(0b111, 0b101), 0b10);
        /// ```
        #[must_use]
        #[inline]
        pub const fn clear_bits(base: $primitive_ty, clear_mask: $primitive_ty) -> $primitive_ty {
            base & !clear_mask
        }

        /// Returns the highest bit that is set.
        ///
        /// The bit position starts at `0`.
        ///
        /// # Example
        ///
        /// ```rust
        #[doc = concat!("use bit_ops::bitops_", stringify!($primitive_ty), "::highest_bit;")]
        ///
        /// assert_eq!(highest_bit(0), None);
        /// assert_eq!(highest_bit(1), Some(0));
        /// assert_eq!(highest_bit(0b1011), Some(3));
        /// ```
        #[must_use]
        #[inline]
        pub const fn highest_bit(val: $primitive_ty) -> Option<$primitive_ty> {
            if val == 0 {
                None
            } else {
                let max_pos = BIT_COUNT - 1;
                let bit = max_pos - (val.leading_zeros() as $primitive_ty);
                Some(bit)
            }
        }

        /// Returns the lowest bit that is set.
        ///
        /// The bit position starts at `0`.
        ///
        /// # Example
        ///
        /// ```rust
        #[doc = concat!("use bit_ops::bitops_", stringify!($primitive_ty), "::lowest_bit;")]
        ///
        /// assert_eq!(lowest_bit(0), None);
        /// assert_eq!(lowest_bit(1), Some(0));
        /// assert_eq!(lowest_bit(0b1011), Some(0));
        /// assert_eq!(lowest_bit(0b1000), Some(3));
        /// ```
        #[must_use]
        #[inline]
        pub const fn lowest_bit(val: $primitive_ty) -> Option<$primitive_ty> {
            if val == 0 {
                None
            } else {
                Some(val.trailing_zeros() as $primitive_ty)
            }
        }

        /// Get the requested contiguous bits as new integer.
        ///
        /// # Parameters
        ///
        /// - `base`: Base value to get a specific set of bits from.
        /// - `value_bits`: Amount of bits of `value` that are relevant, starting
        ///                 from the right.
        /// - `value_shit`: Position of `value` inside `self`.
        ///
        /// # Panics
        ///
        /// This function panics for overflowing shifts and bit positions that
        /// are outside the range of the underlying type.
        #[must_use]
        #[inline]
        pub const fn get_bits(
            base: $primitive_ty,
            value_bits: $primitive_ty,
            value_shift: $primitive_ty,
        ) -> $primitive_ty {
            let mask = create_mask(value_bits);
            (base >> value_shift) & mask
        }

        /// Creates a bitmask (`1`s) with the given amount of contiguous bits.
        ///
        /// # Example
        ///
        /// ```rust
        #[doc = concat!("use bit_ops::bitops_", stringify!($primitive_ty), "::create_mask;")]
        ///
        /// assert_eq!(create_mask(0), 0);
        /// assert_eq!(create_mask(1), 1);
        /// assert_eq!(create_mask(8), 0b1111_1111);
        /// ```
        ///
        /// # Panics
        ///
        /// This function panics for bit positions that are outside the range
        /// of the underlying type.
        #[must_use]
        #[inline]
        pub const fn create_mask(bits: $primitive_ty) -> $primitive_ty {
            assert_in_range(bits, true);
            if bits == 0 {
                0
            } else if bits == BIT_COUNT {
                <$primitive_ty>::MAX
            } else {
                paste::paste! {
                    [< 1_ $primitive_ty >].wrapping_shl(bits as u32) - 1
                }
            }
        }
    };
}

/// Implements the module wrapping the corresponding [`impl_bit_ops`] code.
macro_rules! impl_mod {
    ($primitive_ty:ty) => {
        paste::paste! {
            /// Various bit manipulation operations for the primitive type
            #[doc = concat!("[`", stringify!($primitive_ty), "`].")]
            ///
            /// All functions are non-mutating but produce a new value.
            pub mod [< bitops _ $primitive_ty >]  {
                impl_bit_ops!($primitive_ty);
            }
        }
    };
}
