use crate::dice_distributions::DICE_ORDER_MAP;

mod yatzy;
mod dice_distributions;

fn main() {
    println!("{:?}", DICE_ORDER_MAP.5);
}
