#[derive(Clone)]
#[derive(Debug)]
pub struct Todo {
    pub title: String,
    pub details: String,
    pub priority: String,
    pub day: String,
    pub date: u32,
}

impl Todo {
    pub fn new() -> Self {
        Todo {
            title: String::new(),
            details: String::new(),
            priority: String::new(),
            day: String::new(),
            date: 0,
        }
    }
}