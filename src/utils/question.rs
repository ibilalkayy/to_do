use std::io::Write;

pub fn ask(todo: &mut String, input: &mut String) {
    print!("Enter the task {}: ", todo);
    std::io::stdout().flush().unwrap();
    input.clear();
    std::io::stdin().read_line(input).unwrap();
}