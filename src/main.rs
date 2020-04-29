use std::env;

struct TodoItem {
    name: String,
    completed: char,
}

impl TodoItem {
    fn new(name: String) -> TodoItem {
        return TodoItem {
            name: name,
            completed: ' ',
        };
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let todo_item1 = TodoItem::new("Say hi".to_string());
    let todo_item2 = TodoItem::new("Say bye".to_string());
    let todo_list = vec![todo_item1, todo_item2];

    if command == "get" {
        for item in todo_list {
            println!("[{}] - {}", item.completed, item.name)
        }
    }
}
