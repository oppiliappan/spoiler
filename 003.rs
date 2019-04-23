// The prime factors of 13195 are 5, 7, 13 and 29.
// 
// What is the largest prime factor of the number 600851475143 ?

fn main() {
    let mut tool = 2.;
    let mut num = 600851475143.;
    while num > tool {
        if num % tool == 0. {
            num /= tool;
            tool = 2.;
        } else {
            tool += 1.;
        }
    }
    println!("{}", num);
}
