fn main() {
    let ans = (1..20).fold(1, |l, x| lcm(l, x));
    println!("{}", ans);
}

fn gcd(a: i64, b: i64) -> i64 {
    if a == 0 {
        b.abs()
    } else {
        gcd(b % a, a)
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    (a * b) / gcd(a, b)
}
