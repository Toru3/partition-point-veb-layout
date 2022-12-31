#![allow(dead_code)]
use std::io::Write;
#[path = "src/index.rs"]
mod index;
use index::veb_index;

#[derive(Clone, Debug)]
enum Ast {
    Leaf(usize),
    Branch(usize, Box<Ast>, Box<Ast>),
}

fn veb_layout_ast_aux(v: &[usize], s: usize, e: usize) -> Ast {
    use Ast::*;
    let l = e - s;
    if l == 0 {
        let i = veb_index(s, v.len());
        Leaf(i)
    } else if l == 1 {
        let i = veb_index(s, v.len());
        let j = veb_index(s + 1, v.len());
        Ast::Branch(i, Box::new(Leaf(j)), Box::new(Leaf(i)))
    } else {
        let mid = (e - s) / 2 + s;
        let left = veb_layout_ast_aux(v, s, mid);
        let right = veb_layout_ast_aux(v, mid + 1, e);
        Ast::Branch(veb_index(mid, v.len()), Box::new(right), Box::new(left))
    }
}

fn veb_layout_ast(v: &[usize]) -> Ast {
    let l = v.len();
    veb_layout_ast_aux(v, 0, l)
}

fn ast<W: Write>(dst: &mut W, a: &Ast, indent: usize) -> std::io::Result<()> {
    let space = " ".repeat(4 * indent);
    match a {
        Ast::Leaf(x) => {
            writeln!(dst, "{space}{x}")
        }
        Ast::Branch(a, b, c) => {
            writeln!(dst, "{space}if func(&v[{a}]) {{")?;
            ast(dst, b, indent + 1)?;
            writeln!(dst, "{space}}} else {{")?;
            ast(dst, c, indent + 1)?;
            writeln!(dst, "{space}}}")
        }
    }
}

fn ast_print<W: Write>(dst: &mut W, a: &Ast, n: usize) -> std::io::Result<()> {
    writeln!(
        dst,
        r"pub fn vpp_aux_{n}<T, F>(v: &[T], func: &mut F) -> usize
where
    T: Clone,
    F: FnMut(&T) -> bool,
{{"
    )?;
    ast(dst, a, 1)?;
    writeln!(dst, "}}\n")
}

fn generate<W: Write>(dst: &mut W, n: usize) -> std::io::Result<()> {
    let v = (0..n).collect::<Vec<_>>();
    let a = veb_layout_ast(&v);
    ast_print(dst, &a, n)
}

fn main() {
    use std::{env, fs::File, io::BufWriter, path::PathBuf};
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/index_rev.rs");
    let out_dir = env::var_os("OUT_DIR").expect("OUT_DIR not set");
    let dst_path = {
        let mut p = PathBuf::from(out_dir);
        p.push("small.rs");
        p
    };
    let mut file = BufWriter::new(File::create(dst_path).expect("file create failed"));
    for n in 1..256 {
        generate(&mut file, n).expect("write failed");
    }
    for d in 9..=13 {
        let n = (1 << d) - 1;
        generate(&mut file, n).expect("write failed");
    }
}
