// Bring in standard input/output library
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number !");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");
        // adding mut in the definition of guess makes it mutable 
        // The syntax :: indicates that new is an associated function of the String type
        // In total this line creates a new mutable variable guess that contains an empty string instance
        let mut guess = String::new();

        // The code below could be written as: 
        // io::stdin().read_line(&mut guess).expect("Failed to read line");
        // The newlines make it more readable

        io::stdin()
            .read_line(&mut guess) // passing our mutable string into the read line method. This will append the user input to the string we give it
            .expect("Failed to read line");

        // using & before mut guess indicates that the argument is a reference. This allows the program to access the data in a variable without copying 
        // it into memory multiple times

        // When read_line is called it also returns a value "Result" which is an enum that is either "Ok" or "Err" indicating if the operation was successful
        // Result has a method expect. If Result is Err, then the program will crash and will display the message found within expect 
        // if you do not add the expect method then the compiler will give you a warnign that you are not handling the value. 
        

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, 
            Err(_) => continue // ignore non numerical guesses
        };
        // Using .trim() removes whitespace like \n appearing in the input string 
        // .parse() converts strings to another type 
        // we handle the potential of the user not entering a number using .expect 
        
        

        println!("You Guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal =>  {
                println!("You win!");
                break; // end the game if the user guesses correctly 
            }
        }
    }
}
