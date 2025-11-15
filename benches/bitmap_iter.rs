use bit_ops::BitmapIter;
use bitvec::prelude::Lsb0;
use criterion::{Criterion, criterion_group, criterion_main};
use rand::Rng;
use std::hint::black_box;

fn get_random_u8(ones_percent: f64) -> u8 {
    assert!((0.0..=100.0).contains(&ones_percent));
    let mut rng = rand::rng();
    let mut value = 0u8;
    for i in 0..8 {
        let bit_is_one = (rng.random_range(0..100) as f64) < ones_percent;
        value |= (bit_is_one as u8) << i;
    }
    value
}

fn get_random_bitmap_u8(ones_percent: f64) -> Box<[u8]> {
    let mut vec = Vec::with_capacity(100_000);
    for _ in 0..vec.capacity() {
        vec.push(get_random_u8(ones_percent));
    }
    vec.into_boxed_slice()
}

fn get_random_u64(ones_percent: f64) -> u64 {
    assert!((0.0..=100.0).contains(&ones_percent));
    let mut rng = rand::rng();
    let mut value = 0;
    for i in 0..64 {
        let bit_is_one = (rng.random_range(0..100) as f64) < ones_percent;
        value |= (bit_is_one as u64) << i;
    }
    value
}

fn get_random_u128(ones_percent: f64) -> u128 {
    assert!((0.0..=100.0).contains(&ones_percent));
    let mut rng = rand::rng();
    let mut value = 0;
    for i in 0..128 {
        let bit_is_one = (rng.random_range(0..100) as f64) < ones_percent;
        value |= (bit_is_one as u128) << i;
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

fn get_random_bitmap_u128(ones_percent: f64) -> Box<[u128]> {
    let mut vec = Vec::with_capacity(5_000);
    for _ in 0..vec.capacity() {
        vec.push(get_random_u128(ones_percent));
    }
    vec.into_boxed_slice()
}

fn bench_bitmap_iter(c: &mut Criterion) {
    c.bench_function("bitmap_iter_u8_0%ones", |b| {
        let bitmap = get_random_bitmap_u8(0.0);
        b.iter(|| {
            let iter = BitmapIter::<u8, _>::new(black_box(bitmap.as_ref().iter().copied()));
            for x in iter {
                let _ = black_box(x);
            }
        })
    });
    c.bench_function("bitmap_iter_u8_0.1%ones", |b| {
        let bitmap = get_random_bitmap_u8(0.1);
        b.iter(|| {
            let iter = BitmapIter::<u8, _>::new(black_box(bitmap.as_ref().iter().copied()));
            for x in iter {
                let _ = black_box(x);
            }
        })
    });
    c.bench_function("bitmap_iter_u8_1%ones", |b| {
        let bitmap = get_random_bitmap_u8(1.0);
        b.iter(|| {
            let iter = BitmapIter::<u8, _>::new(black_box(bitmap.as_ref().iter().copied()));
            for x in iter {
                let _ = black_box(x);
            }
        })
    });
    c.bench_function("bitmap_iter_u8_5%ones", |b| {
        let bitmap = get_random_bitmap_u8(5.0);
        b.iter(|| {
            let iter = BitmapIter::new(black_box(bitmap.as_ref().iter().copied()));
            for x in iter {
                let _ = black_box(x);
            }
        })
    });
    c.bench_function("bitmap_iter_u8_99.9%ones", |b| {
        let bitmap = get_random_bitmap_u8(99.9);
        b.iter(|| {
            let iter = BitmapIter::new(black_box(bitmap.as_ref().iter().copied()));
            for x in iter {
                let _ = black_box(x);
            }
        })
    });
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
    c.bench_function("bitmap_iter_u128_0%ones", |b| {
        let bitmap = get_random_bitmap_u128(0.0);
        b.iter(|| {
            let iter = BitmapIter::<u128, _>::new(black_box(bitmap.as_ref().iter().copied()));
            for x in iter {
                let _ = black_box(x);
            }
        })
    });
    c.bench_function("bitmap_iter_u128_0.1%ones", |b| {
        let bitmap = get_random_bitmap_u128(0.1);
        b.iter(|| {
            let iter = BitmapIter::<u128, _>::new(black_box(bitmap.as_ref().iter().copied()));
            for x in iter {
                let _ = black_box(x);
            }
        })
    });
    c.bench_function("bitmap_iter_u128_1%ones", |b| {
        let bitmap = get_random_bitmap_u128(1.0);
        b.iter(|| {
            let iter = BitmapIter::<u128, _>::new(black_box(bitmap.as_ref().iter().copied()));
            for x in iter {
                let _ = black_box(x);
            }
        })
    });
    c.bench_function("bitmap_iter_u128_5%ones", |b| {
        let bitmap = get_random_bitmap_u128(5.0);
        b.iter(|| {
            let iter = BitmapIter::<u128, _>::new(black_box(bitmap.as_ref().iter().copied()));
            for x in iter {
                let _ = black_box(x);
            }
        })
    });
    c.bench_function("bitmap_iter_u128_99.9%ones", |b| {
        let bitmap = get_random_bitmap_u128(99.9);
        b.iter(|| {
            let iter = BitmapIter::<u128, _>::new(black_box(bitmap.as_ref().iter().copied()));
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
}

criterion_group!(benches, bench_bitmap_iter);
criterion_main!(benches);
