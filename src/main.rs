use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the Number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut counter: u32 = 1;
    loop{
        println!("Guess Count: {counter}");
        println!("Please input your guess.");
    
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number){
        Ordering::Less => {
            counter += 1;
            println!("Too small!")
        },
        Ordering::Greater => {
            counter += 1;
            println!("Too large!")
        },
        Ordering::Equal => {
            println!("You win! Guesses: {counter}");
            break;            
            }
        }

    
    }    
}
