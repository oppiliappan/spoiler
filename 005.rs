// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any
//      remainder.
// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to
// 20?
//
// https://projecteuler.net/problem=5

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
