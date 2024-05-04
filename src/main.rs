use proconio::input;

fn main() {
    let cnt = 1000;

    println!("{}", func1());
    println!("{}", func2(100, cnt));
    println!("{}", cnt);
}

fn func1() -> i32 {
    return 2021;
}

fn func2(pos: i32, mut cnt: i32) -> i32 {
    cnt += 1;

    return pos + cnt;
}
