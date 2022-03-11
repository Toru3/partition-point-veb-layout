#[derive(Clone, Debug)]
enum MidOrRange {
    Mid(usize),
    Range((usize, usize), usize),
}

pub fn tree_depth(size: usize) -> u32 {
    (size + 1).next_power_of_two().trailing_zeros()
}

fn index_aux(
    index_rev: usize,
    mut start: usize,
    mut end: usize,
    rem_depth: u32,
    mut i: usize,
    mut sum: usize,
) -> MidOrRange {
    use std::cmp::Ordering::*;
    use MidOrRange::{Mid, Range};
    for rem_depth in (1..=rem_depth).rev() {
        let mid = (end - start) / 2 + start;
        let d = (1 << rem_depth) - 1;
        match index_rev.cmp(&mid) {
            Equal => {
                return Mid(i + d);
            }
            Less => {
                end = mid;
            }
            Greater => {
                i += d + 1;
                sum += mid - start - d;
                start = mid + 1;
            }
        }
    }
    let mid = (end - start) / 2 + start;
    match index_rev.cmp(&mid) {
        Equal => Mid(i),
        Less => Range((start, mid), sum),
        Greater => Range((mid + 1, end), sum + mid - start),
    }
}

/// convert normal index to vEB index
pub fn veb_index(i: usize, size: usize) -> usize {
    use MidOrRange::{Mid, Range};
    if i == size || size < 2 {
        i
    } else {
        let depth = tree_depth(size);
        let u_depth = depth / 2;
        let u_len = (1 << u_depth) - 1;
        match index_aux(i, 0, size, u_depth - 1, 0, u_len) {
            Mid(x) => veb_index(x, (1 << u_depth) - 1),
            Range((s, e), sum) => sum + veb_index(i - s, e - s),
        }
    }
}

fn index_upper(
    index_rev: usize,
    mut start: usize,
    mut end: usize,
    rem_depth: u32,
    mut i: usize,
) -> usize {
    use std::cmp::Ordering::*;
    for rem_depth in (1..=rem_depth).rev() {
        let mid = (end - start) / 2 + start;
        let d = (1 << rem_depth) - 1;
        match index_rev.cmp(&(i + d)) {
            Equal => {
                return mid;
            }
            Less => {
                end = mid;
            }
            Greater => {
                i += d + 1;
                start = mid + 1;
            }
        }
    }
    (end - start) / 2 + start
}

fn index_lower(
    index_rev: usize,
    mut start: usize,
    mut end: usize,
    rem_depth: u32,
    mut sum: usize,
) -> ((usize, usize), usize) {
    for rem_depth in (1..=rem_depth).rev() {
        let mid = (end - start) / 2 + start;
        let d = (1 << rem_depth) - 1;
        let nsum = sum + mid - start - d;
        if index_rev < nsum {
            end = mid;
        } else {
            start = mid + 1;
            sum = nsum;
        }
    }
    let mid = (end - start) / 2 + start;
    let nsum = sum + mid - start;
    if index_rev < nsum {
        ((start, mid), sum)
    } else {
        ((mid + 1, end), nsum)
    }
}

/// convert vEB index to normal index
pub fn veb_index_rev(i: usize, size: usize) -> usize {
    if i == size || size < 2 {
        i
    } else {
        let depth = tree_depth(size);
        let u_depth = depth / 2;
        let u_len = (1 << u_depth) - 1;
        if i < u_len {
            index_upper(veb_index_rev(i, u_len), 0, size, u_depth - 1, 0)
        } else {
            let ((s, e), sum) = index_lower(i, 0, size, u_depth - 1, u_len);
            veb_index_rev(i - sum, e - s) + s
        }
    }
}

pub fn binary_mid(n: usize) -> usize {
    if n < 2 {
        0
    } else {
        let d = tree_depth(n);
        std::cmp::min((1 << (d - 1)) - 1, n - (1 << (d - 2)))
    }
}

pub fn index(i: usize, size: usize) -> usize {
    use std::cmp::Ordering::*;
    if i == size || size < 2 {
        i
    } else {
        let mut start = 0;
        let mut end = size;
        let mut sum = 0;
        loop {
            let mid = binary_mid(end - start) + start;
            match i.cmp(&mid) {
                Equal => return sum,
                Less => {
                    end = mid;
                    sum = 2 * sum + 1;
                }
                Greater => {
                    start = mid + 1;
                    sum = 2 * sum + 2;
                }
            }
        }
    }
}

pub fn index_rev(i: usize, size: usize) -> usize {
    if i == size || size < 2 {
        i
    } else {
        let mut suml = 0;
        let mut sumr = 0;
        let mut depth = 0;
        let p = loop {
            if suml <= i && i <= sumr {
                break i - suml;
            }
            suml = 2 * suml + 1;
            sumr = 2 * sumr + 2;
            depth += 1;
        };
        let mut start = 0;
        let mut end = size;
        let mut sum = 0;
        for d in 0..depth {
            let mid = binary_mid(end - start) + start;
            if p >> (depth - d - 1) & 1 == 0 {
                end = mid;
                sum = 2 * sum + 1;
            } else {
                start = mid + 1;
                sum = 2 * sum + 2;
            }
        }
        binary_mid(end - start) + start
    }
}
