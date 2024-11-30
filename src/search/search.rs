use std::io::Write;
use crate::utils::structs::Todo;

pub fn search_todo(input: &mut String, datas: &mut [Todo]) {
    if datas.is_empty() {
        println!("No data is found yet");
        return
    }

    print!("Enter the todo task to search: ");
    std::io::stdout().flush().unwrap();
    input.clear();
    std::io::stdin().read_line(input).unwrap();
    let old_title = input.trim().to_string();

    for data in datas.iter_mut() {
        if data.title == old_title {
            println!("Data is found. You can find it through the view option") 
        } else {
            println!("Data is not found. Add the data");
        }
    }
}