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
        /// ```rust
        #[doc = concat!("use bit_ops::bitops_", stringify!($primitive_ty), "::set_bit;")]
        ///
        /// let raw = set_bit(0, 7);
        /// assert_eq!(raw, 0b1000_0000);
        /// ```
        ///
        /// # Panics
        ///
        /// Panics for invalid values of `bit`.
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
        /// ```rust
        #[doc = concat!("use bit_ops::bitops_", stringify!($primitive_ty), "::clear_bit;")]
        ///
        /// let raw = clear_bit(0b1000_0000, 7);
        /// assert_eq!(raw, 0);
        /// ```
        ///
        /// # Panics
        ///
        /// Panics for invalid values of `bit`.
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
        /// ```rust
        #[doc = concat!("use bit_ops::bitops_", stringify!($primitive_ty), "::is_set;")]
        ///
        /// assert!(!is_set(0, 0));
        /// assert!(!is_set(2, 0));
        /// assert!(is_set(2, 1));
        /// ```
        ///
        /// # Panics
        ///
        /// Panics for invalid values of `bit`.
        #[must_use]
        #[inline]
        pub const fn is_set(val: $primitive_ty, bit: $primitive_ty) -> bool {
            assert_in_range(bit, false);
            ((val >> bit) & 1) == 1
        }

        /// Toggles (flips) the given bit.
        ///
        /// The bit position starts at `0`.
        ///
        /// # Example
        /// ```rust
        #[doc = concat!("use bit_ops::bitops_", stringify!($primitive_ty), "::toggle;")]
        ///
        /// assert_eq!(toggle(0, 0), 1);
        /// assert_eq!(toggle(0, 1), 2);
        /// assert_eq!(toggle(0, 2), 4);
        /// assert_eq!(toggle(0, 7), 128);
        /// ```
        ///
        /// # Panics
        ///
        /// Panics for invalid values of `bit`.
        #[must_use]
        #[inline]
        pub const fn toggle(val: $primitive_ty, bit: $primitive_ty) -> $primitive_ty {
            assert_in_range(bit, false);
            if is_set(val, bit) {
                clear_bit(val, bit)
            } else {
                set_bit(val, bit)
            }
        }

        /// Sets the bits of `value` in `base` without clearing already set bits.
        ///
        /// # Parameters
        /// - `base`: Base value to set bits in.
        /// - `value`: New value/bits to be set in `base`, but unshifted.
        /// - `value_bits`: Amount of bits of `value` that are relevant, starting from the right.
        /// - `value_shit`: Position of `value` inside `base`.
        ///
        /// # Example
        /// ```rust
        #[doc = concat!("use bit_ops::bitops_", stringify!($primitive_ty), "::set_bits;")]
        ///
        /// // props of a fictional interrupt controler
        /// let delivery_mode = 111;
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
        ///
        /// Panics for invalid values of `value_bits` or `value_shift`.
        /// Only `0..=64` are valid. Shifting bits outside of the type panics
        /// as well.
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

        /// Like [`set_bits`] but calls [`clear_bits`] beforehand for
        /// the relevant bits.
        #[must_use]
        #[inline]
        pub const fn set_bits_exact(
            base: $primitive_ty,
            value: $primitive_ty,       /* unshifted new bits */
            value_bits: $primitive_ty,  /* bits of value to use */
            value_shift: $primitive_ty, /* bits to left-shift value before updating base  */
        ) -> $primitive_ty {
            let clear_mask = create_mask(value_bits) << value_shift;
            let base = clear_bits(base, clear_mask);
            set_bits(base, value, value_bits, value_shift)
        }

        /// Clears all bits specified in the mask by setting them to `0`.
        ///
        /// # Parameters
        /// - `base`: Base value to set bits in.
        /// - `clear_mask`: Bitmask with bits to clear.
        ///
        /// # Example
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
                let bit = BIT_COUNT - (val.leading_zeros() as $primitive_ty) - 1;
                Some(bit)
            }
        }

        /// Returns the lowest bit that is set.
        ///
        /// The bit position starts at `0`.
        ///
        /// # Example
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

        /// Creates a bitmask (`1`s) with the given amount of bits.
        ///
        /// # Example
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
        /// Panics for invalid values of `bits`. Only `0..=64` are valid.
        #[must_use]
        #[inline]
        pub const fn create_mask(bits: $primitive_ty) -> $primitive_ty {
            assert_in_range(bits, true);
            if bits == 0 {
                0
            } else if bits == BIT_COUNT {
                <$primitive_ty>::MAX
            }
            else {
                paste::paste! {
                    [< 1_ $primitive_ty >].wrapping_shl(bits as u32) - 1
                }
            }
        }

        /// Like [`create_mask`] but shifts the mask.
        ///
        /// # Example
        /// ```rust
        #[doc = concat!("use bit_ops::bitops_", stringify!($primitive_ty), "::create_shifted_mask;")]
        ///
        /// assert_eq!(create_shifted_mask(0, 3), 0);
        /// assert_eq!(create_shifted_mask(1, 3), 0b1000);
        /// assert_eq!(create_shifted_mask(3, 2), 0b11100);
        /// ```
        ///
        /// # Panics
        ///
        /// Panics for invalid values of `bits` or `bits`.
        /// Only `0..=64` are valid. Shifting bits outside of the type panics
        /// as well.
        #[must_use]
        #[inline]
        pub const fn create_shifted_mask(
            bits: $primitive_ty,
            lshift: $primitive_ty,
        ) -> $primitive_ty {
            assert_in_range(bits, true);
            create_mask(bits) << lshift
        }
    };
}


/// Implements the module wrapping the corresponding [`impl_bit_ops`] code.
// #[macro_export]
macro_rules! impl_mod {
    ($primitive_ty:ty) => {
        paste::paste! {
            /// Various bit manipulation operations for the primitive type
            #[doc = concat!("[", stringify!($primitive_ty), "].")]
            ///
            /// All functions are non-mutating but produce a new value.
            pub mod [< bitops _ $primitive_ty >]  {
                impl_bit_ops!($primitive_ty);
            }
        }
    };
}
