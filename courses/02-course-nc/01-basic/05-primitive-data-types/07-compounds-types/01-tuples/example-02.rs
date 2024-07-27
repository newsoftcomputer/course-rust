
fn main() {

    let wallet_nc: (i32, &str, f64) = (1, "Wallet_1", 100.00);
    let money_send: f64 = 20.5;
    
    
    let result: bool = transaction(wallet_nc, money_send);
    if result {
        println!("The money to send correctly");
    } else {
        println!("Not have money to make this transaction");
    }

}


fn transaction(tuple: (i32, &str, f64), money_send: f64) -> bool {

    if tuple.2 >= money_send {
        true
    } else {
        false
    }
    
}