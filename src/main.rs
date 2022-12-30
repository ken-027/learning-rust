use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    ex_1();
    ex_2();
    ex_3();
    ex_4();
    ex_5();
    ex_6();
    ex_7();
    ex_8();
    ex_9();

    let celcuis: f64 = convert_temp_f_c(33);
    println!("converted to celcuis: {celcuis}");
}

fn convert_temp_f_c(fahrenheit: u64) -> f64 {
    let celcuis: f64 = fahrenheit as f64 - 32.0;
    celcuis * (5.0/9.0)
}

fn ex_9() {
    let a: [u64; 5] = [10, 20, 30, 40, 50];
    let mut index: usize = 0;

    println!("WHILE LOOP!");
    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    println!("\nFOR LOOP!");
    for el in a {
        println!("the value is: {el}");
    }

    for number in (1..4).rev() {
        println!("{number}...");
    }
}

fn ex_8() {
    let mut number: u64 = 3;

    while number != 0 {
        println!("{number}...");
        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn ex_7() {
    let mut count: u64 = 0;

    'count_up: loop {
        println!("count = {count}");
        let mut remaining: u64 = 10;

        loop {
            println!("remaining = {}", remaining);

            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'count_up;
            }

            remaining -= 1;
        }
        count += 1;
    }

    println!("End count = {}", count)
}

fn ex_6() {
    let mut counter: u32 = 0;

    let result: u32 = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn ex_5() {
    let condition: bool = false;
    let number: u32 = if condition { 4 } else { 5 };

    println!("The number is {number}");
}

fn ex_4() {
    let x = fn_var();
    println!("The x is {x}")
}

fn fn_var() -> u64 {
    let y: u64 = {
        let x: u64 = 3;
        x + 1
    };
    y
}

fn ex_3() {
    let tup: (u32, f64, i32) = (32, 5.2, -3);
    let months: [&str; 5] = ["January", "February", "March", "April", "May"];
    let (x, y, z) = tup;

    println!("tuple is {x}, {y}, {z}");
    println!("tuple in indexing {}", tup.0);
    println!("first month {}", months[0]);
}

fn ex_2() {
    let spaces: &str = "     ";
    const THREEHOURSINSECONDS: i32 = 60 * 60 * 3;
    let space: usize = spaces.len();

    println!("the spaces of variable \"spaces\" is {space}");
    println!("3 hours in seconds {THREEHOURSINSECONDS}");
}

fn ex_1() {
    let x: u32 = 5;

    let x: u32 = x + 10;

    {
        let x: u32 = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

fn guess_game() {
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
