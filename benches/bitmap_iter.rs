use bit_ops::BitmapIter;
use bitvec::prelude::Lsb0;
use criterion::{Criterion, criterion_group, criterion_main};
use rand::Rng;
use std::hint::black_box;
use std::ops::Mul;

/// An iterator that turns a sequence of u64s into a sequence of bit positions
/// that are set.
///
/// This is useful to iterate over dirty memory bitmaps.
struct BlitzBitposIterator<I> {
    underlying_it: I,

    /// How many `u64`'s we've already consumed.
    ///
    /// `u32` is sufficient.
    word_pos: u32,

    /// If we already started working on a u64, it's here. Together with the bit
    /// position where we have to continue.
    current_word: Option<(u64 /* cur word */, u32 /* cur pos */)>,
}

impl<I> Iterator for BlitzBitposIterator<I>
where
    I: Iterator<Item = u64>,
{
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.current_word.is_none() {
                self.current_word = self.underlying_it.next().map(|w| (w, 0));
            }

            let (word, word_bit) = self.current_word?;

            // Continue early if there is no chance to find something.
            if word != 0 && word_bit < 64 {
                let shifted_word = word >> word_bit;
                if shifted_word != 0 {
                    let zeroes = shifted_word.trailing_zeros();

                    self.current_word = Some((word, zeroes + word_bit + 1));
                    let next_bitpos = (self.word_pos as u64)
                        .mul(64)
                        // the inner value can not overflow
                        .checked_add(word_bit as u64 + zeroes as u64)
                        .unwrap();

                    return Some(next_bitpos);
                }
            }

            self.current_word = None;
            self.word_pos += 1;
        }
    }
}



pub trait BitposIteratorExt: Iterator<Item = u64> + Sized {
    /// Turn an iterator over `u64` into an iterator over the bit positions of
    /// all 1s. We basically treat the incoming `u64` as one gigantic integer
    /// and just spit out which bits are set.
    fn bit_positions(self) -> impl Iterator<Item = u64> {
        BlitzBitposIterator {
            underlying_it: self,
            word_pos: 0,
            current_word: None,
        }
    }
}

impl<I: Iterator<Item = u64> + Sized> BitposIteratorExt for I {}

fn get_random_u8(ones_percent: u8) -> u8 {
    assert!(ones_percent <= 100);
    let mut rng = rand::rng();
    let mut value = 0u8;
    for i in 0..8 {
        let bit_is_one = rng.random_range(0..100) < ones_percent;
        value |= (bit_is_one as u8) << i;
    }
    value
}

fn get_random_bitmap_u8(ones_percent: u8) -> Box<[u8]> {
    let mut vec = Vec::with_capacity(100_000);
    for _ in 0..vec.capacity() {
        vec.push(get_random_u8(ones_percent));
    }
    vec.into_boxed_slice()
}

fn get_random_u64(ones_percent: f64) -> u64 {
    assert!(ones_percent <= 100.0 && ones_percent >= 0.0);
    let mut rng = rand::rng();
    let mut value = 0;
    for i in 0..64 {
        let bit_is_one = (rng.random_range(0..100) as f64) < ones_percent;
        value |= (bit_is_one as u64) << i;
    }
    value
}

fn get_random_bitmap_u64(ones_percent: f64) -> Box<[u64]> {
    let mut vec = Vec::with_capacity(10_000);
    for _ in 0..vec.capacity() {
        vec.push(get_random_u64(ones_percent));
    }
    vec.into_boxed_slice()
}

fn bench_bitmap_iter(c: &mut Criterion) {
    /*c.bench_function("bitmap_iter_u8_0%ones", |b| {
        let bitmap = get_random_bitmap_u8(0);
        b.iter(|| {
            let iter = BitmapIter::<u8, _>::new(black_box(bitmap.as_ref().iter().copied()));
            for x in iter {
                let _ = black_box(x);
            }
        })
    });
    c.bench_function("bitmap_iter_u8_10%ones", |b| {
        let bitmap = get_random_bitmap_u8(10);
        b.iter(|| {
            let iter = BitmapIter::<u8, _>::new(black_box(bitmap.as_ref().iter().copied()));
            for x in iter {
                let _ = black_box(x);
            }
        })
    });
    c.bench_function("bitmap_iter_u8_40%ones", |b| {
        let bitmap = get_random_bitmap_u8(40);
        b.iter(|| {
            let iter = BitmapIter::<u8, _>::new(black_box(bitmap.as_ref().iter().copied()));
            for x in iter {
                let _ = black_box(x);
            }
        })
    });
    c.bench_function("bitmap_iter_u8_70%ones", |b| {
        let bitmap = get_random_bitmap_u8(70);
        b.iter(|| {
            let iter = BitmapIter::new(black_box(bitmap.as_ref().iter().copied()));
            for x in iter {
                let _ = black_box(x);
            }
        })
    });
    c.bench_function("bitmap_iter_u8_99%ones", |b| {
        let bitmap = get_random_bitmap_u8(99);
        b.iter(|| {
            let iter = BitmapIter::new(black_box(bitmap.as_ref().iter().copied()));
            for x in iter {
                let _ = black_box(x);
            }
        })
    });*/
    /* ------------------------------------------------------------ */
    c.bench_function("bitmap_iter_u64_0%ones", |b| {
        let bitmap = get_random_bitmap_u64(0.0);
        b.iter(|| {
            let iter = BitmapIter::<u64, _>::new(black_box(bitmap.as_ref().iter().copied()));
            for x in iter {
                let _ = black_box(x);
            }
        })
    });
    c.bench_function("bitmap_iter_u64_0.1%ones", |b| {
        let bitmap = get_random_bitmap_u64(0.1);
        b.iter(|| {
            let iter = BitmapIter::<u64, _>::new(black_box(bitmap.as_ref().iter().copied()));
            for x in iter {
                let _ = black_box(x);
            }
        })
    });
    c.bench_function("bitmap_iter_u64_1%ones", |b| {
        let bitmap = get_random_bitmap_u64(1.0);
        b.iter(|| {
            let iter = BitmapIter::<u64, _>::new(black_box(bitmap.as_ref().iter().copied()));
            for x in iter {
                let _ = black_box(x);
            }
        })
    });
    c.bench_function("bitmap_iter_u64_5%ones", |b| {
        let bitmap = get_random_bitmap_u64(5.0);
        b.iter(|| {
            let iter = BitmapIter::<u64, _>::new(black_box(bitmap.as_ref().iter().copied()));
            for x in iter {
                let _ = black_box(x);
            }
        })
    });
    c.bench_function("bitmap_iter_u64_99.9%ones", |b| {
        let bitmap = get_random_bitmap_u64(99.9);
        b.iter(|| {
            let iter = BitmapIter::<u64, _>::new(black_box(bitmap.as_ref().iter().copied()));
            for x in iter {
                let _ = black_box(x);
            }
        })
    });
    /* ------------------------------------------------------------ */
    c.bench_function("bitvec_iter_u64_0%ones", |b| {
        let bitmap = get_random_bitmap_u64(0.0);
        b.iter(|| {
            let iter = bitvec::slice::BitSlice::<u64, Lsb0>::from_slice(&bitmap);
            let iter = iter.iter_ones();
            for x in iter {
                let _ = black_box(x);
            }
        })
    });
    c.bench_function("bitvec_iter_u64_0.1%ones", |b| {
        let bitmap = get_random_bitmap_u64(0.1);
        b.iter(|| {
            let iter = bitvec::slice::BitSlice::<u64, Lsb0>::from_slice(&bitmap);
            let iter = iter.iter_ones();
            for x in iter {
                let _ = black_box(x);
            }
        })
    });
    c.bench_function("bitvec_iter_u64_1%ones", |b| {
        let bitmap = get_random_bitmap_u64(1.0);
        b.iter(|| {
            let iter = bitvec::slice::BitSlice::<u64, Lsb0>::from_slice(&bitmap);
            let iter = iter.iter_ones();
            for x in iter {
                let _ = black_box(x);
            }
        })
    });
    c.bench_function("bitvec_iter_u64_5%ones", |b| {
        let bitmap = get_random_bitmap_u64(5.0);
        b.iter(|| {
            let iter = bitvec::slice::BitSlice::<u64, Lsb0>::from_slice(&bitmap);
            let iter = iter.iter_ones();
            for x in iter {
                let _ = black_box(x);
            }
        })
    });
    c.bench_function("bitvec_iter_u64_99.9%ones", |b| {
        let bitmap = get_random_bitmap_u64(99.9);
        b.iter(|| {
            let iter = bitvec::slice::BitSlice::<u64, Lsb0>::from_slice(&bitmap);
            let iter = iter.iter_ones();
            for x in iter {
                let _ = black_box(x);
            }
        })
    });
    /* ------------------------------------------------------------ */
    c.bench_function("blitz_bitmap_iter_u64_0%ones", |b| {
        let bitmap = get_random_bitmap_u64(0.0);
        b.iter(|| {
            let iter = black_box(bitmap.as_ref().iter().copied().bit_positions());
            for x in iter {
                let _ = black_box(x);
            };
        })
    });
    c.bench_function("blitz_bitmap_iter_u64_0.1%ones", |b| {
        let bitmap = get_random_bitmap_u64(0.1);
        b.iter(|| {
            let iter = black_box(bitmap.as_ref().iter().copied().bit_positions());
            for x in iter {
                let _ = black_box(x);
            };
        })
    });
    c.bench_function("blitz_bitmap_iter_u64_1%ones", |b| {
        let bitmap = get_random_bitmap_u64(1.0);
        b.iter(|| {
            let iter = black_box(bitmap.as_ref().iter().copied().bit_positions());
            for x in iter {
                let _ = black_box(x);
            };
        })
    });
    c.bench_function("blitz_bitmap_iter_u64_5%ones", |b| {
        let bitmap = get_random_bitmap_u64(5.0);
        b.iter(|| {
            let iter = black_box(bitmap.as_ref().iter().copied().bit_positions());
            for x in iter {
                let _ = black_box(x);
            };
        })
    });
    c.bench_function("blitz_bitmap_iter_u64_99.9%ones", |b| {
        let bitmap = get_random_bitmap_u64(99.9);
        b.iter(|| {
            let iter = black_box(bitmap.as_ref().iter().copied().bit_positions());
            for x in iter {
                let _ = black_box(x);
            };
        })
    });
}

criterion_group!(benches, bench_bitmap_iter);
criterion_main!(benches);
