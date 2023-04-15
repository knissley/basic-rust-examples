use std::collections::HashMap;
use std::io;
mod two_sum;

const DIFFICULTIES: [&str; 3] = ["EASY", "MEDIUM", "HARD"];
const CATEGORIES: [&str; 1] = ["Strings"];
const ENTRY_CHOICES: [&str; 1] = ["Name"];
//  "Difficulty", "Category"
struct ToyProblem<'a> {
    name: &'a str,
    difficulty: &'a str,
    category: &'a str,
    description: &'a str,
}

const TOY_PROBLEMS: [ToyProblem; 1] = [ToyProblem {
    name: "Two Sum",
    difficulty: "EASY",
    category: "Strings",
    description: "Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target."
}];

pub fn toy_problems() {
    println!("Welcome to the toy problems module!");

    let toy_problem: Option<ToyProblem> = find_toy_problem();

    if toy_problem.is_none() {
        return;
    }

    let toy_problem = toy_problem.unwrap();
    println!("Found toy problem of name: {}", toy_problem.name);
    println!("{}", toy_problem.description);
    println!(
        "{:?}",
        two_sum::two_sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 10)
    );
}

fn find_toy_problem<'a>() -> Option<ToyProblem<'a>> {
    println!("How would you like to pick a problem?");
    println!("Choices are: {:?}", ENTRY_CHOICES);
    let mut entry_choice = String::new();

    io::stdin()
        .read_line(&mut entry_choice)
        .expect("To read entry choice");

    let toy_problem = match entry_choice.trim() {
        "Name" => find_toy_problem_by_name(),
        _ => None,
    };

    toy_problem
}

fn find_toy_problem_by_name<'a>() -> Option<ToyProblem<'a>> {
    println!("What name would you like to search for?");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("to read in a name.");
    let name = name.trim();

    for toy_problem in TOY_PROBLEMS {
        if toy_problem.name == name {
            return Some(toy_problem);
        }
    }

    println!("No toy problem found for that name.");
    None
}

// fn find_toy_problem_by_difficulty() -> Option<ToyProblem> {
//   println!("What difficulty would you like to narrow down to?")
// }
