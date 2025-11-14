use bit_ops::{BitmapIter, SimdBitmapIter};
use criterion::{Criterion, criterion_group, criterion_main};
use rand::Rng;
use std::hint::black_box;

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

fn get_random_u64(ones_percent: u64) -> u64 {
    assert!(ones_percent <= 100);
    let mut rng = rand::rng();
    let mut value = 0;
    for i in 0..64 {
        let bit_is_one = rng.random_range(0..100) < ones_percent;
        value |= (bit_is_one as u64) << i;
    }
    value
}

fn get_random_bitmap_u64(ones_percent: u64) -> Box<[u64]> {
    let mut vec = Vec::with_capacity(10_000);
    for _ in 0..vec.capacity() {
        vec.push(get_random_u64(ones_percent));
    }
    vec.into_boxed_slice()
}

fn bench_bitmap_iter(c: &mut Criterion) {
    /*
    c.bench_function("bitmap_iter_u8_0%ones", |b| {
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

    c.bench_function("bitmap_iter_u64_0%ones", |b| {
        let bitmap = get_random_bitmap_u64(0);
        b.iter(|| {
            let iter = BitmapIter::new(black_box(bitmap.as_ref().iter().copied()));
            for x in iter {
                let _ = black_box(x);
            }
        })
    });
    c.bench_function("bitmap_iter_u64_10%ones", |b| {
        let bitmap = get_random_bitmap_u64(10);
        b.iter(|| {
            let iter = BitmapIter::new(black_box(bitmap.as_ref().iter().copied()));
            for x in iter {
                let _ = black_box(x);
            }
        })
    });
    /*c.bench_function("bitmap_iter_u64_40%ones", |b| {
        let bitmap = get_random_bitmap_u64(40);
        b.iter(|| {
            let iter = BitmapIter::new(black_box(bitmap.as_ref().iter().copied()));
            for x in iter {
                let _ = black_box(x);
            }
        })
    });*/
    c.bench_function("bitmap_iter_u64_70%ones", |b| {
        let bitmap = get_random_bitmap_u64(70);
        b.iter(|| {
            let iter = BitmapIter::new(black_box(bitmap.as_ref().iter().copied()));
            for x in iter {
                let _ = black_box(x);
            }
        })
    });
    /*c.bench_function("bitmap_iter_u64_99%ones", |b| {
        let bitmap = get_random_bitmap_u64(99);
        b.iter(|| {
            let iter = BitmapIter::new(black_box(bitmap.as_ref().iter().copied()));
            for x in iter {
                let _ = black_box(x);
            }
        })
    });*/

    c.bench_function("simd_bitmap_iter_u64_0%ones", |b| {
        let bitmap = get_random_bitmap_u64(0);
        b.iter(|| {
            let iter = SimdBitmapIter::new(black_box(bitmap.as_ref().iter().copied()));
            for x in iter {
                let _ = black_box(x);
            }
        })
    });
    c.bench_function("simd_bitmap_iter_u64_10%ones", |b| {
        let bitmap = get_random_bitmap_u64(10);
        b.iter(|| {
            let iter = SimdBitmapIter::new(black_box(bitmap.as_ref().iter().copied()));
            for x in iter {
                let _ = black_box(x);
            }
        })
    });
    /*c.bench_function("simd_bitmap_iter_u64_40%ones", |b| {
        let bitmap = get_random_bitmap_u64(40);
        b.iter(|| {
            let iter = SimdBitmapIter::new(black_box(bitmap.as_ref().iter().copied()));
            for x in iter {
                let _ = black_box(x);
            }
        })
    });*/
    c.bench_function("simd_bitmap_iter_u64_70%ones", |b| {
        let bitmap = get_random_bitmap_u64(70);
        b.iter(|| {
            let iter = SimdBitmapIter::new(black_box(bitmap.as_ref().iter().copied()));
            for x in iter {
                let _ = black_box(x);
            }
        })
    });
    /*c.bench_function("simd_bitmap_iter_u64_99%ones", |b| {
        let bitmap = get_random_bitmap_u64(99);
        b.iter(|| {
            let iter = SimdBitmapIter::new(black_box(bitmap.as_ref().iter().copied()));
            for x in iter {
                let _ = black_box(x);
            }
        })
    });*/
}

criterion_group!(benches, bench_bitmap_iter);
criterion_main!(benches);
