// The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
// 
// Find the sum of all the primes below two million.

fn main() {
    let sum: u64 = (2..2_000_000u64).filter(|&x| is_prime(x)).fold(0, |a, x| a + x);
    println!("{}", sum);
}

fn is_prime(n: u64) -> bool {
    match n {
        1 => return false,
        2 | 3 => return true,
        _ => {}
    };

    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    for i in (5u64..((n as f64).sqrt() as u64)).step_by(6) {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
    }
    return true
}
