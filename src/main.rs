use proconio::input;

fn main() {
    println!("{}", func1(1000000.0));
    println!("{}", 16f64.log2());
}

fn func1(n: f64) -> f64 {
    n.log10()
}
