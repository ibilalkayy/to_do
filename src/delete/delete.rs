use std::io::Write;
use crate::utils::structs::Todo;

pub fn delete_todo(input: &mut String, tasks: &mut Vec<Todo>) {
    if tasks.is_empty() {
        println!("No tasks are available.");
        return;
    }

    print!("Enter the title of the task to delete: ");
    std::io::stdout().flush().unwrap();
    input.clear();
    std::io::stdin().read_line(input).unwrap();
    let title_to_delete = input.trim();

    if let Some(pos) = tasks.iter().position(|task| task.title == title_to_delete) {
        let removed_task = tasks.remove(pos);
        println!("Deleted task: {}", removed_task.title);
    } else {
        println!("Task not found. Please ensure the title is correct.");
    }
}
