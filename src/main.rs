use proconio::input;

fn main() {
    input! {
        n: usize
    }

    let mut result = 1;

    for i in 2..=n {
        result *= i;
    }

    println!("{}", result);
}
