use proconio::input;
// use proconio::marker::Chars;

fn main() {
    input! {
        n: i32
    }

    let mut ans = 0;

    for a in 1..n {
        ans += (n-1) / a
    }

    println!("{}", ans)
}

fn eratosthenes(n: usize) -> Vec<usize> {
    let mut table = vec!(true; n+1);
    let mut primes = vec![];
    table[0] = false;
    table[1] = false;
    for i in 2..=(n as f32).sqrt() as usize {
        if table[i] == false {
            continue;
        }
        let mut j = 2*i;
        while j <= n {
            table[j] = false;
            j = j + i;
        }
    }

    for (i, is_prime) in table.into_iter().enumerate(){
        if is_prime { primes.push(i)}
    }

    primes
}

#[test]
fn eratosthenes_test() {
    assert_eq!(eratosthenes(11), vec![2,3,5,7,11]);
    assert_eq!(eratosthenes(12), vec![2,3,5,7,11]);
    assert_eq!(eratosthenes(16), vec![2,3,5,7,11,13]);
}