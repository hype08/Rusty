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

struct TodoList {
    list: Vec<TodoItem>,
}

impl TodoList {
    fn new() -> TodoList {
        return TodoList { list: Vec::new() };
    }

    fn add_to_list(&mut self, name: String) {
        let todo_item = TodoItem::new(name);
        self.list.push(todo_item);
    }

    fn print(&self) {
        for item in &self.list {
            println!("[{}] - {}", item.completed, item.name);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let mut todo_list = TodoList::new();

    todo_list.add_to_list("Say hi".to_string());

    if command == "get" {
        todo_list.print();
    }
}
