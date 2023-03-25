use crate::command_node::CommandNode;
use crate::todo_manager::TodoManager;

pub struct Command {
    pub name: String,
    pub description: String,
    pub expect: String,
    pub callback: fn(Vec<CommandNode>, &mut TodoManager)
}

impl Command {
    pub fn print_pretty(&self) {
        let mut res = String::new();
        for arg in self.expect.chars() {
            let r = match arg {
                's' => "\"STRING\" ",
                'n' => "NUMBER ",
                _ => "?"
            };
            res.push_str(r);
        }
        println!("{} {}", self.name, res);
        println!("  {}\n", self.description);
    }
}