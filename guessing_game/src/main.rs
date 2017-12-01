use std::io;

fn main() {
    println!("Guess the number!");

    println!("Input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    
    let x: Result<i32, &str> = Err("You dun goofed");
    x.expect("Things went horribly wrong");

    println!("You guessed [{}]", guess);
}
