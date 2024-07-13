/// Implements the trait for the primitive type by forwarding all calls to
/// the function API.
macro_rules! impl_trait {
    ($primitive_ty:ty) => {
        impl BitOps for $primitive_ty {
            #[doc = concat!("Wrapper around [`bitops_", stringify!($primitive_ty), "::set_bit`],")]
            #[doc = concat!("but as associated function (method) on `", stringify!($primitive_ty), "`.")]
            #[doc = ""] // newline needed so that markdown links work
            #[doc = concat!("[`bitops_", stringify!($primitive_ty), "::set_bit`]: crate::bitops_", stringify!($primitive_ty), "::set_bit")]
            fn set_bit(self, bit: Self) -> Self {
                paste::paste! {
                    $crate::[< bitops _ $primitive_ty >]::set_bit(self, bit)
                }
            }

            #[doc = concat!("Wrapper around [`bitops_", stringify!($primitive_ty), "::clear_bit`],")]
            #[doc = concat!("but as associated function (method) on `", stringify!($primitive_ty), "`.")]
            #[doc = ""] // newline needed so that markdown links work
            #[doc = concat!("[`bitops_", stringify!($primitive_ty), "::clear_bit`]: crate::bitops_", stringify!($primitive_ty), "::clear_bit")]
            fn clear_bit(self, bit: Self) -> Self {
                paste::paste! {
                    $crate::[< bitops _ $primitive_ty >]::clear_bit(self, bit)
                }
            }

            #[doc = concat!("Wrapper around [`bitops_", stringify!($primitive_ty), "::is_set`],")]
            #[doc = concat!("but as associated function (method) on `", stringify!($primitive_ty), "`.")]
            #[doc = ""] // newline needed so that markdown links work
            #[doc = concat!("[`bitops_", stringify!($primitive_ty), "::is_set`]: crate::bitops_", stringify!($primitive_ty), "::is_set")]
            fn is_set(self, bit: Self) -> bool {
                paste::paste! {
                    $crate::[< bitops _ $primitive_ty >]::is_set(self, bit)
                }
            }

            #[doc = concat!("Wrapper around [`bitops_", stringify!($primitive_ty), "::toggle`],")]
            #[doc = concat!("but as associated function (method) on `", stringify!($primitive_ty), "`.")]
            #[doc = ""] // newline needed so that markdown links work
            #[doc = concat!("[`bitops_", stringify!($primitive_ty), "::toggle`]: crate::bitops_", stringify!($primitive_ty), "::toggle")]
            fn toggle(self, bit: Self) -> Self {
                paste::paste! {
                    $crate::[< bitops _ $primitive_ty >]::toggle(self, bit)
                }
            }

            #[doc = concat!("Wrapper around [`bitops_", stringify!($primitive_ty), "::set_bits`],")]
            #[doc = concat!("but as associated function (method) on `", stringify!($primitive_ty), "`.")]
            #[doc = ""] // newline needed so that markdown links work
            #[doc = concat!("[`bitops_", stringify!($primitive_ty), "::set_bits`]: crate::bitops_", stringify!($primitive_ty), "::set_bits")]
            fn set_bits(self, value: Self, value_bits: Self, value_shift: Self) -> Self {
                paste::paste! {
                    $crate::[< bitops _ $primitive_ty >]::set_bits(self, value, value_bits, value_shift)
                }
            }

            #[doc = concat!("Wrapper around [`bitops_", stringify!($primitive_ty), "::set_bits_n`],")]
            #[doc = concat!("but as associated function (method) on `", stringify!($primitive_ty), "`.")]
            #[doc = ""] // newline needed so that markdown links work
            #[doc = concat!("[`bitops_", stringify!($primitive_ty), "::set_bits_n`]: crate::bitops_", stringify!($primitive_ty), "::set_bits_n")]
            fn set_bits_n(self, ops: &[(Self, Self, Self)]) -> Self {
                paste::paste! {
                    $crate::[< bitops _ $primitive_ty >]::set_bits_n(self, ops)
                }
            }

            #[doc = concat!("Wrapper around [`bitops_", stringify!($primitive_ty), "::set_bits_exact`],")]
            #[doc = concat!("but as associated function (method) on `", stringify!($primitive_ty), "`.")]
            #[doc = ""] // newline needed so that markdown links work
            #[doc = concat!("[`bitops_", stringify!($primitive_ty), "::set_bits_exact`]: crate::bitops_", stringify!($primitive_ty), "::set_bits_exact")]
            fn set_bits_exact(self, value: Self, value_bits: Self, value_shift: Self) -> Self {
                paste::paste! {
                    $crate::[< bitops _ $primitive_ty >]::set_bits_exact(self, value, value_bits, value_shift)
                }
            }

            #[doc = concat!("Wrapper around [`bitops_", stringify!($primitive_ty), "::set_bits_exact_n`],")]
            #[doc = concat!("but as associated function (method) on `", stringify!($primitive_ty), "`.")]
            #[doc = ""] // newline needed so that markdown links work
            #[doc = concat!("[`bitops_", stringify!($primitive_ty), "::set_bits_exact_n`]: crate::bitops_", stringify!($primitive_ty), "::set_bits_exact_n")]
            fn set_bits_exact_n(self, ops: &[(Self, Self, Self)]) -> Self {
                paste::paste! {
                    $crate::[< bitops _ $primitive_ty >]::set_bits_exact_n(self, ops)
                }
            }

            #[doc = concat!("Wrapper around [`bitops_", stringify!($primitive_ty), "::clear_bits`],")]
            #[doc = concat!("but as associated function (method) on `", stringify!($primitive_ty), "`.")]
            #[doc = ""] // newline needed so that markdown links work
            #[doc = concat!("[`bitops_", stringify!($primitive_ty), "::clear_bits`]: crate::bitops_", stringify!($primitive_ty), "::clear_bits")]
            fn clear_bits(self, clear_mask: Self) -> Self {
                paste::paste! {
                    $crate::[< bitops _ $primitive_ty >]::clear_bits(self, clear_mask)
                }
            }

            #[doc = concat!("Wrapper around [`bitops_", stringify!($primitive_ty), "::highest_bit`],")]
            #[doc = concat!("but as associated function (method) on `", stringify!($primitive_ty), "`.")]
            #[doc = ""] // newline needed so that markdown links work
            #[doc = concat!("[`bitops_", stringify!($primitive_ty), "::highest_bit`]: crate::bitops_", stringify!($primitive_ty), "::highest_bit")]
            fn highest_bit(self) -> Option<Self> {
                paste::paste! {
                    $crate::[< bitops _ $primitive_ty >]::highest_bit(self)
                }
            }

            #[doc = concat!("Wrapper around [`bitops_", stringify!($primitive_ty), "::lowest_bit`],")]
            #[doc = concat!("but as associated function (method) on `", stringify!($primitive_ty), "`.")]
            #[doc = ""] // newline needed so that markdown links work
            #[doc = concat!("[`bitops_", stringify!($primitive_ty), "::lowest_bit`]: crate::bitops_", stringify!($primitive_ty), "::lowest_bit")]
            fn lowest_bit(self) -> Option<Self> {
                paste::paste! {
                    $crate::[< bitops _ $primitive_ty >]::lowest_bit(self)
                }
            }

            #[doc = concat!("Wrapper around [`bitops_", stringify!($primitive_ty), "::get_bits`],")]
            #[doc = concat!("but as associated function (method) on `", stringify!($primitive_ty), "`.")]
            #[doc = ""] // newline needed so that markdown links work
            #[doc = concat!("[`bitops_", stringify!($primitive_ty), "::get_bits`]: crate::bitops_", stringify!($primitive_ty), "::get_bits")]
            fn get_bits(self, value_bits: Self, value_shift: Self) -> Self {
                paste::paste! {
                    $crate::[< bitops _ $primitive_ty >]::get_bits(self, value_bits, value_shift)
                }
            }

            #[doc = concat!("Wrapper around [`bitops_", stringify!($primitive_ty), "::create_mask`],")]
            #[doc = concat!("but as associated function (method) on `", stringify!($primitive_ty), "`.")]
            #[doc = ""] // newline needed so that markdown links work
            #[doc = concat!("[`bitops_", stringify!($primitive_ty), "::create_mask`]: crate::bitops_", stringify!($primitive_ty), "::create_mask")]
            fn create_mask(bits: Self) -> Self {
                paste::paste! {
                    $crate::[< bitops _ $primitive_ty >]::create_mask(bits)
                }
            }

            #[doc = concat!("Wrapper around [`bitops_", stringify!($primitive_ty), "::create_shifted_mask`],")]
            #[doc = concat!("but as associated function (method) on `", stringify!($primitive_ty), "`.")]
            #[doc = ""] // newline needed so that markdown links work
            #[doc = concat!("[`bitops_", stringify!($primitive_ty), "::create_shifted_mask`]: crate::bitops_", stringify!($primitive_ty), "::create_shifted_mask")]
            fn create_shifted_mask(bits: Self, lshift: Self) -> Self {
                paste::paste! {
                    $crate::[< bitops _ $primitive_ty >]::create_shifted_mask(bits, lshift)
                }
            }
        }
    };
}
