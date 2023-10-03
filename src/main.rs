use std::{env, io};
use std::io::Write;
use std::time::SystemTime;

use rand::prelude::SliceRandom;
use rand::Rng;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mode= args.get(1).map_or("challenge", |arg| arg.as_str());

    match mode.to_lowercase().as_str() {
        "practice" => {
            practice_mode();
        }
        _ => {
            challenge();
        }
    }
}

fn challenge() {

    // Clear screen
    println!("\x1b[2J");
    println!("Control-c to exit.");

    let mut rng = rand::thread_rng();
    let mut x = (1..=26).collect::<Vec<_>>();
    let mut wins = 0;
    let mut results = Vec::new();
    x.shuffle(&mut rng);

    let mut game_time: Option<SystemTime> = None;
    for (key, value) in x.iter().enumerate() {
        let answer = (b'a' - 1 + value) as char;
        let guess_time = SystemTime::now();

        let value_pad = if value < &10 { " " } else { "" };
        let index_pad = if key < 10 {"0"} else {""};

        //                                      Set color Blue and Bold
        print!("#{}{key} of {}. Which letter is \x1b[1;34m{}{}\x1b[0m: ", index_pad, x.len(), value_pad, value);
        io::stdout().flush().unwrap();

        let mut line = String::new();
        while line.trim().is_empty() {
            io::stdin().read_line(&mut line).expect("Unable to read line");
        }

        let elapsed = guess_time.elapsed().unwrap().as_secs_f64();
        let message = if line.trim().to_lowercase() == answer.to_string() {
            wins +=1;
            //Set color Green
            "👍 \x1b[32mCorrect!\x1b[0m"
        } else {
            //Set color red
            "👺 \x1b[1;31mWrong!\x1b[0m  "
        };

        if game_time.is_none() {
            game_time = Some(SystemTime::now());
        }

        //Move up a line and over 33 columns
        println!("\x1b[F\x1b[33G {message} answer: {answer} time: {elapsed}");

        results.push((value, elapsed));
    }

    let elapsed = game_time.expect("game_time should be set by now.").elapsed().unwrap().as_secs_f64();
    println!("✅ Done. {wins}/26 Total time: {elapsed}");
    println!("{:?}", results);
}

fn practice_mode()
{
    let mut rng = rand::thread_rng();

    let alphabet: Vec<_> = ('a'..='z').collect();
    let mut n1: usize = 0;
    let mut won = true;
    loop {
        let prev = n1;
        while won && prev == n1 {
            n1 = rng.gen_range(0..alphabet.len());
        }

        println!("Which letter is: {}", n1 + 1);
        let now = SystemTime::now();
        let mut line = String::new();
        while line.trim().is_empty() {
            io::stdin().read_line(&mut line).expect("Unable to read line");
        }

        let elapsed = now.elapsed().unwrap().as_secs_f64();
        won = line.trim().chars().collect::<Vec<_>>()[0] == alphabet[n1];
        if won {
            println!("Correct {}", elapsed);
        } else {
            // println!("Incorrect it was {}", alphabet[n1]);
            println!("Incorrect guess again.");
        }
    }
}
