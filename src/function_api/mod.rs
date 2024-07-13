//! The function-oriented API of this crate, which is also the main
//! functionality. For every relevant primitive type (`u8`, `u16`, `u32`, `u64`,
//! `usize`), there is a module with typical bit operations as dedicated
//! functions.
//!
//! This enables a smooth manual and direct approach to several bit-oriented
//! operations without the need for sophisticated high-level types with specific
//! bitfields.
//!
//! - [`bitops_u8`]
//! - [`bitops_u16`]
//! - [`bitops_u32`]
//! - [`bitops_u64`]
//! - [`bitops_usize`]

#[macro_use]
mod macros;

impl_mod!(u8);
impl_mod!(u16);
impl_mod!(u32);
impl_mod!(u64);
impl_mod!(usize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_bit() {
        assert_eq!(bitops_u8::set_bit(0, 0), 1);
        assert_eq!(bitops_u8::set_bit(1, 1), 3);
        assert_eq!(bitops_u8::set_bit(1, 0), 1);
        assert_eq!(bitops_u8::set_bit(1 << 7, 6), 0b1100_0000);
        assert_eq!(bitops_u8::set_bit(u8::MAX - 1, 0), u8::MAX);

        assert_eq!(bitops_u64::set_bit(0, 0), 1);
        assert_eq!(bitops_u64::set_bit(1, 1), 3);
        assert_eq!(bitops_u64::set_bit(1, 0), 1);
        assert_eq!(bitops_u64::set_bit(1 << 7, 6), 0b1100_0000);
        assert_eq!(bitops_u64::set_bit(u64::MAX - 1, 0), u64::MAX);
    }

    #[test]
    fn clear_bit() {
        assert_eq!(bitops_u8::clear_bit(1, 0), 0);
        assert_eq!(bitops_u8::clear_bit(3, 1), 1);
        assert_eq!(bitops_u8::clear_bit(1, 0), 0);
        assert_eq!(bitops_u8::clear_bit(1 << 7, 7), 0);
        assert_eq!(bitops_u8::clear_bit(u8::MAX, 7), u8::MAX / 2);

        assert_eq!(bitops_u64::clear_bit(1, 0), 0);
        assert_eq!(bitops_u64::clear_bit(3, 1), 1);
        assert_eq!(bitops_u64::clear_bit(1, 0), 0);
        assert_eq!(bitops_u64::clear_bit(1 << 7, 7), 0);
        assert_eq!(bitops_u64::clear_bit(u64::MAX, 63), u64::MAX / 2);
    }

    #[test]
    fn is_set() {
        assert!(!bitops_u8::is_set(0, 0));
        assert!(bitops_u8::is_set(1, 0));
        assert!(bitops_u8::is_set(u8::MAX, 0));
        assert!(bitops_u8::is_set(u8::MAX, 7));
        assert!(!bitops_u8::is_set(1 << 6, 7));

        assert!(!bitops_u64::is_set(0, 0));
        assert!(bitops_u64::is_set(1, 0));
        assert!(bitops_u64::is_set(u64::MAX, 0));
        assert!(bitops_u64::is_set(u64::MAX, 7));
    }

    #[test]
    fn toggle() {
        assert!(bitops_u8::is_set(bitops_u8::toggle(0, 0), 0));
        assert!(!bitops_u8::is_set(bitops_u8::toggle(1, 0), 0));
        assert_eq!(bitops_u8::toggle(3, 0), 0b10);
        assert_eq!(bitops_u8::toggle(u8::MAX, 7), u8::MAX / 2);

        assert!(bitops_u64::is_set(bitops_u64::toggle(0, 0), 0));
        assert!(!bitops_u64::is_set(bitops_u64::toggle(1, 0), 0));
        assert_eq!(bitops_u64::toggle(3, 0), 0b10);
        assert_eq!(bitops_u64::toggle(u64::MAX, 63), u64::MAX / 2);
    }

    #[test]
    fn set_bits() {
        assert_eq!(bitops_u8::set_bits(0, 0, 0, 0), 0);
        assert_eq!(bitops_u8::set_bits(5, 0, 0, 0), 5);
        assert_eq!(bitops_u8::set_bits(0b100, 0b11, 2, 0), 0b111);
        assert_eq!(bitops_u8::set_bits(0b100, 0b11, 2, 2), 0b1100);
        assert_eq!(bitops_u8::set_bits(0b100, 0b11, 1, 0), 0b101);
        assert_eq!(bitops_u8::set_bits(0b100, 0b11, 0, 0), 0b100);
        assert_eq!(bitops_u8::set_bits(0, u8::MAX, 8, 0), u8::MAX);
        assert_eq!(bitops_u8::set_bits(0, u8::MAX, 1, 7), 0b1000_0000);

        assert_eq!(bitops_u64::set_bits(0, 0, 0, 0), 0);
        assert_eq!(bitops_u64::set_bits(5, 0, 0, 0), 5);
        assert_eq!(bitops_u64::set_bits(0b100, 0b11, 2, 0), 0b111);
        assert_eq!(bitops_u64::set_bits(0b100, 0b11, 2, 2), 0b1100);
        assert_eq!(bitops_u64::set_bits(0b100, 0b11, 1, 0), 0b101);
        assert_eq!(bitops_u64::set_bits(0b100, 0b11, 0, 0), 0b100);
        assert_eq!(bitops_u64::set_bits(0, u64::MAX, 64, 0), u64::MAX);
    }

    #[test]
    fn set_bits_exact() {
        assert_eq!(bitops_u8::set_bits_exact(0, 0, 0, 0), 0);
        assert_eq!(
            bitops_u8::set_bits_exact(0b1000_1100, 0b010, 3, 0),
            0b1000_1010
        );
        assert_eq!(
            bitops_u8::set_bits_exact(0b11111111, 0b10101, 5, 3),
            0b10101111
        );

        assert_eq!(bitops_u64::set_bits_exact(0, 0, 0, 0), 0);
        assert_eq!(
            bitops_u64::set_bits_exact(0b1000_1100, 0b010, 3, 0),
            0b1000_1010
        );
        assert_eq!(
            bitops_u64::set_bits_exact(0b11111111, 0b10101, 5, 3),
            0b10101111
        );
    }

    #[test]
    fn clear_bits() {
        assert_eq!(bitops_u8::clear_bits(0, 0), 0);
        assert_eq!(bitops_u8::clear_bits(0b11111111, 0), 0b11111111);
        assert_eq!(bitops_u8::clear_bits(0b11111111, 0b01111110), 0b10000001);

        assert_eq!(bitops_u64::clear_bits(0, 0), 0);
        assert_eq!(bitops_u64::clear_bits(0b11111111, 0), 0b11111111);
        assert_eq!(bitops_u64::clear_bits(0b11111111, 0b01111110), 0b10000001);
        assert_eq!(bitops_u64::clear_bits(u64::MAX, u64::MAX), 0);
    }

    #[test]
    fn highest_bit() {
        assert_eq!(bitops_u8::highest_bit(0), None);
        assert_eq!(bitops_u8::highest_bit(1), Some(0));
        assert_eq!(bitops_u8::highest_bit(0b10), Some(1));
        assert_eq!(bitops_u8::highest_bit(0b11), Some(1));
        assert_eq!(bitops_u8::highest_bit(u8::MAX), Some(7));

        assert_eq!(bitops_u64::highest_bit(0), None);
        assert_eq!(bitops_u64::highest_bit(1), Some(0));
        assert_eq!(bitops_u64::highest_bit(0b10), Some(1));
        assert_eq!(bitops_u64::highest_bit(0b11), Some(1));
        assert_eq!(bitops_u64::highest_bit(u64::MAX), Some(63));
    }

    #[test]
    fn lowest_bit() {
        assert_eq!(bitops_u8::lowest_bit(0), None);
        assert_eq!(bitops_u8::lowest_bit(1), Some(0));
        assert_eq!(bitops_u8::lowest_bit(0b1000_0001), Some(0));
        assert_eq!(bitops_u8::lowest_bit(0b1000_0010), Some(1));
        assert_eq!(bitops_u8::lowest_bit(u8::MAX), Some(0));

        assert_eq!(bitops_u64::lowest_bit(0), None);
        assert_eq!(bitops_u64::lowest_bit(1), Some(0));
        assert_eq!(bitops_u64::lowest_bit(0b1000_0001), Some(0));
        assert_eq!(bitops_u64::lowest_bit(0b1000_0010), Some(1));
        assert_eq!(bitops_u64::lowest_bit(u64::MAX), Some(0));
    }

    #[test]
    fn create_mask() {
        assert_eq!(bitops_u8::create_mask(0), 0);
        assert_eq!(bitops_u8::create_mask(1), 0b1);
        assert_eq!(bitops_u8::create_mask(2), 0b11);
        assert_eq!(bitops_u8::create_mask(3), 0b111);
        assert_eq!(bitops_u8::create_mask(8), 0xff);

        assert_eq!(bitops_u64::create_mask(0), 0);
        assert_eq!(bitops_u64::create_mask(64), u64::MAX);
    }

    #[test]
    fn create_mask_shifted() {
        assert_eq!(bitops_u8::create_shifted_mask(0, 5), 0);
        assert_eq!(bitops_u8::create_shifted_mask(1, 5), 0b10_0000);
        assert_eq!(bitops_u8::create_shifted_mask(2, 5), 0b110_0000);
        assert_eq!(bitops_u8::create_shifted_mask(3, 5), 0b1110_0000);

        assert_eq!(bitops_u64::create_shifted_mask(1, 63), u64::MAX / 2 + 1);
    }

    /// This tests various functions in combination using a real-world scenario.
    #[test]
    fn combined() {
        use x86_ioapic::*;

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

        let vector = 7;
        let delivery_mode = 0b111; // ExtInt
        let destination_mode = 0; // physical
        let pin_polarity = 1; // low-active
        let trigger_mode = 1; // level-triggered
        let masked = 1;
        let destination = 13;

        let redirection_entry = bitops_u64::set_bits_exact_n(
            0,
            &[
                (vector, VECTOR_BITS, VECTOR_SHIFT),
                (delivery_mode, DELIVERY_MODE_BITS, DELIVERY_MODE_SHIFT),
                (
                    destination_mode,
                    DESTINATION_MODE_BITS,
                    DESTINATION_MODE_SHIFT,
                ),
                (pin_polarity, PIN_POLARITY_BITS, PIN_POLARITY_SHIFT),
                (trigger_mode, TRIGGER_MODE_BITS, TRIGGER_MODE_SHIFT),
                (masked, MASKED_BITS, MASKED_SHIFT),
                (destination, DESTINATION_BITS, DESTINATION_SHIFT),
            ],
        );
        assert_eq!(redirection_entry, 0xd0000000001a707);

        // Now check the other direction: get those bits again

        assert_eq!(
            bitops_u64::get_bits(redirection_entry, VECTOR_BITS, VECTOR_SHIFT),
            vector
        );
        assert_eq!(
            bitops_u64::get_bits(redirection_entry, DELIVERY_MODE_BITS, DELIVERY_MODE_SHIFT),
            delivery_mode
        );
        assert_eq!(
            bitops_u64::get_bits(redirection_entry, DESTINATION_MODE_BITS, DESTINATION_MODE_SHIFT),
            destination_mode
        );
        assert_eq!(
            bitops_u64::get_bits(redirection_entry, PIN_POLARITY_BITS, PIN_POLARITY_SHIFT),
            pin_polarity
        );
        assert_eq!(
            bitops_u64::get_bits(redirection_entry, TRIGGER_MODE_BITS, TRIGGER_MODE_SHIFT),
            trigger_mode
        );
        assert_eq!(
            bitops_u64::get_bits(redirection_entry, MASKED_BITS, MASKED_SHIFT),
            masked
        );
        assert_eq!(
            bitops_u64::get_bits(redirection_entry, DESTINATION_BITS, DESTINATION_SHIFT),
            destination
        );
    }
}
