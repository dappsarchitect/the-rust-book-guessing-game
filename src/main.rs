use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new();

    // io::stdin() is equivalent to std::io::stdin if
    // the use statement is not used on the first line.
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");

    println!("You guess: {guess}");
}
