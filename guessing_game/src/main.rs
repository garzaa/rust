use std::io::stdin;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the nummber");

    println!("Input your guess");

    let secret = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();
        stdin()
            .read_line(&mut guess)
            .expect("Didn't read line");

        // SHADOWING: overwrite a previous variable name, replacing types
        // match expression here
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret){
            // a match expression consists of arms, each one is a pattern and then code that runs if it matches
            Ordering::Less => println!("too low!"),
            Ordering::Greater => println!("too high!"),
            Ordering::Equal => {
                println!("GOT EEM");
                break;
            }
        }
    }
}
