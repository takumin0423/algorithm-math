use proconio::input;

fn main() {
    println!("{}", func1(1.0));
    println!("{}", func1(5.0));
    println!("{}", func1(10.0));
}

fn func1(n: f64) -> f64 {
    n.powi(3)
}
