use crate::utils::structs::Todo;
use crate::utils::question::ask;

pub fn add_todo(todo: &mut Todo, input: &mut String, data: &mut Vec<Todo>) {
    ask(&mut "title".to_string(), input);
    todo.title = input.trim().to_string();

    ask(&mut "details".to_string(), input);
    todo.details = input.trim().to_string();

    ask(&mut "priority - [high/medium/low]".to_string(), input);
    todo.priority = input.trim().to_string();

    ask(&mut "day".to_string(), input);
    todo.day = input.trim().to_string();

    ask(&mut "date".to_string(), input);
    todo.date = input.trim().parse().expect("Expected a number");

    data.push(todo.clone());
    println!("\nYour task is successfully added.");
}