use proconio::input;
// use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        dice: [(i32, i32);n]
    }
    
    let mut ans = "No";

    let mut count = 0;

    for (d1, d2) in dice {
        if d1 == d2 {
            count += 1;
        }
        else
        {
            count = 0;
        }

        if count == 3 {
            ans = "Yes";
            break;
        }
    }
    
    println!("{}", ans)
}
