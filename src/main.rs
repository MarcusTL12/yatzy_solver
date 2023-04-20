use dice_throw::DiceThrow;


pub mod yatzy;
pub mod dice_distributions;
pub mod dice_throw;

fn main() {
    let d = DiceThrow::throw(5);

    println!("{d}");
    d.test_all();
    println!("P(d) = {}", d.probability());

    for sd in d.into_sub_throw_iter() {
        println!("{}", sd);
    }
}
