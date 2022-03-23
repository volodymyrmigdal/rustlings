// quiz1.rs
// This is a quiz for the following sections:
// - Variables
// - Functions
// - If

// Mary is buying apples. One apple usually costs 2 Rustbucks, but if you buy
// more than 40 at once, each apple only costs 1! Write a function that calculates
// the price of an order of apples given the quantity bought. No hints this time!

// Put your function here!
const NORMAL_PRICE: usize = 2;
const DISCOUT_PRICE: usize = 1;
const MIN_Q_FOR_DISCOUNT: usize = 41;

fn calculate_apple_price(apples: usize) -> usize {
    if apples < MIN_Q_FOR_DISCOUNT {
        apples * NORMAL_PRICE
    } else {
        apples * DISCOUT_PRICE
    }
}

// Don't modify this function!
#[test]
fn verify_test() {
    assert_eq!(calculate_apple_price(35), 70);
    assert_eq!(calculate_apple_price(40), 80);
    assert_eq!(calculate_apple_price(65), 65);
}
