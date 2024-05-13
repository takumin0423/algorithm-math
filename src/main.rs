use proconio::input;

fn main() {
    input! {
        n: usize
    }

    let mut results = vec![2];

    for i in 3..=n {
        if is_prime(i) {
            results.push(i)
        }
    }

    println!(
        "{}",
        results
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(" ")
    );
}

fn is_prime(x: usize) -> bool {
    for i in 2..x {
        if x % i == 0 {
            return false;
        }
    }

    return true;
}
