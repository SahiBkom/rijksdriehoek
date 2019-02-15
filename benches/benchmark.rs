use criterion::Criterion;
use criterion::{criterion_group, criterion_main};
use rijksdriehoek::*;

pub fn assert_f64(a: f64, b: f64, delta: f64) {
    assert!((a - b).abs() < delta, "{} != {}", a, b);
}

pub fn assert_f32(a: f32, b: f32, delta: f32) {
    assert!((a - b).abs() < delta, "{} != {}", a, b);
}

fn bench(c: &mut Criterion) {
    c.bench_function("Amsterdam westertoren f64", |b| {
        let mut lat_lon = (52.37453253f64, 4.88352559f64);
        b.iter(|| {
            for _ in 0..100 {
                let (x, y) = wgs84_to_rijksdriehoek(lat_lon.0, lat_lon.1);
                lat_lon = rijksdriehoek_to_wgs84(x, y);
            }
        });
        // TODO: why? 5.125536254862837 != 4.88352559
        // assert_f64(lat_lon.0, 52.37453253, 0.00001);
        // assert_f64(lat_lon.1, 4.88352559, 0.00001);
    });

    c.bench_function("Amsterdam westertoren f32", |b| {
        let mut lat_lon = (52.37453253f32, 4.88352559f32);
        b.iter(|| {
            for _ in 0..100 {
                let (x, y) = wgs84_to_rijksdriehoek(lat_lon.0, lat_lon.1);
                lat_lon = rijksdriehoek_to_wgs84(x, y);
            }
        });
        assert_f32(lat_lon.0, 52.37453253, 0.00001);
        assert_f32(lat_lon.1, 4.88352559, 0.00001);
    });
}

criterion_group!(benches, bench);
// criterion_group!(benches, bench_amsterdam_westertoren_f64);
criterion_main!(benches);
