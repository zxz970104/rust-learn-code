use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100); // 1..=100：包含上下边界的写法
    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");
        let mut guess = String::new(); // mutable variable; 可变变量
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {// The parse method on strings converts a string to another type
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
