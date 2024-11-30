use crate::utils::structs::Todo;

pub fn view_todo(datas: &mut [Todo]) {
    if datas.is_empty() {
        println!("No data is added yet");
        return
    }
    println!("\nFollowing is the data:\n");
    for (_, data) in datas.iter().enumerate() {
        println!(
            "Title: {}\nDetails: {}\nPriority: {}\nDay: {}\nDate: {}",
            data.title,
            data.details,
            data.priority,
            data.day,
            data.date,
        );        
    }
}