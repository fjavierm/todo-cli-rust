use std::env;

struct TodoItem {
    name: String,
    completed: char
}

impl TodoItem {
    fn new(name: String) -> TodoItem {
        return TodoItem {
            name: name,
            completed: ' '
        };
    }
}

struct TodoList {
    list: Vec<TodoItem>
}

impl TodoList {
    fn new () -> TodoList {
        return TodoList {
            list: Vec::new()
        }; // Return and ; can be omitted
    }

    fn add_to_list(&mut self, item: TodoItem) {
        self.list.push(item);
    }

    fn print(&self) {
        for (index, item) in self.list.iter().enumerate() {
            println!("{}. [{}] - {}", index, item.completed, item.name);
        }
    }

    fn change_status(&mut self, index: usize) {
        if self.list[index].completed == ' ' {
            self.list[index].completed = 'x';
        } else {
            self.list[index].completed = ' ';
        }
    }

    fn remove_task(&mut self, index: usize) {
        self.list.remove(index);
    }
}

enum Command {
    Get,
    Add(String),
    Done(usize),
    Remove(usize)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut todo_list = TodoList::new();

    let command = match args[1].as_str() {
        "get" => Command::Get,
        "add" => Command::Add(args[2].clone()),
        "done" => Command::Done(args[2].parse().expect("Error converting to integer")),
        "remove" => Command::Remove(args[2].parse().expect("Error converting to integer")),
        _ => panic!("you must provide an accepted command")
    };

    todo_list.add_to_list(TodoItem::new("Say hi to John".to_string()));
    todo_list.add_to_list(TodoItem::new("Do something with Rust".to_string()));
    todo_list.change_status(1);

    match command {
        Command::Get => todo_list.print(),
        Command::Add(task) => {
            todo_list.add_to_list(TodoItem::new(task.to_string()));
            todo_list.print();
        },
        Command::Done(index) => {
            todo_list.change_status(index);
            todo_list.print();
        },
        Command::Remove(index) => {
            todo_list.remove_task(index);
            todo_list.print();
        }
    }
}
