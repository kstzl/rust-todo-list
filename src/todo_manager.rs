use crate::todo::Todo;

pub struct TodoManager {
    todos: Vec<Todo>
}

impl TodoManager {
    pub fn add_todo(&mut self, name: &str) {
        self.todos.push(Todo {
            name: String::from(name),
            finished: false
        });
    }

    fn todo_exist_at_index(&mut self, index: usize) -> bool {
        index < self.todos.len()
    }

    pub fn remove_todo(&mut self, index: usize) -> bool {
        if self.todo_exist_at_index(index) {
            self.todos.remove(index);
            true
        } else {
            println!("The Todo N°{} does not exist !", index);
            false
        }
    }

    pub fn flip_todo(&mut self, index: usize) -> bool {
        if self.todo_exist_at_index(index) {
            self.todos[index].flip();
            true
        } else {
            println!("The Todo N°{} does not exist !", index);
            false
        }
    }

    pub fn print_todos(&self) {
        if self.todos.len() == 0 {
            println!("No Todos.");
        } else {
            for (i, todo) in self.todos.iter().enumerate() {
                let check_box = match todo.finished {
                    true => "✅",
                    false => "❌",
                };
                println!("[{}] {} - {}", check_box, i, todo.name.to_uppercase());
            }
        }
    }
}

impl Default for TodoManager {
    fn default() -> Self {
        TodoManager { todos: Vec::new() }
    }
}