use std::io; //standard library
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("Secret number: {secret_number}");

    loop {
        println!("Please input your guess.");

        // let to create variable
        // mut to make it mutable
        // String::new() means use the new() associated function with String type
        let mut guess = String::new();

        //&mut guess means mutable reference
        // reference allows you to use variable without copying
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),  
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // exit loop
            }
        }
    }
}
