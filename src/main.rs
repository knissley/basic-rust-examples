use std::io;
mod survey;

const PROGRAM_OPTIONS: [&str; 1] = ["Survey"];

fn main() {
    let mut app_choice = String::new();
    println!("Welcome to my little Rust program!");
    println!("Which app would you like to go to?");
    println!("Your choices are: {:?}", PROGRAM_OPTIONS);

    io::stdin()
        .read_line(&mut app_choice)
        .expect("To enter an app choice");

    let app_choice = app_choice.trim();

    // TODO:: use a better matching system
    if app_choice == "Survey" {
        survey::survey();
    } else {
        println!("Invalid choice, aborting...");
    }
}
