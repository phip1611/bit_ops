//! Module providing iterators to iterate over set bits in an unsigned integer.
//!
//! See [`BitsIter`] and [`BitmapIter`].

use core::fmt::Debug;
use core::ops::{Add, BitAndAssign, Sub};
use wide::u64x8;

/// **Internal helper** trait for [`BitsIter`].
pub trait Uint:
    Copy + Eq + Add<Output = Self> + Sub<Output = Self> + Sized + BitAndAssign + TryInto<usize>
{
    /// Number of bits of that type.
    const BITS: usize;
    /// `0` value of the underlying primitive type.
    const ZERO: Self;
    /// `1` value of the underlying primitive type.
    const ONE: Self;
    /// Number of trailing zeroes.
    fn trailing_zeros(self) -> Self;
}

/// Implements the relevant bit operations for the specified primitive type.
///
/// Note that the bit positions start at `0`. The highest `bit` position thus
/// is `BITS - 1`.
macro_rules! impl_uint_trait {
    ($primitive_ty:ty) => {
        impl Uint for $primitive_ty {
            const BITS: usize = <$primitive_ty>::BITS as usize;
            const ZERO: Self = 0;
            const ONE: Self = 1;

            fn trailing_zeros(self) -> Self {
                <$primitive_ty>::trailing_zeros(self) as Self
            }
        }
    };
}

impl_uint_trait!(u8);
impl_uint_trait!(u16);
impl_uint_trait!(u32);
impl_uint_trait!(u64);
impl_uint_trait!(u128);
impl_uint_trait!(usize);

/// Iterator over set bits of an unsigned integer.
///
/// The index / bit position starts at `0`, the last bit position is
/// `n_bits - 1`.
///
/// The iterator can be used with [`u8`], [`u16`], [`u32`], [`u64`], [`u128`],
/// and [`usize`].
///
/// # Example
/// ```rust
/// # use bit_ops::BitsIter;
/// // also works with u16, u32, u64, u128, and usize
/// let iter = BitsIter::<u8>::new(0);
/// assert_eq!(&iter.collect::<Vec<_>>(), &[]);
///
/// let iter = BitsIter::<u8>::new(1);
/// assert_eq!(&iter.collect::<Vec<_>>(), &[0]);
///
/// let iter = BitsIter::<u8>::new(0b1010_1010);
/// assert_eq!(&iter.collect::<Vec<_>>(), &[1, 3, 5, 7]);
/// ```
#[derive(Debug)]
pub struct BitsIter<U> {
    value: U,
}

impl<U: Uint> BitsIter<U>
where
    <U as TryInto<usize>>::Error: Debug,
{
    /// Creates a new iterator.
    pub const fn new(value: U) -> Self {
        Self { value }
    }
}

impl<U: Uint> Iterator for BitsIter<U>
where
    <U as TryInto<usize>>::Error: Debug,
{
    type Item = U;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.value == U::ZERO {
            return None;
        }
        let tz = self.value.trailing_zeros();
        self.value &= self.value - U::ONE; // clear lowest set bit
        Some(tz)
    }
}

/// Iterator over set bits in (large) bitmaps, i.e., collection of unsigned
/// integers.
///
/// This wraps an iterator emitting corresponding unsigned integers and
/// uses [`BitsIter`] on each element. While doing so, [`BitmapIter`] keeps
/// track of consumed elements to properly report the bit position relative to
/// the very first bit.
///
/// The iterator can be used with [`u8`], [`u16`], [`u32`], [`u64`], [`u128`],
/// and [`usize`].
///
/// # Example
/// ```rust
/// # use bit_ops::BitmapIter;
/// // also works with u16, u32, u64, u128, and usize
/// let iter = BitmapIter::<u8, _>::new([0b1111_0010, 0b1000, 1].into_iter());
/// assert_eq!(&iter.collect::<Vec<_>>(), &[1, 4, 5, 6, 7, 11, 16]);
/// ```
#[derive(Debug)]
pub struct BitmapIter<U, I> {
    bitmap_iter: I,
    consumed_count: usize,
    current_element_it: BitsIter<U>,
}

impl<U: Uint, I: Iterator<Item = U>> BitmapIter<U, I>
where
    <U as TryInto<usize>>::Error: Debug,
{
    /// Creates a new iterator.
    ///
    /// This consumes everything that implements [`IntoIterator`] for an
    /// [`Iterator`] of the corresponding [`Uint`].
    ///
    /// # Example
    /// ```rust
    /// # use bit_ops::BitmapIter;
    /// let _ = BitmapIter::<u8, _>::new([0_u8]);
    /// let _ = BitmapIter::<u16, _>::new([0_u16].iter().copied());
    /// let _ = BitmapIter::<u16, _>::new((&[0_u16]).iter().copied());
    /// let _ = BitmapIter::<usize, _>::new((vec![42_usize]));
    /// ```
    pub fn new<In: IntoIterator<IntoIter = I>>(bitmap_iter: In) -> Self {
        let mut bitmap_iter = bitmap_iter.into_iter();
        let current_element_it = BitsIter::new(bitmap_iter.next().unwrap_or(U::ZERO));
        Self {
            bitmap_iter,
            consumed_count: 0,
            current_element_it,
        }
    }
}

impl<U: Uint, I: Iterator<Item = U>> Iterator for BitmapIter<U, I>
where
    <U as TryInto<usize>>::Error: Debug,
{
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // We return here, if we currently have an element.
            if let Some(bit) = self.current_element_it.next() {
                let bit: usize = bit.try_into().unwrap();
                return Some(bit + self.consumed_count);
            }

            // Current byte exhausted: load next one or return `None` / exit.
            let next_byte = self.bitmap_iter.next()?;
            self.consumed_count += U::BITS;
            self.current_element_it = BitsIter::new(next_byte);
        }
    }
}

/*enum SimdBitmapIterState {
    SkipZeroesSimd {
        /// Total amount of processed elements.
        count: usize,
    },
    IterateBitPositions {
        bitpos_iter: BitsIter<u64>,
        elements_left: usize,
        /// Total amount of processed elements.
        count: usize,
    }
}*/

/// .
#[derive(Debug)]
pub struct SimdBitmapIter<I> {
    underlying_it: I,
    buffer: u64x8,
    elems_to_iter_normally: usize,
    current_bitpos_iter: Option<BitsIter<u64>>,
    count: usize,
}

impl<I: ExactSizeIterator<Item = u64>> SimdBitmapIter<I> {
    /// .
    pub const fn new(underlying_it: I) -> Self {
        Self {
            underlying_it,
            buffer: u64x8::ZERO,
            elems_to_iter_normally: 0,
            current_bitpos_iter: None,
            count: 0,
        }
    }
}

impl<I: ExactSizeIterator<Item = u64> + Clone> Iterator for SimdBitmapIter<I> {
    type Item = u64;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // Iterate actual non-zero fields (discovered in a previous
            // iteration) from the underlying buffer (we already consumed the
            // iterator).
            if self.elems_to_iter_normally > 0 {
                // Update `self.current_bitpos_iter` if necessary
                if self.current_bitpos_iter.is_none() {
                    let index = 8 - self.elems_to_iter_normally;
                    self.elems_to_iter_normally -= 1;
                    // SAFETY: valid index.
                    let elem = unsafe {
                        *self.buffer.as_array().get_unchecked(index)
                    };
                    self.current_bitpos_iter = Some(BitsIter::new(elem));
                }

                // SAFETY: We just checked the iterator is there.
                let iter = unsafe { self.current_bitpos_iter.as_mut().unwrap_unchecked() };
                match iter.next() {
                    // We still had at least one bit left on the current element
                    Some(pos) => {
                        return Some(pos + self.count as u64);
                    }
                    // We drained the current element
                    None => {
                        // Ensure we advance the element for the next
                        // invocation.
                        self.count += 64;
                        let _ = self.current_bitpos_iter.take();
                        continue;
                    }
                }
            }
            // Using SIMD, we skip all only-zero elements (likely case).
            {
                let hint = self.underlying_it.size_hint().0;
                if hint == 0 {
                    // drained
                    return None;
                }
                if hint < 8 {
                    panic!("hint is {hint}");
                };

                // SAFETY: We checked the remaining length beforehand.
                self.buffer = unsafe {
                    u64x8::new([
                        self.underlying_it.next().unwrap_unchecked(),
                        self.underlying_it.next().unwrap_unchecked(),
                        self.underlying_it.next().unwrap_unchecked(),
                        self.underlying_it.next().unwrap_unchecked(),
                        self.underlying_it.next().unwrap_unchecked(),
                        self.underlying_it.next().unwrap_unchecked(),
                        self.underlying_it.next().unwrap_unchecked(),
                        self.underlying_it.next().unwrap_unchecked(),
                    ])
                };

                if self.buffer == u64x8::ZERO {
                    self.count += 8 * 64;
                    continue;
                } else {
                    self.elems_to_iter_normally = 8;
                    continue;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::vec::Vec;

    #[test]
    fn test_bits_iter() {
        let iter = BitsIter::<u8>::new(0);
        assert_eq!(&iter.collect::<Vec<_>>(), &[]);

        let iter = BitsIter::<u8>::new(1);
        assert_eq!(&iter.collect::<Vec<_>>(), &[0]);

        let iter = BitsIter::<u8>::new(0b1010_1010);
        assert_eq!(&iter.collect::<Vec<_>>(), &[1, 3, 5, 7]);

        let iter = BitsIter::<u8>::new(0b1111_1111);
        assert_eq!(&iter.collect::<Vec<_>>(), &[0, 1, 2, 3, 4, 5, 6, 7]);

        let iter = BitsIter::<u128>::new(0b1111_1111);
        assert_eq!(&iter.collect::<Vec<_>>(), &[0, 1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn test_bitmap_iter() {
        let iter = BitmapIter::<u8, _>::new([0_u8]);
        assert_eq!(&iter.collect::<Vec<_>>(), &[]);

        let iter = BitmapIter::<u8, _>::new([0b1111_0010, 0b1000, 1]);
        assert_eq!(&iter.collect::<Vec<_>>(), &[1, 4, 5, 6, 7, 11, 16]);

        let iter = BitmapIter::<u128, _>::new([0b10, 0b10, 0b11]);
        assert_eq!(&iter.collect::<Vec<_>>(), &[1, 129, 256, 257]);
    }

    #[test]
    fn test_simd_bitmap_iter() {
        let data = [0, 0, 0, 0, 0, 0, 0, 0];
        let iter = SimdBitmapIter::new(data.into_iter());
        assert_eq!(&iter.collect::<Vec<_>>(), &[]);

        let data = [1, 0, 0, 0, 0, 0, 0, 0];
        let iter = SimdBitmapIter::new(data.into_iter());
        assert_eq!(&iter.collect::<Vec<_>>(), &[0]);

        let data = [1, 1, 0, 0, 0, 0, 0, 0];
        let iter = SimdBitmapIter::new(data.into_iter());
        assert_eq!(&iter.collect::<Vec<_>>(), &[0, 64]);

        #[rustfmt::skip]
        let data = [
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            1, 0, 0, 0, 1, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            1, 1, 1, 1, 1, 1, 1, 1
        ];
        let iter = SimdBitmapIter::new(data.into_iter());
        assert_eq!(
            &iter.collect::<Vec<_>>(),
            &[1024, 1280, 2560, 2624, 2688, 2752, 2816, 2880, 2944, 3008]
        );
    }
}
