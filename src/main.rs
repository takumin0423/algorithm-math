use proconio::input;

fn main() {
    input! {
        mut n: usize
    };

    let mut result: String = String::new();

    while n >= 1 {
        if n % 2 == 1 {
            result = String::from("1") + &result;
        } else if n % 2 == 0 {
            result = String::from("0") + &result;
        }

        n /= 2;
    }

    println!("{}", result);
}
