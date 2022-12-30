use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn run() {
    println!("Guest the number!");

    loop {
        println!("Please input your guess.");
        let secret: u32 = rand::thread_rng().gen_range(1..=10);
        let mut guess: String = String::new();
        // let apples = 5;
        // let mut bananas= 5;

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed: {guess} and the result is {secret}");

        match guess.cmp(&secret) {
            Ordering::Less => println!("To small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You guess it!");
                break;
            }
        }
    }
}
