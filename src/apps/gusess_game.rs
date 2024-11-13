use rand::Rng;
use std::cmp::Ordering;
use std::io;

/// Guess the number Game!
///
/// This is a simple console-based number guessing game. The secret number
/// is randomly generated and lies between 1 and 100. The user is asked to
/// input their guess. The program then compares the guess with the secret number
/// and prints out whether the guess is too small or too big. The game continues
/// until the user guesses the secret number correctly.
fn guessing_game() {
    println!("================= Guess the number Game! =================");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        // the secret number
        // user input
        let mut guess = String::new();
        // read user input
        println!("Please input your guess: ");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // convert string to number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        print!("You guessed: {} \r\n", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    println!("================= ================= =================");
}
