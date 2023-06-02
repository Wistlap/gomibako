use rand;
use rand::seq::SliceRandom;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn main() -> io::Result<()> {
    let file = File::open("aizuchi.txt")?;
    let vocabulary: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();

    println!("こいつ: なに．");
    loop {
        print!("わい　: ");
        io::stdout().flush().unwrap();
        let mut word = String::new();
        io::stdin()
            .read_line(&mut word)
            .expect("Failed to read line.");

        if word.trim() == "あきた" {
            println!("こいつ: じゃあな．");
            break;
        }

        let mut rng = rand::thread_rng();
        println!("こいつ: {}．", vocabulary.choose(&mut rng).unwrap());
    }
    Ok(())
}
