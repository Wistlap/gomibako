use num::integer::Roots;
use std::thread;
use std::time;
use std::io::{self, Write};

fn factorial(num: i128) -> i128 {
    let mut fac = 1;
    for i in 1..=num {
        fac *= i;
    }

    let sec = time::Duration::from_secs(1);
    thread::sleep(sec);
    return fac;
}

fn sum(num: i32) -> i32 {
    let mut sum = 0;
    for i in 0..=num {
        sum += i;
    }

    let sec = time::Duration::from_secs(1);
    thread::sleep(sec);
    return sum;
}

fn divisor(num: i32) -> Vec<i32> {
    let mut div: Vec<i32> = Vec::new();
    for i in 1..=num.sqrt() {
        if num % i == 0 {
            //div.extend_from_slice(&[i, num/i]);
            div.push(i);
            if num / i != i {
                div.push(num / i);
            }
        }
    }
    div.sort();

    let sec = time::Duration::from_secs(1);
    thread::sleep(sec);
    return div;
}

fn main() {
    print!("input number: ");
    io::stdout().flush().unwrap();
    let mut input_num = String::new();
    io::stdin()
        .read_line(&mut input_num)
        .expect("Faild to read line.");
    let num_i32 = input_num.trim().parse::<i32>().unwrap();
    let sum = sum(num_i32);
    println!("sum: {}", &sum);
    let fac = factorial(num_i32 as i128);
    println!("factorial: {}", &fac);
    let div = divisor(num_i32);
    println!("divisor: {:?}", &div);
}
