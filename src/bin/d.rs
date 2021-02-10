use proconio::input;
// use proconio::marker::Chars;

fn main() {
    input!{
        n: i32,
        k: i32,
        lr: [(i32, i32); k]
    }
    
    let mut ans = 0;
    
    println!("{}", ans)
}


pub fn binom_knuth(n: u64, mut k: u64) -> u64 {
    k = std::cmp::min(k, n-k);
    (0..n + 1)
        .rev()
        .zip(1..k + 1)
        .fold(1, |mut r, (n, d)| { r *= n; r /= d; r })
}
