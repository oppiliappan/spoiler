// The sum of the squares of the first ten natural numbers is,
// 12 + 22 + ... + 102 = 385
// 
// The square of the sum of the first ten natural numbers is,
// (1 + 2 + ... + 10)2 = 552 = 3025
// 
// Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 − 385 = 2640.
// 
// Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.
// 
// https://projecteuler.net/problem=6
fn main() {
    let sum_of_sq = (1..100).map(|x| x * x).sum::<i64>();
    let sq_of_sum = (1..100).sum::<i64>().pow(2);
    println!("{}", sq_of_sum - sum_of_sq);
}
