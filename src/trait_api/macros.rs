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
            fn set_bit(self, bit: Self) -> Self {
                paste::paste! {
                    $crate::[< bitops _ $primitive_ty >]::set_bit(self, bit)
                }
            }

            #[doc = concat!("Wrapper around [`bitops_", stringify!($primitive_ty), "::set_bit_exact`],")]
            #[doc = concat!("but as associated function (method) on `", stringify!($primitive_ty), "`.")]
            #[doc = ""] // newline needed so that markdown links work
            #[doc = concat!("[`bitops_", stringify!($primitive_ty), "::set_bit_exact`]: crate::bitops_", stringify!($primitive_ty), "::set_bit_exact")]
            #[inline]
            fn set_bit_exact(self, bit: Self, value: bool) -> Self {
                paste::paste! {
                    $crate::[< bitops _ $primitive_ty >]::set_bit_exact(self, bit, value)
                }
            }

            #[doc = concat!("Wrapper around [`bitops_", stringify!($primitive_ty), "::clear_bit`],")]
            #[doc = concat!("but as associated function (method) on `", stringify!($primitive_ty), "`.")]
            #[doc = ""] // newline needed so that markdown links work
            #[doc = concat!("[`bitops_", stringify!($primitive_ty), "::clear_bit`]: crate::bitops_", stringify!($primitive_ty), "::clear_bit")]
            #[inline]
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
            fn get_bit(self, bit: Self) -> Self {
                paste::paste! {
                    $crate::[< bitops _ $primitive_ty >]::get_bit(self, bit)
                }
            }

            #[doc = concat!("Wrapper around [`bitops_", stringify!($primitive_ty), "::toggle_bit`],")]
            #[doc = concat!("but as associated function (method) on `", stringify!($primitive_ty), "`.")]
            #[doc = ""] // newline needed so that markdown links work
            #[doc = concat!("[`bitops_", stringify!($primitive_ty), "::toggle_bit`]: crate::bitops_", stringify!($primitive_ty), "::toggle_bit")]
            #[inline]
            fn toggle_bit(self, bit: Self) -> Self {
                paste::paste! {
                    $crate::[< bitops _ $primitive_ty >]::toggle_bit(self, bit)
                }
            }

            #[doc = concat!("Wrapper around [`bitops_", stringify!($primitive_ty), "::toggle_bits`],")]
            #[doc = concat!("but as associated function (method) on `", stringify!($primitive_ty), "`.")]
            #[doc = ""] // newline needed so that markdown links work
            #[doc = concat!("[`bitops_", stringify!($primitive_ty), "::toggle_bits`]: crate::bitops_", stringify!($primitive_ty), "::toggle_bits")]
            #[inline]
            fn toggle_bits(self, bits: Self, shift: Self) -> Self {
                paste::paste! {
                    $crate::[< bitops _ $primitive_ty >]::toggle_bits(self, bits, shift)
                }
            }

            #[doc = concat!("Wrapper around [`bitops_", stringify!($primitive_ty), "::set_bits`],")]
            #[doc = concat!("but as associated function (method) on `", stringify!($primitive_ty), "`.")]
            #[doc = ""] // newline needed so that markdown links work
            #[doc = concat!("[`bitops_", stringify!($primitive_ty), "::set_bits`]: crate::bitops_", stringify!($primitive_ty), "::set_bits")]
            #[inline]
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
            fn create_mask(bits: Self) -> Self {
                paste::paste! {
                    $crate::[< bitops _ $primitive_ty >]::create_mask(bits)
                }
            }
        }
    };
}
