use partition_point_veb_layout::*;
use rand::{Rng, SeedableRng};
use rayon::prelude::*;
mod aux {
    use smallvec::{smallvec, SmallVec};
    fn tree_depth(size: usize) -> u32 {
        (size + 1).next_power_of_two().trailing_zeros()
    }

    fn separate_aux<T: Clone, const N: usize, const M: usize>(
        v: &[T],
        start: usize,
        end: usize,
        rem_depth: u32,
        ret: &mut SmallVec<[T; N]>,
        int: &mut SmallVec<[(usize, usize); M]>,
    ) {
        let mid = (end - start) / 2 + start;
        if rem_depth == 0 {
            int.push((start, mid));
            ret.push(v[mid].clone());
            int.push((mid + 1, end));
        } else {
            separate_aux(v, start, mid, rem_depth - 1, ret, int);
            ret.push(v[mid].clone());
            separate_aux(v, mid + 1, end, rem_depth - 1, ret, int);
        }
    }

    type VecRange<const N: usize> = SmallVec<[(usize, usize); N]>;

    fn separate<T: Clone, const N: usize, const M: usize>(
        v: &[T],
        limit: u32,
    ) -> (SmallVec<[T; N]>, VecRange<M>) {
        if limit == 1 {
            let mid = v.len() / 2;
            let ret = smallvec![v[mid].clone()];
            let int = smallvec![(0, mid), (mid + 1, v.len())];
            (ret, int)
        } else {
            let mut ret = SmallVec::with_capacity((1 << limit) - 1);
            let mut int = SmallVec::with_capacity(1 << limit);
            separate_aux(v, 0, v.len(), limit - 1, &mut ret, &mut int);
            (ret, int)
        }
    }

    pub fn veb_layout<T: Clone + Send + Sync>(v: &[T]) -> Vec<T> {
        if v.len() < 2 {
            v.to_vec()
        } else {
            //println!("v = {:?}", v);
            let l = v.len();
            let depth = tree_depth(l);
            let u_depth = depth / 2;
            let (uv, dvs) = separate::<T, 7, 8>(v, u_depth);
            let mut r = veb_layout(&uv);
            r.reserve(v.len() - r.len());
            for (s, e) in dvs {
                let mut t = veb_layout(&v[s..e]);
                r.append(&mut t);
            }
            //println!("r = {:?}", r);
            r
        }
    }
}

fn test_short_aux(n: usize) {
    dbg!(n);
    {
        let v = (0..n).collect::<Vec<_>>();
        let x = aux::veb_layout(&v);
        #[cfg(feature = "rayon")]
        {
            let y = par_veb_layout(&v);
            assert_eq!(x, y);
        }
        for i in 0..n {
            //println!(
            //    "--------------------------------------------------------------------------------"
            //);
            //println!("{:?}", x);
            //dbg!(i, n);
            let j = veb_index(i, n);
            let k = veb_index_rev(j, n);
            //dbg!(n, i, j, k);
            assert_eq!(i, x[j]);
            assert_eq!(i, k);
        }
    }
    dbg!(n);
    (0..100i32).into_par_iter().for_each(|_| {
        let mut v = vec![0; n];
        let nh = n as i64 / 2;
        v.chunks_mut(1024 * 1024).for_each(|v| {
            let mut rng = pcg_rand::Pcg32::from_entropy();
            for i in v.iter_mut() {
                *i = rng.gen_range(0..nh);
            }
        });
        v.sort_unstable();
        let x = veb_layout(&v);
        let mut rng = pcg_rand::Pcg32::from_entropy();
        for _ in 0..2_000 {
            let t = rng.gen_range(0..=nh);
            let il = v.partition_point(|u| u < &t);
            let iu = v.partition_point(|u| u <= &t);
            let jl = veb_partition_point(&x, |u| u < &t);
            let ju = veb_partition_point(&x, |u| u <= &t);
            let kl = veb_index(il, n);
            let ku = veb_index(iu, n);
            for z in &v[il..iu] {
                assert_eq!(t, *z);
            }
            if jl != kl || ju != ku {
                println!("v = {v:?}");
                println!("x = {x:?}");
                dbg!(n, t, il, iu, jl, ju, kl, ku);
            }
            assert_eq!(jl, kl);
            assert_eq!(ju, ku);
        }
    });
}
#[test]
fn test_short() {
    for n in 2..1024 {
        test_short_aux(n);
    }
}
#[test]
fn test_mid1() {
    for n in 0..250 {
        test_short_aux(1013 * n + 1009);
    }
}
#[test]
fn test_mid2() {
    for n in 250..500 {
        test_short_aux(1013 * n + 1009);
    }
}
#[test]
fn test_mid3() {
    for n in 500..750 {
        test_short_aux(1013 * n + 1009);
    }
}
#[test]
fn test_mid4() {
    for n in 750..1000 {
        test_short_aux(1013 * n + 1009);
    }
}

#[test]
fn test_long() {
    let n = 3 * 1024 * 1024 * 1024;
    let mut v = vec![0; n];
    let nh = n as i64 / 2;
    println!("fill rand");
    v.par_chunks_mut(1024 * 1024).for_each(|v| {
        let mut rng = pcg_rand::Pcg32::from_entropy();
        for i in v.iter_mut() {
            *i = rng.gen_range(0..nh);
        }
    });
    println!("sort");
    v.par_sort_unstable();
    println!("veb layout");
    #[cfg(feature = "rayon")]
    let x = par_veb_layout(&v);
    #[cfg(not(feature = "rayon"))]
    let x = veb_layout(&v);
    println!("lower_bound");
    (0..1_000i32).into_par_iter().for_each(|_| {
        let mut rng = pcg_rand::Pcg32::from_entropy();
        for _ in 0..1_000_000 {
            let t = rng.gen_range(0..=nh);
            let il = v.partition_point(|u| u < &t);
            let iu = v.partition_point(|u| u <= &t);
            let jl = veb_partition_point(&x, |u| u < &t);
            let ju = veb_partition_point(&x, |u| u <= &t);
            let kl = veb_index(il, n);
            let ku = veb_index(iu, n);
            if jl != kl || ju != ku {
                println!("v = {v:?}");
                println!("x = {x:?}");
                dbg!(n, t, il, iu, jl, ju, kl, ku,);
            }
            assert_eq!(jl, kl);
            assert_eq!(ju, ku);
        }
    });
}

#[test]
fn short_rev() {
    let v = [8, 4, 12, 2, 6, 10, 14, 1, 3, 5, 7, 9, 11, 13, 15];
    for i in 0..15 {
        let j = binary::index(i, 15);
        let k = v[j] - 1;
        let l = binary::index_rev(j, 15);
        dbg!(i, j, k, l);
        assert_eq!(i, k);
        assert_eq!(i, l);
    }
    for n in 2..1000 {
        for i in 0..n {
            let j = binary::index(i, n);
            let k = binary::index_rev(j, n);
            if i != k {
                dbg!(n, i, j, k);
            }
            assert_eq!(i, k);
        }
    }
    for n in 2..=1024 {
        let v = (0..n).collect::<Vec<_>>();
        let x = binary::layout(&v);
        for t in 0..n {
            let il = v.partition_point(|u| u < &t);
            let iu = v.partition_point(|u| u <= &t);
            let jl = binary::partition_point(&x, |u| u < &t);
            let ju = binary::partition_point(&x, |u| u <= &t);
            let kl = il; //index(il, n);
            let ku = iu; //index(iu, n);
            for z in &v[il..iu] {
                assert_eq!(t, *z);
            }
            if jl != kl || ju != ku {
                println!("v = {v:?}");
                println!("x = {x:?}");
                dbg!(n, t, il, iu, jl, ju, kl, ku);
            }
            assert_eq!(jl, kl);
            assert_eq!(ju, ku);
        }
    }
    for n in 2..=1024 {
        let mut v = vec![0; n];
        let nh = n as i64 / 2;
        v.chunks_mut(1024 * 1024).for_each(|v| {
            let mut rng = pcg_rand::Pcg32::from_entropy();
            for i in v.iter_mut() {
                *i = rng.gen_range(0..nh);
            }
        });
        v.sort_unstable();
        let x = binary::layout(&v);
        let mut rng = pcg_rand::Pcg32::from_entropy();
        for _ in 0..2_000 {
            let t = rng.gen_range(0..=nh);
            let il = v.partition_point(|u| u < &t);
            let iu = v.partition_point(|u| u <= &t);
            let jl = binary::partition_point(&x, |u| u < &t);
            let ju = binary::partition_point(&x, |u| u <= &t);
            let kl = il; //index(il, n);
            let ku = iu; //index(iu, n);
            for z in &v[il..iu] {
                assert_eq!(t, *z);
            }
            if jl != kl || ju != ku {
                println!("v = {v:?}");
                println!("x = {x:?}");
                dbg!(n, t, il, iu, jl, ju, kl, ku);
            }
            assert_eq!(jl, kl);
            assert_eq!(ju, ku);
        }
    }
}
