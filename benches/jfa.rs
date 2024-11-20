use blue_noise::jfa_wgpu::run;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let points = vec![(1.0, 1.0), (2.0, 2.0), (3.0, 3.0), (4.0, 4.0), (5.0, 5.0)];

    let mut group = c.benchmark_group("jfa");
    group.sample_size(10);
    //group.bench_function("jfa_cpu", |b| b.iter(|| jfa(black_box(&points), black_box((10.,10.)))));
    group.bench_function("jfa_gpu", |b| {
        b.iter(|| run(black_box(&points), black_box((10., 10.))))
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
