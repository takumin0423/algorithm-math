use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize
    }

    let results = (2..=n).filter(|&x| is_prime(x)).collect_vec();

    println!("{}", results.iter().join(" "));
}

fn is_prime(x: usize) -> bool {
    (2..x).all(|i| x % i != 0)
}
