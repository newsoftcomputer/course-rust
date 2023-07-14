
fn main() {
    enum WebEvent {
        // An enum variant can be like a unit struct without fields or data types
        WELoad,
        // An enum variant can be like a tuple struct with data types but no named fields
        WEKeys(String, char),
        // An enum variant can be like a classic struct with named fields and their data types
        WEClick { x: i64, y: i64 }
    }

    
}