fn main() {
    let result_factorial: u128 = calculate_factorial(25);
    println!("The factorial result from 25 is: {}", result_factorial);
}

fn calculate_factorial(num: u128) -> u128 {
    if num == 0 || num == 1 {
        1
    } else {
        let mut result: u128 = num;
        for i in (1..num).rev() {
            result = result * i;
        }
        return result;
    }
}
