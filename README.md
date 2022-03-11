vEB(van Emde Boas) layout `partition_point` (cache oblivious)

```rust
use partition_point_veb_layout::*;
let v = vec![0, 0, 1, 2, 2, 4, 6];
let lb = v.partition_point(|x| x < &2);
let w = veb_layout(&v);
let vlb = veb_partition_point(&w, |x| x < &2);
assert_eq!(lb, veb_index_rev(vlb, v.len()));
```
