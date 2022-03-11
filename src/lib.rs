#![doc = include_str!("../README.md")]
mod index;
mod small;
use index::tree_depth;
pub use index::{veb_index, veb_index_rev};

/// get vEB layout vector
pub fn veb_layout<T: Clone>(v: &[T]) -> Vec<T> {
    let l = v.len();
    (0..l)
        .into_iter()
        .map(|i| v[veb_index_rev(i, l)].clone())
        .collect()
}
/// get vEB layout vector (parallel version)
#[cfg(feature = "rayon")]
pub fn par_veb_layout<T: Clone + Send + Sync>(v: &[T]) -> Vec<T> {
    use rayon::prelude::*;
    let l = v.len();
    (0..l)
        .into_par_iter()
        .map(|i| v[veb_index_rev(i, l)].clone())
        .collect()
}

fn get_range(index_rev: usize, l: usize, limit: u32) -> (usize, usize) {
    let mut start = 0;
    let mut end = l;
    for i in (0..limit).rev() {
        let mid = (end - start + 1) / 2 + start;
        if index_rev >> i & 1 == 0 {
            end = mid;
        } else {
            start = mid;
        }
    }
    (start, end)
}

fn veb_partition_point_aux<T, F>(v: &[T], func: &mut F) -> usize
where
    T: Clone,
    F: FnMut(&T) -> bool,
{
    let l = v.len();
    match l {
        0 => 0,
        1 => small::vpp_aux_1(v, func),
        2 => small::vpp_aux_2(v, func),
        3 => small::vpp_aux_3(v, func),
        4 => small::vpp_aux_4(v, func),
        5 => small::vpp_aux_5(v, func),
        6 => small::vpp_aux_6(v, func),
        7 => small::vpp_aux_7(v, func),
        8 => small::vpp_aux_8(v, func),
        9 => small::vpp_aux_9(v, func),
        10 => small::vpp_aux_10(v, func),
        11 => small::vpp_aux_11(v, func),
        12 => small::vpp_aux_12(v, func),
        13 => small::vpp_aux_13(v, func),
        14 => small::vpp_aux_14(v, func),
        15 => small::vpp_aux_15(v, func),
        16 => small::vpp_aux_16(v, func),
        17 => small::vpp_aux_17(v, func),
        18 => small::vpp_aux_18(v, func),
        19 => small::vpp_aux_19(v, func),
        20 => small::vpp_aux_20(v, func),
        21 => small::vpp_aux_21(v, func),
        22 => small::vpp_aux_22(v, func),
        23 => small::vpp_aux_23(v, func),
        24 => small::vpp_aux_24(v, func),
        25 => small::vpp_aux_25(v, func),
        26 => small::vpp_aux_26(v, func),
        27 => small::vpp_aux_27(v, func),
        28 => small::vpp_aux_28(v, func),
        29 => small::vpp_aux_29(v, func),
        30 => small::vpp_aux_30(v, func),
        31 => small::vpp_aux_31(v, func),
        32 => small::vpp_aux_32(v, func),
        33 => small::vpp_aux_33(v, func),
        34 => small::vpp_aux_34(v, func),
        35 => small::vpp_aux_35(v, func),
        36 => small::vpp_aux_36(v, func),
        37 => small::vpp_aux_37(v, func),
        38 => small::vpp_aux_38(v, func),
        39 => small::vpp_aux_39(v, func),
        40 => small::vpp_aux_40(v, func),
        41 => small::vpp_aux_41(v, func),
        42 => small::vpp_aux_42(v, func),
        43 => small::vpp_aux_43(v, func),
        44 => small::vpp_aux_44(v, func),
        45 => small::vpp_aux_45(v, func),
        46 => small::vpp_aux_46(v, func),
        47 => small::vpp_aux_47(v, func),
        48 => small::vpp_aux_48(v, func),
        49 => small::vpp_aux_49(v, func),
        50 => small::vpp_aux_50(v, func),
        51 => small::vpp_aux_51(v, func),
        52 => small::vpp_aux_52(v, func),
        53 => small::vpp_aux_53(v, func),
        54 => small::vpp_aux_54(v, func),
        55 => small::vpp_aux_55(v, func),
        56 => small::vpp_aux_56(v, func),
        57 => small::vpp_aux_57(v, func),
        58 => small::vpp_aux_58(v, func),
        59 => small::vpp_aux_59(v, func),
        60 => small::vpp_aux_60(v, func),
        61 => small::vpp_aux_61(v, func),
        62 => small::vpp_aux_62(v, func),
        63 => small::vpp_aux_63(v, func),
        64 => small::vpp_aux_64(v, func),
        65 => small::vpp_aux_65(v, func),
        66 => small::vpp_aux_66(v, func),
        67 => small::vpp_aux_67(v, func),
        68 => small::vpp_aux_68(v, func),
        69 => small::vpp_aux_69(v, func),
        70 => small::vpp_aux_70(v, func),
        71 => small::vpp_aux_71(v, func),
        72 => small::vpp_aux_72(v, func),
        73 => small::vpp_aux_73(v, func),
        74 => small::vpp_aux_74(v, func),
        75 => small::vpp_aux_75(v, func),
        76 => small::vpp_aux_76(v, func),
        77 => small::vpp_aux_77(v, func),
        78 => small::vpp_aux_78(v, func),
        79 => small::vpp_aux_79(v, func),
        80 => small::vpp_aux_80(v, func),
        81 => small::vpp_aux_81(v, func),
        82 => small::vpp_aux_82(v, func),
        83 => small::vpp_aux_83(v, func),
        84 => small::vpp_aux_84(v, func),
        85 => small::vpp_aux_85(v, func),
        86 => small::vpp_aux_86(v, func),
        87 => small::vpp_aux_87(v, func),
        88 => small::vpp_aux_88(v, func),
        89 => small::vpp_aux_89(v, func),
        90 => small::vpp_aux_90(v, func),
        91 => small::vpp_aux_91(v, func),
        92 => small::vpp_aux_92(v, func),
        93 => small::vpp_aux_93(v, func),
        94 => small::vpp_aux_94(v, func),
        95 => small::vpp_aux_95(v, func),
        96 => small::vpp_aux_96(v, func),
        97 => small::vpp_aux_97(v, func),
        98 => small::vpp_aux_98(v, func),
        99 => small::vpp_aux_99(v, func),
        100 => small::vpp_aux_100(v, func),
        101 => small::vpp_aux_101(v, func),
        102 => small::vpp_aux_102(v, func),
        103 => small::vpp_aux_103(v, func),
        104 => small::vpp_aux_104(v, func),
        105 => small::vpp_aux_105(v, func),
        106 => small::vpp_aux_106(v, func),
        107 => small::vpp_aux_107(v, func),
        108 => small::vpp_aux_108(v, func),
        109 => small::vpp_aux_109(v, func),
        110 => small::vpp_aux_110(v, func),
        111 => small::vpp_aux_111(v, func),
        112 => small::vpp_aux_112(v, func),
        113 => small::vpp_aux_113(v, func),
        114 => small::vpp_aux_114(v, func),
        115 => small::vpp_aux_115(v, func),
        116 => small::vpp_aux_116(v, func),
        117 => small::vpp_aux_117(v, func),
        118 => small::vpp_aux_118(v, func),
        119 => small::vpp_aux_119(v, func),
        120 => small::vpp_aux_120(v, func),
        121 => small::vpp_aux_121(v, func),
        122 => small::vpp_aux_122(v, func),
        123 => small::vpp_aux_123(v, func),
        124 => small::vpp_aux_124(v, func),
        125 => small::vpp_aux_125(v, func),
        126 => small::vpp_aux_126(v, func),
        127 => small::vpp_aux_127(v, func),
        128 => small::vpp_aux_128(v, func),
        129 => small::vpp_aux_129(v, func),
        130 => small::vpp_aux_130(v, func),
        131 => small::vpp_aux_131(v, func),
        132 => small::vpp_aux_132(v, func),
        133 => small::vpp_aux_133(v, func),
        134 => small::vpp_aux_134(v, func),
        135 => small::vpp_aux_135(v, func),
        136 => small::vpp_aux_136(v, func),
        137 => small::vpp_aux_137(v, func),
        138 => small::vpp_aux_138(v, func),
        139 => small::vpp_aux_139(v, func),
        140 => small::vpp_aux_140(v, func),
        141 => small::vpp_aux_141(v, func),
        142 => small::vpp_aux_142(v, func),
        143 => small::vpp_aux_143(v, func),
        144 => small::vpp_aux_144(v, func),
        145 => small::vpp_aux_145(v, func),
        146 => small::vpp_aux_146(v, func),
        147 => small::vpp_aux_147(v, func),
        148 => small::vpp_aux_148(v, func),
        149 => small::vpp_aux_149(v, func),
        150 => small::vpp_aux_150(v, func),
        151 => small::vpp_aux_151(v, func),
        152 => small::vpp_aux_152(v, func),
        153 => small::vpp_aux_153(v, func),
        154 => small::vpp_aux_154(v, func),
        155 => small::vpp_aux_155(v, func),
        156 => small::vpp_aux_156(v, func),
        157 => small::vpp_aux_157(v, func),
        158 => small::vpp_aux_158(v, func),
        159 => small::vpp_aux_159(v, func),
        160 => small::vpp_aux_160(v, func),
        161 => small::vpp_aux_161(v, func),
        162 => small::vpp_aux_162(v, func),
        163 => small::vpp_aux_163(v, func),
        164 => small::vpp_aux_164(v, func),
        165 => small::vpp_aux_165(v, func),
        166 => small::vpp_aux_166(v, func),
        167 => small::vpp_aux_167(v, func),
        168 => small::vpp_aux_168(v, func),
        169 => small::vpp_aux_169(v, func),
        170 => small::vpp_aux_170(v, func),
        171 => small::vpp_aux_171(v, func),
        172 => small::vpp_aux_172(v, func),
        173 => small::vpp_aux_173(v, func),
        174 => small::vpp_aux_174(v, func),
        175 => small::vpp_aux_175(v, func),
        176 => small::vpp_aux_176(v, func),
        177 => small::vpp_aux_177(v, func),
        178 => small::vpp_aux_178(v, func),
        179 => small::vpp_aux_179(v, func),
        180 => small::vpp_aux_180(v, func),
        181 => small::vpp_aux_181(v, func),
        182 => small::vpp_aux_182(v, func),
        183 => small::vpp_aux_183(v, func),
        184 => small::vpp_aux_184(v, func),
        185 => small::vpp_aux_185(v, func),
        186 => small::vpp_aux_186(v, func),
        187 => small::vpp_aux_187(v, func),
        188 => small::vpp_aux_188(v, func),
        189 => small::vpp_aux_189(v, func),
        190 => small::vpp_aux_190(v, func),
        191 => small::vpp_aux_191(v, func),
        192 => small::vpp_aux_192(v, func),
        193 => small::vpp_aux_193(v, func),
        194 => small::vpp_aux_194(v, func),
        195 => small::vpp_aux_195(v, func),
        196 => small::vpp_aux_196(v, func),
        197 => small::vpp_aux_197(v, func),
        198 => small::vpp_aux_198(v, func),
        199 => small::vpp_aux_199(v, func),
        200 => small::vpp_aux_200(v, func),
        201 => small::vpp_aux_201(v, func),
        202 => small::vpp_aux_202(v, func),
        203 => small::vpp_aux_203(v, func),
        204 => small::vpp_aux_204(v, func),
        205 => small::vpp_aux_205(v, func),
        206 => small::vpp_aux_206(v, func),
        207 => small::vpp_aux_207(v, func),
        208 => small::vpp_aux_208(v, func),
        209 => small::vpp_aux_209(v, func),
        210 => small::vpp_aux_210(v, func),
        211 => small::vpp_aux_211(v, func),
        212 => small::vpp_aux_212(v, func),
        213 => small::vpp_aux_213(v, func),
        214 => small::vpp_aux_214(v, func),
        215 => small::vpp_aux_215(v, func),
        216 => small::vpp_aux_216(v, func),
        217 => small::vpp_aux_217(v, func),
        218 => small::vpp_aux_218(v, func),
        219 => small::vpp_aux_219(v, func),
        220 => small::vpp_aux_220(v, func),
        221 => small::vpp_aux_221(v, func),
        222 => small::vpp_aux_222(v, func),
        223 => small::vpp_aux_223(v, func),
        224 => small::vpp_aux_224(v, func),
        225 => small::vpp_aux_225(v, func),
        226 => small::vpp_aux_226(v, func),
        227 => small::vpp_aux_227(v, func),
        228 => small::vpp_aux_228(v, func),
        229 => small::vpp_aux_229(v, func),
        230 => small::vpp_aux_230(v, func),
        231 => small::vpp_aux_231(v, func),
        232 => small::vpp_aux_232(v, func),
        233 => small::vpp_aux_233(v, func),
        234 => small::vpp_aux_234(v, func),
        235 => small::vpp_aux_235(v, func),
        236 => small::vpp_aux_236(v, func),
        237 => small::vpp_aux_237(v, func),
        238 => small::vpp_aux_238(v, func),
        239 => small::vpp_aux_239(v, func),
        240 => small::vpp_aux_240(v, func),
        241 => small::vpp_aux_241(v, func),
        242 => small::vpp_aux_242(v, func),
        243 => small::vpp_aux_243(v, func),
        244 => small::vpp_aux_244(v, func),
        245 => small::vpp_aux_245(v, func),
        246 => small::vpp_aux_246(v, func),
        247 => small::vpp_aux_247(v, func),
        248 => small::vpp_aux_248(v, func),
        249 => small::vpp_aux_249(v, func),
        250 => small::vpp_aux_250(v, func),
        251 => small::vpp_aux_251(v, func),
        252 => small::vpp_aux_252(v, func),
        253 => small::vpp_aux_253(v, func),
        254 => small::vpp_aux_254(v, func),
        255 => small::vpp_aux_255(v, func),
        511 => small::vpp_aux_511(v, func),
        1023 => small::vpp_aux_1023(v, func),
        2047 => small::vpp_aux_2047(v, func),
        4095 => small::vpp_aux_4095(v, func),
        8191 => small::vpp_aux_8191(v, func),
        _ => {
            let depth = tree_depth(l);
            let u_depth = depth / 2;
            let u_len = (1 << u_depth) - 1;
            let d_len = l - u_len;
            let p = veb_partition_point_aux(&v[0..u_len], func);
            let pi = veb_index_rev(p, u_len);
            let (s, e) = get_range(pi, d_len, u_depth);
            let s = u_len + s;
            let e = u_len + e;
            let pp = veb_partition_point_aux(&v[s..e], func);
            let ll = e - s;
            if pp == ll {
                if func(&v[p]) {
                    l
                } else {
                    p
                }
            } else {
                s + pp
            }
        }
    }
}

/** like [`partition_point`](https://doc.rust-lang.org/std/primitive.slice.html#method.partition_point)

```
use partition_point_veb_layout::*;
let v = vec![0, 0, 1, 2, 2, 4, 6];
let lb = v.partition_point(|x| x < &2);
let w = veb_layout(&v);
let vlb = veb_partition_point(&w, |x| x < &2);
assert_eq!(lb, veb_index_rev(vlb, v.len()));
```
*/
pub fn veb_partition_point<T, F>(v: &[T], mut func: F) -> usize
where
    T: Clone,
    F: FnMut(&T) -> bool,
{
    veb_partition_point_aux(v, &mut func)
}

pub mod binary {
    use crate::index::binary_mid;
    pub use crate::index::{index, index_rev};

    pub fn layout<T: Clone>(v: &[T]) -> Vec<T> {
        let l = v.len();
        (0..l)
            .into_iter()
            .map(|i| v[index_rev(i, l)].clone())
            .collect()
    }
    #[cfg(feature = "rayon")]
    pub fn par_layout<T: Clone + Send + Sync>(v: &[T]) -> Vec<T> {
        use rayon::prelude::*;
        let l = v.len();
        (0..l)
            .into_par_iter()
            .map(|i| v[index_rev(i, l)].clone())
            .collect()
    }
    pub fn partition_point<T, F>(v: &[T], mut func: F) -> usize
    where
        T: Clone,
        F: FnMut(&T) -> bool,
    {
        let mut start = 0;
        let mut ps = 0;
        let mut end = v.len();
        let mut sum = 0;
        while end - start > 1 {
            let mid = binary_mid(end - ps) + ps;
            if func(&v[sum]) {
                start = mid;
                ps = mid + 1;
                sum = 2 * sum + 2;
            } else {
                end = mid;
                sum = 2 * sum + 1;
            }
        }
        if func(&v[index(start, v.len())]) {
            end
        } else {
            start
        }
    }
}
