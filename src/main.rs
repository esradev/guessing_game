use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("🎮 Welcome to the Guessing Game! 🎮");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut attempts = 0;
    let max_attempts = 7; // Limit attempts to increase challenge
    
    println!("🔢 I have selected a number between 1 and 100. Try to guess it!");
    
    loop {
        if attempts >= max_attempts {
            println!("❌ You've used all your attempts! The correct number was {secret_number}. Try again next time!");
            break;
        }

        println!("⏳ Attempt {}/{} - Please input your guess:", attempts + 1, max_attempts);

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("⚠️ Invalid input! Please enter a number.");
                continue;
            }
        };

        attempts += 1;
        println!("🎯 You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                if secret_number - guess > 20 {
                    println!("❄️ Too cold! Try a much higher number.");
                } else {
                    println!("🔥 Close, but still a bit too low.");
                }
            }
            Ordering::Greater => {
                if guess - secret_number > 20 {
                    println!("❄️ Way too high! Try much lower.");
                } else {
                    println!("🔥 Almost there, but a bit too high.");
                }
            }
            Ordering::Equal => {
                println!("🎉 Congratulations! You guessed the number in {} attempts! 🏆", attempts);
                break;
            }
        }
    }
}