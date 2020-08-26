#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        e: usize,
    }
    let mut array = [a, b, c, d, e];
    array.sort();
    println!("{}", (n + array[0] - 1) / array[0] + 4);
}
