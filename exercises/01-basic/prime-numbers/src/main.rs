fn main() {
    let result_prime: bool = prime_numbers(17);
    println!("The number 17 is prime: {}", result_prime);
}

fn prime_numbers(num: u128) -> bool {
    let mut is_prime: bool = true;
    let number: f64 = num as f64;

    if num > 1 {
        for i in 2..((number.sqrt() as u128) + 1) {
            if (num as u128) % i == 0 {
                is_prime = false;
                break;
            }
        }
    }

    is_prime
}
