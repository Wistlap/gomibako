use rand::Rng;

extern crate add_one;
extern crate add_two;
extern crate add_random_1to10;
extern crate rand;


fn main() {
    let rand_num = rand::thread_rng().gen_range(0..1000);
    println!("You got a random number {}!", rand_num);
    // こんにちは世界！{}+1は{}!
    println!("{} plus one is {}!", rand_num, add_one::add_one(rand_num));
    println!("{} plus two is {}!", rand_num, add_two::add_two(rand_num));
    println!("{} plus random rand_number(1~10) is {}!", rand_num, add_random_1to10::add_random_range_1to10(rand_num));
}
