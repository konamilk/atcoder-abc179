use proconio::input;
// use proconio::marker::Chars;

fn main() {
    input!{
        mut s: String
    }

    if s.chars().last().unwrap() == 's' {
        s = s + "es"
    }
    else {
        s = s + "s"
    }

    println!("{}", s)
}
