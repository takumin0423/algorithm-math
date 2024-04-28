use proconio::input;

fn main() {
    input! {
        n: usize
    }

    let mut result = 0;

    for _i in 0..n {
        input! {
            m: usize
        }

        result += m;
    }

    println!("{}", result);
}
