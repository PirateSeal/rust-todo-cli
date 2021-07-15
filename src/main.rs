use std::env;

struct TodoItem {
    name: String,
    completed: char,
}

impl TodoItem {
    fn new(name: String) -> TodoItem {
        TodoItem {
            name,
            completed: '❌',
        }
    }
}

struct TodoList {
    list: Vec<TodoItem>,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList { list: Vec::new() }
    }

    fn add_to_list(&mut self, name: String) {
        self.list.push(TodoItem::new(name))
    }

    fn print(&self) {
        for (index, item) in self.list.iter().enumerate() {
            println!("{}. {} - {}", index, item.completed, item.name)
        }
    }

    fn mark_done(&mut self, index: usize) {
        if self.list[index].completed == '❌' {
            self.list[index].completed = '✅'
        } else { self.list[index].completed = '❌' }
    }

    fn delete(&mut self, index: usize) {
        self.list.remove(index);
    }
}

enum Command {
    Get,
    Add(String),
    Delete(usize),
    Done(usize),
}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let command = match arguments[1].as_str() {
        "get" => Command::Get,
        "all" => Command::Get,
        "add" => Command::Add(arguments[2].clone()),
        "done" => Command::Done(arguments[2].parse().expect("Error converting String to i32")),
        "finish" => Command::Done(arguments[2].parse().expect("Error converting String to i32")),
        "delete" => Command::Delete(arguments[2].parse().expect("Error converting String to i32")),
        "del" => Command::Delete(arguments[2].parse().expect("Error converting String to i32")),
        _ => panic!("⚠️ Unknown or empty command!")
    };
    let mut todo_list = TodoList::new();

    match command {
        Command::Get => todo_list.print(),
        Command::Add(task) => {
            todo_list.add_to_list(task);
            todo_list.print();
        }
        Command::Done(index) => {
            todo_list.mark_done(index);
            todo_list.print();
        }
        Command::Delete(index) => {
            todo_list.delete(index);
            todo_list.print();
        }
    };
}
