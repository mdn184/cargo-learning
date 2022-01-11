use std::io;

fn main() {
    println!("Guess the number");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed {}", guess);
    let x = 5;
    let y = 10;

    println!("My numbers are {} {}", x, y);
}
