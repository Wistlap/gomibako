use rand::Rng;

extern crate rand;

pub fn add_random_range_1to10(x: i32) -> i32 {
    let rand_num = rand::thread_rng().gen_range(0..11);
    x + rand_num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn may_pass_add_random_1to10() {
        let result = add_random_range_1to10(2);
        assert_eq!((2..12).contains(&result), true);
    }
}
