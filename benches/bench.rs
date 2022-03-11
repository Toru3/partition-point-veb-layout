use criterion::{
    black_box, criterion_group, criterion_main, AxisScale, BenchmarkId, Criterion,
    PlotConfiguration,
};
use partition_point_veb_layout::*;
use rand::{Rng, SeedableRng};
use rayon::prelude::*;

fn make_input(n: usize, nh: usize) -> (Vec<usize>, Vec<usize>, Vec<usize>) {
    let mut v = vec![0; n];
    v.par_chunks_mut(1024 * 1024).for_each(|v| {
        let mut rng = pcg_rand::Pcg32::from_entropy();
        for i in v.iter_mut() {
            *i = rng.gen_range(0..nh);
        }
    });
    v.par_sort_unstable();
    #[cfg(feature = "rayon")]
    let w = binary::par_layout(&v);
    #[cfg(not(feature = "rayon"))]
    let w = binary::layout(&v);
    #[cfg(feature = "rayon")]
    let x = par_veb_layout(&v);
    #[cfg(not(feature = "rayon"))]
    let x = veb_layout(&v);
    (v, w, x)
}

pub fn bench(c: &mut Criterion) {
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    let mut group = c.benchmark_group("partition_point_2^n+42");
    group.plot_config(plot_config);
    for i in 10..=31 {
        let size = (1 << i) + 42;
        let nh = size / 2;
        let (v, w, x) = make_input(size, nh);
        let mut rng = pcg_rand::Pcg32::from_entropy();
        group.bench_with_input(BenchmarkId::new("slice", size), &size, |b, &_| {
            b.iter(|| {
                let t = black_box(rng.gen_range(0..=nh));
                black_box(&v).partition_point(|u| u < &t)
            })
        });
        group.bench_with_input(
            BenchmarkId::new("veb_partition_point", size),
            &size,
            |b, &_| {
                b.iter(|| {
                    let t = black_box(rng.gen_range(0..=nh));
                    veb_partition_point(black_box(&x), |u| u < &t)
                })
            },
        );
        group.bench_with_input(BenchmarkId::new("partition_point", size), &size, |b, &_| {
            b.iter(|| {
                let t = black_box(rng.gen_range(0..=nh));
                binary::partition_point(black_box(&w), |u| u < &t)
            })
        });
    }
    group.finish();
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    let mut group = c.benchmark_group("equal_range_2^n+42");
    group.plot_config(plot_config);
    for i in 10..=31 {
        let size = (1 << i) + 42;
        let nh = size / 10;
        let (v, w, x) = make_input(size, nh);
        let mut rng = pcg_rand::Pcg32::from_entropy();
        group.bench_with_input(BenchmarkId::new("slice", size), &size, |b, &_| {
            b.iter(|| {
                let t = black_box(rng.gen_range(0..=nh));
                let lb = black_box(&v).partition_point(|u| u < &t);
                let ub = black_box(&v).partition_point(|u| u <= &t);
                let _ = black_box(&v[lb..ub]);
            })
        });
        group.bench_with_input(
            BenchmarkId::new("veb_partition_point", size),
            &size,
            |b, &_| {
                b.iter(|| {
                    let t = black_box(rng.gen_range(0..=nh));
                    let lb = veb_partition_point(black_box(&x), |u| u < &t);
                    let ub = veb_partition_point(black_box(&x), |u| u <= &t);
                    let il = veb_index_rev(lb, x.len());
                    let iu = veb_index_rev(ub, x.len());
                    let mut v = Vec::with_capacity(iu - il);
                    for i in il..iu {
                        v.push(x[veb_index(i, x.len())]);
                    }
                    let _ = black_box(v);
                })
            },
        );
        group.bench_with_input(BenchmarkId::new("partition_point", size), &size, |b, &_| {
            b.iter(|| {
                let t = black_box(rng.gen_range(0..=nh));
                let il = binary::partition_point(black_box(&w), |u| u < &t);
                let iu = binary::partition_point(black_box(&w), |u| u <= &t);
                let mut v = Vec::with_capacity(iu - il);
                for i in il..iu {
                    v.push(w[binary::index(i, w.len())]);
                }
                let _ = black_box(v);
            })
        });
    }
}

criterion_group! {
    name = benches;
    config = Criterion::default().measurement_time(std::time::Duration::new(10, 0));
    targets = bench
}
criterion_main!(benches);
