use crate::command::Command;
use crate::todo_manager::TodoManager;
use crate::command_node::CommandNode;

pub struct CommandProcessor<'a> {
    pub commands: Vec<Command>,
    pub todo_manager: &'a mut TodoManager
}

impl CommandProcessor<'_> {

    pub fn print_commands(&self) {
        for cmd in self.commands.iter() {
            cmd.print_pretty();
        }
    }

    pub fn register_command(&mut self, name: &str, description: &str, expect: &str, callback: fn(Vec<CommandNode>, &mut TodoManager)) {
        self.commands.push(Command {
            name: String::from(name),
            description: String::from(description),
            expect: String::from(expect),
            callback: callback
        });
    }

    fn find_command(&mut self, name: &str) -> Option<&Command> {
        self.commands.iter().find(|f| f.name == name)
    }

    pub fn exec_by_tokens(&mut self, name: &str, tokens: Vec<CommandNode>) {

        match self.find_command(name) {
            Some(command) => {
                if command.expect.len() > tokens.len() {
                    println!("Not enough arguments for the command '{name}'.");
                } else if command.expect.len() < tokens.len() {
                    println!("Too many arguments for the command '{name}'.");
                } else {
                    let mut success = true;
                    for (i, token) in tokens.iter().enumerate() {
                        let expected_now = command.expect.chars().nth(i).unwrap().to_string();
                        if expected_now != token.get_name() {
                            success = false;
                            println!("The argument N°{} for the command '{}' must be '{}'.", i, name, expected_now);
                        }
                    }
                    if success {
                        (command.callback)(tokens, self.todo_manager);
                    }
                }
            },
            None => {
                println!("Unknown command.");
            }
        }
    }
}