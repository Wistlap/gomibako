use std::{
    fs::File,
    io::{stdout, Write},
    thread,
    time::{self, Duration},
};

use serde::{Deserialize, Serialize};

use tokio;

#[derive(Serialize, Deserialize, Clone)]
struct Member {
    name: String,
    age: u32,
    instrument: String,
}

impl Member {
    fn introduce(&self) {
        let str = format!(
            "{}，{}歳❗ {}担当です❗❗",
            &self.name, &self.age, &self.instrument
        );
        let dur = time::Duration::from_millis(100);
        typewriter(str, dur);
    }
}

fn typewriter(str: String, dur: Duration) {
    let mut stdout = stdout();
    for c in str.chars() {
        print!("{}", c);
        stdout.flush().unwrap();
        thread::sleep(dur);
    }
    println!();
}

#[tokio::main]
async fn main() {
    let file = File::open("members.yml").unwrap();
    let members: Vec<Member> = serde_yaml::from_reader(file).unwrap();
    let mut tasks = Vec::new();
    for (i, member) in members.iter().enumerate() {
        let member = (*member).clone();
        tasks.push(tokio::spawn(async move {
            let sec = time::Duration::from_secs(i.try_into().unwrap());
            thread::sleep(sec);
            member.introduce();
        }));
    }

    for task in tasks {
        task.await.unwrap();
    }
}
