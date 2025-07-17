// To obtain user input and print outputs, use the input/output library from the standard  library.
use std::io;

// To generate random numbers, use the Rng trait from the rand library.
use rand::Rng;

// Define the main function with no parameters and the following body:
fn main() {
	// Use the println! macro to print strings to introduce the game
	println!("Guess the number!");

	// Define a new variable called secret_number
	// Bind secret_number to the output of the following:
		// call rand::thread_rng to specify the random number generation—current thread, seeded by operating system.
		// call rand::gen_range to generate a random number within the argument range. 
	let secret_number = rand::thread_rng().gen_range(1..=100);

	println!("The secret number is: {secret_number}");

	println!("Please input your guess.");

	// Define a mutable (the value can change) variable called guess
	// Bind guess to 'new', an associated function of the String type from the standard library.
	// i.e. 'guess' is a variable starting as an empty string.
	let mut guess = String::new();
	
	// Call the stdin function from std:io	
	io::stdin()
		// Call the .read_line method to take the user's standard input and append it to a string.
		// By passing a mutable reference ('&mut') to guess as the argument, tell .read_line which string to append to.
		.read_line(&mut guess)
		// In addition to the string value, the .read_line method will return an enum Result about its success.
		// Here, we should handle errors. In this case, the .expect method will simply crash the program.
		.expect("Failed to read line");

	// Use the println! macro to print the user's guess.
	// This println! uses curly brackets and the variable name 'guess' as a placeholder for the variable's value.
	println!("You guessed: {guess}");
}
