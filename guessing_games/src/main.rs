use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess The Number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut cnt = 0;
    loop {
        println!("Enter your guess: ");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
            
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number.");
                continue;
            }
        };

        println!("You guessed {guess}");
        
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                cnt += 1;
                println!("Too small!");
            },
            Ordering::Greater => {
                cnt += 1;
                println!("Too big!");
            },
            Ordering::Equal => {
                cnt += 1;
                println!("You win!\nYou took {cnt} turns to find the correct number.");
                break;
            }
        }
    }

}
