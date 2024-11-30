use std::io::Write;
use crate::utils::structs::Todo;
use crate::utils::question::ask;

pub fn edit_todo(input: &mut String, datas: &mut [Todo]) {
    if datas.is_empty() {
        println!("No data is found yet");
        return
    }

    print!("Enter the todo task to find: ");
    std::io::stdout().flush().unwrap();
    input.clear();
    std::io::stdin().read_line(input).unwrap();
    let old_title = input.trim().to_string();

    for data in datas.iter_mut() {
        if data.title == old_title {
            println!(
                "Title: {}\nDetails: {}\nPriority: {}\nDay: {}\nDate: {}",
                data.title,
                data.details,
                data.priority,
                data.day,
                data.date,
            );

            ask(&mut "title".to_string(), input);
            let new_title = input.trim();
            data.title = new_title.to_string();
        
            ask(&mut "details".to_string(), input);
            let new_details = input.trim();
            data.details = new_details.to_string();
        
            ask(&mut "priority - [high/medium/low]".to_string(), input);
            let new_priority = input.trim();
            data.priority = new_priority.to_string();
        
            ask(&mut "day".to_string(), input);
            let new_day = input.trim().to_string();
            data.day = new_day.to_string();
        
            ask(&mut "date".to_string(), input);
            if let Ok(new_date) = input.trim().parse::<u32>(){
                data.date = new_date;
            }
        }
    }
    println!("Data with the title {} is not found", old_title);
}