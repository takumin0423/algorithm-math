use proconio::input;

fn main() {
    input! {
        n: usize,
        m: [usize; n]
    }

    let sum_m = m.iter().sum::<usize>();

    println!("{}", sum_m % 100);
}
