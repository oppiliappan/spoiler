// A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
// a2 + b2 = c2
// 
// For example, 32 + 42 = 9 + 16 = 25 = 52.
// 
// There exists exactly one Pythagorean triplet for which a + b + c = 1000.
// Find the product abc.

fn main() {
    let mut solution = (0, 0, 0);
    for a in 1..1001 {
        for b in a+1..501 {
            let c = 1000 - a - b;
            if a * a + b * b == c * c {
                solution = (a, b, c);
            }
        }
    }
    println!("{}, {}, {}", solution.0, solution.1, solution.2);
}
