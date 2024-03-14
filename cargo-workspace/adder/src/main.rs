extern crate add_one;
extern crate add_two;
extern crate add_random_1to10;
fn main() {
    let num = 10;
    // こんにちは世界！{}+1は{}!
    println!("{} plus one is {}!", num, add_one::add_one(num));
    println!("{} plus two is {}!", num, add_two::add_two(num));
    println!("{} plus random number(1~10) is {}!", num, add_random_1to10::add_random_range_1to10(num));
}
