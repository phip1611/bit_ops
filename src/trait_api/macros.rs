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
/// Implements the trait for the primitive type by forwarding all calls to
/// the function API.
macro_rules! impl_trait {
    ($primitive_ty:ty) => {
        impl BitOps for $primitive_ty {
            #[doc = concat!("Wrapper around [`bitops_", stringify!($primitive_ty), "::set_bit`],")]
            #[doc = concat!("but as associated function (method) on `", stringify!($primitive_ty), "`.")]
            #[doc = ""] // newline needed so that markdown links work
            #[doc = concat!("[`bitops_", stringify!($primitive_ty), "::set_bit`]: crate::bitops_", stringify!($primitive_ty), "::set_bit")]
            #[inline]
            #[must_use]
            fn set_bit(self, bit: Self) -> Self {
                paste::paste! {
                    $crate::[< bitops _ $primitive_ty >]::set_bit(self, bit)
                }
            }

            #[doc = concat!("Wrapper around [`bitops_", stringify!($primitive_ty), "::clear_bit`],")]
            #[doc = concat!("but as associated function (method) on `", stringify!($primitive_ty), "`.")]
            #[doc = ""] // newline needed so that markdown links work
            #[doc = concat!("[`bitops_", stringify!($primitive_ty), "::clear_bit`]: crate::bitops_", stringify!($primitive_ty), "::clear_bit")]
            #[inline]
            #[must_use]
            fn clear_bit(self, bit: Self) -> Self {
                paste::paste! {
                    $crate::[< bitops _ $primitive_ty >]::clear_bit(self, bit)
                }
            }

            #[doc = concat!("Wrapper around [`bitops_", stringify!($primitive_ty), "::is_set`],")]
            #[doc = concat!("but as associated function (method) on `", stringify!($primitive_ty), "`.")]
            #[doc = ""] // newline needed so that markdown links work
            #[doc = concat!("[`bitops_", stringify!($primitive_ty), "::is_set`]: crate::bitops_", stringify!($primitive_ty), "::is_set")]
            #[inline]
            #[must_use]
            fn is_set(self, bit: Self) -> bool {
                paste::paste! {
                    $crate::[< bitops _ $primitive_ty >]::is_set(self, bit)
                }
            }

            #[doc = concat!("Wrapper around [`bitops_", stringify!($primitive_ty), "::get_bit`],")]
            #[doc = concat!("but as associated function (method) on `", stringify!($primitive_ty), "`.")]
            #[doc = ""] // newline needed so that markdown links work
            #[doc = concat!("[`bitops_", stringify!($primitive_ty), "::get_bit`]: crate::bitops_", stringify!($primitive_ty), "::get_bit")]
            #[inline]
            #[must_use]
            fn get_bit(self, bit: Self) -> Self {
                paste::paste! {
                    $crate::[< bitops _ $primitive_ty >]::get_bit(self, bit)
                }
            }

            #[doc = concat!("Wrapper around [`bitops_", stringify!($primitive_ty), "::toggle`],")]
            #[doc = concat!("but as associated function (method) on `", stringify!($primitive_ty), "`.")]
            #[doc = ""] // newline needed so that markdown links work
            #[doc = concat!("[`bitops_", stringify!($primitive_ty), "::toggle`]: crate::bitops_", stringify!($primitive_ty), "::toggle")]
            #[inline]
            #[must_use]
            fn toggle(self, bit: Self) -> Self {
                paste::paste! {
                    $crate::[< bitops _ $primitive_ty >]::toggle(self, bit)
                }
            }

            #[doc = concat!("Wrapper around [`bitops_", stringify!($primitive_ty), "::set_bits`],")]
            #[doc = concat!("but as associated function (method) on `", stringify!($primitive_ty), "`.")]
            #[doc = ""] // newline needed so that markdown links work
            #[doc = concat!("[`bitops_", stringify!($primitive_ty), "::set_bits`]: crate::bitops_", stringify!($primitive_ty), "::set_bits")]
            #[inline]
            #[must_use]
            fn set_bits(self, value: Self, value_bits: Self, value_shift: Self) -> Self {
                paste::paste! {
                    $crate::[< bitops _ $primitive_ty >]::set_bits(self, value, value_bits, value_shift)
                }
            }

            #[doc = concat!("Wrapper around [`bitops_", stringify!($primitive_ty), "::set_bits_n`],")]
            #[doc = concat!("but as associated function (method) on `", stringify!($primitive_ty), "`.")]
            #[doc = ""] // newline needed so that markdown links work
            #[doc = concat!("[`bitops_", stringify!($primitive_ty), "::set_bits_n`]: crate::bitops_", stringify!($primitive_ty), "::set_bits_n")]
            #[inline]
            #[must_use]
            fn set_bits_n(self, ops: &[(Self, Self, Self)]) -> Self {
                paste::paste! {
                    $crate::[< bitops _ $primitive_ty >]::set_bits_n(self, ops)
                }
            }

            #[doc = concat!("Wrapper around [`bitops_", stringify!($primitive_ty), "::set_bits_exact`],")]
            #[doc = concat!("but as associated function (method) on `", stringify!($primitive_ty), "`.")]
            #[doc = ""] // newline needed so that markdown links work
            #[doc = concat!("[`bitops_", stringify!($primitive_ty), "::set_bits_exact`]: crate::bitops_", stringify!($primitive_ty), "::set_bits_exact")]
            #[inline]
            #[must_use]
            fn set_bits_exact(self, value: Self, value_bits: Self, value_shift: Self) -> Self {
                paste::paste! {
                    $crate::[< bitops _ $primitive_ty >]::set_bits_exact(self, value, value_bits, value_shift)
                }
            }

            #[doc = concat!("Wrapper around [`bitops_", stringify!($primitive_ty), "::set_bits_exact_n`],")]
            #[doc = concat!("but as associated function (method) on `", stringify!($primitive_ty), "`.")]
            #[doc = ""] // newline needed so that markdown links work
            #[doc = concat!("[`bitops_", stringify!($primitive_ty), "::set_bits_exact_n`]: crate::bitops_", stringify!($primitive_ty), "::set_bits_exact_n")]
            #[inline]
            #[must_use]
            fn set_bits_exact_n(self, ops: &[(Self, Self, Self)]) -> Self {
                paste::paste! {
                    $crate::[< bitops _ $primitive_ty >]::set_bits_exact_n(self, ops)
                }
            }

            #[doc = concat!("Wrapper around [`bitops_", stringify!($primitive_ty), "::clear_bits`],")]
            #[doc = concat!("but as associated function (method) on `", stringify!($primitive_ty), "`.")]
            #[doc = ""] // newline needed so that markdown links work
            #[doc = concat!("[`bitops_", stringify!($primitive_ty), "::clear_bits`]: crate::bitops_", stringify!($primitive_ty), "::clear_bits")]
            #[inline]
            #[must_use]
            fn clear_bits(self, clear_mask: Self) -> Self {
                paste::paste! {
                    $crate::[< bitops _ $primitive_ty >]::clear_bits(self, clear_mask)
                }
            }

            #[doc = concat!("Wrapper around [`bitops_", stringify!($primitive_ty), "::highest_bit`],")]
            #[doc = concat!("but as associated function (method) on `", stringify!($primitive_ty), "`.")]
            #[doc = ""] // newline needed so that markdown links work
            #[doc = concat!("[`bitops_", stringify!($primitive_ty), "::highest_bit`]: crate::bitops_", stringify!($primitive_ty), "::highest_bit")]
            #[inline]
            #[must_use]
            fn highest_bit(self) -> Option<Self> {
                paste::paste! {
                    $crate::[< bitops _ $primitive_ty >]::highest_bit(self)
                }
            }

            #[doc = concat!("Wrapper around [`bitops_", stringify!($primitive_ty), "::lowest_bit`],")]
            #[doc = concat!("but as associated function (method) on `", stringify!($primitive_ty), "`.")]
            #[doc = ""] // newline needed so that markdown links work
            #[doc = concat!("[`bitops_", stringify!($primitive_ty), "::lowest_bit`]: crate::bitops_", stringify!($primitive_ty), "::lowest_bit")]
            #[inline]
            #[must_use]
            fn lowest_bit(self) -> Option<Self> {
                paste::paste! {
                    $crate::[< bitops _ $primitive_ty >]::lowest_bit(self)
                }
            }

            #[doc = concat!("Wrapper around [`bitops_", stringify!($primitive_ty), "::get_bits`],")]
            #[doc = concat!("but as associated function (method) on `", stringify!($primitive_ty), "`.")]
            #[doc = ""] // newline needed so that markdown links work
            #[doc = concat!("[`bitops_", stringify!($primitive_ty), "::get_bits`]: crate::bitops_", stringify!($primitive_ty), "::get_bits")]
            #[inline]
            #[must_use]
            fn get_bits(self, value_bits: Self, value_shift: Self) -> Self {
                paste::paste! {
                    $crate::[< bitops _ $primitive_ty >]::get_bits(self, value_bits, value_shift)
                }
            }

            #[doc = concat!("Wrapper around [`bitops_", stringify!($primitive_ty), "::create_mask`],")]
            #[doc = concat!("but as associated function (method) on `", stringify!($primitive_ty), "`.")]
            #[doc = ""] // newline needed so that markdown links work
            #[doc = concat!("[`bitops_", stringify!($primitive_ty), "::create_mask`]: crate::bitops_", stringify!($primitive_ty), "::create_mask")]
            #[inline]
            #[must_use]
            fn create_mask(bits: Self) -> Self {
                paste::paste! {
                    $crate::[< bitops _ $primitive_ty >]::create_mask(bits)
                }
            }

            #[doc = concat!("Wrapper around [`bitops_", stringify!($primitive_ty), "::create_shifted_mask`],")]
            #[doc = concat!("but as associated function (method) on `", stringify!($primitive_ty), "`.")]
            #[doc = ""] // newline needed so that markdown links work
            #[doc = concat!("[`bitops_", stringify!($primitive_ty), "::create_shifted_mask`]: crate::bitops_", stringify!($primitive_ty), "::create_shifted_mask")]
            #[inline]
            #[must_use]
            fn create_shifted_mask(bits: Self, lshift: Self) -> Self {
                paste::paste! {
                    $crate::[< bitops _ $primitive_ty >]::create_shifted_mask(bits, lshift)
                }
            }
        }
    };
}
