use std::io;
mod survey;
mod toy_problems;

const PROGRAM_OPTIONS: [&str; 2] = ["Survey", "Toy Problems"];

fn main() {
    let mut app_choice = String::new();
    println!("Welcome to my little Rust program!");
    println!("Which app would you like to go to?");
    println!("Your choices are: {:?}", PROGRAM_OPTIONS);

    io::stdin()
        .read_line(&mut app_choice)
        .expect("To enter an app choice");

    let app_choice = app_choice.trim();

    match app_choice {
        "Survey" => survey::survey(),
        "Toy Problems" => toy_problems::toy_problems(),
        _ => {
            println!("{app_choice} is an invalid choice.");
            println!("Aborting...");
        }
    }
}
