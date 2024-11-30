mod add;
mod view;
mod utils;
mod edit;
mod search;
mod delete;

use std::io::Write;
use std::process::exit;
use crate::utils::structs::Todo;
use crate::add::add::add_todo;
use crate::view::view::view_todo;
use crate::edit::edit::edit_todo;
use crate::search::search::search_todo;
use crate::delete::delete::delete_todo;

fn main() {
    let mut input: String = String::new();
    let mut todo = Todo::new();
    let mut data = Vec::new();
     
    println!("Welcome to the TODO list");
    loop {
        options(&mut todo, &mut input, &mut data);
    }
}

fn options(todo: &mut Todo, input: &mut String, data: &mut Vec<Todo>) {
    println!("\nChoose one of them");

    println!("\n1. Add Task");
    println!("2. View Tasks");
    println!("3. Edit Task");
    println!("4. Delete Task");
    println!("5. Search Task");
    println!("6. Exit");

    print!("Choose your option: ");
    std::io::stdout().flush().unwrap();
    input.clear();

    std::io::stdin().read_line(input).unwrap();
    let integer = input.trim().parse().expect("Expected a number");

    match integer {
        1 => add_todo(todo, input, data),
        2 => view_todo(data),
        3 => edit_todo(input, data),
        4 => delete_todo(input, data),
        5 => search_todo(input, data),
        6 => {
            println!("Exited");
            exit(0);
        },
        _ => println!("\nInvalid option")
    }
}
