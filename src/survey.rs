use rand::prelude::*;
use std::io;

struct Person<'a> {
    name: String,
    age: i32,
    job: &'a str,
    hobbies: String,
}

pub fn survey() {
    let mut should_continue = true;
    let names = ["Kyle", "John", "Jake", "Joe", "Jane"];

    while should_continue {
        let name = pick_name(names);
        let age = thread_rng().gen_range(18..=99);
        let mut job = String::new();
        let mut hobbies = String::new();

        println!("Hello, {}!", name);
        println!("This is a short survey to learn more about you.");
        println!("What is your job?");
        io::stdin()
            .read_line(&mut job)
            .expect("to be able to read the line");

        let job = job.trim();

        println!("Ok, you're a(n) {}, awesome!", job);
        println!("Please list out some of your hobbies, separated by a comma.");

        io::stdin()
            .read_line(&mut hobbies)
            .expect("to be able to read hobbies");

        let hobbies = hobbies.trim();
        let hobbies = sanitize_hobbies(&hobbies);

        let person = Person {
            name,
            age,
            job,
            hobbies,
        };

        println!(
            "Thank you for answering the survey, does this information look right to you? (Y/N)"
        );
        println!("Your name is: {}", person.name);
        println!("Your age is: {}", person.age);
        println!("For work, you are a(n): {}", person.job);
        println!("And your hobbies include: {:?}", person.hobbies);

        should_continue = get_survey_response();
    }
}

fn pick_name(names: [&str; 5]) -> String {
    String::from(names[thread_rng().gen_range(0..names.len())])
}

fn sanitize_hobbies(hobbies: &str) -> String {
    hobbies.split(",").collect::<String>()
}

fn get_survey_response() -> bool {
    let mut response = String::new();
    io::stdin()
        .read_line(&mut response)
        .expect("to receive a response");

    let response = response.trim();
    match response {
        "Y" => {
            println!("Thank you!");
            false
        }
        "N" => {
            println!("We're sorry to hear that, trying again..");
            true
        }
        _ => {
            println!("Invalid input.");
            true
        }
    }
}
