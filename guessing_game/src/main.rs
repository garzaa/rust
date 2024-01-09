use std::io::stdin;

fn main() {
    println!("Guess the nummber");

    println!("Input your guess");

    let mut guess = String::new();

    stdin().read_line(&mut guess).expect("Didn't read line");

    println!("You guessed: {guess}");
}
