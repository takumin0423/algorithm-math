use proconio::input;

fn main() {
    input! {
        n: usize
    }

    let mut add_result = 0;

    for _i in 0..n {
        input! {
            m: usize
        }

        add_result += m;
    }

    println!("{}", add_result % 100);
}
