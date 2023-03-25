mod command_node;
mod command_processor;
mod command;
mod todo_manager;
mod todo;
mod tokenizer;

use std::{io};

fn main() {

    let mut todo_manager = todo_manager::TodoManager::default();
    let mut command_processor = command_processor::CommandProcessor {
        commands: Vec::new(),
        todo_manager: &mut todo_manager
    };

    command_processor.register_command("!todos", "Show list of Todos", "", |_, todo_mngr| {
        todo_mngr.print_todos();
    });
    
    command_processor.register_command("!add", "Add a Todo", "s", |args, todo_mngr| {
        todo_mngr.add_todo(args[0].as_string().unwrap().as_str());
    });

    command_processor.register_command("!remove", "Remove a Todo", "n", |args, todo_mngr| {
        todo_mngr.remove_todo(args[0].as_number().unwrap());
    });

    command_processor.register_command("!flip", "Flip a Todo", "n", |args, todo_mngr| {
        todo_mngr.flip_todo(args[0].as_number().unwrap());
    });

    command_processor.print_commands();

    loop {
        let mut input_buffer = String::new();
        io::stdin().read_line(&mut input_buffer).expect("Input error!");

        let mut splitted: Vec<&str> = input_buffer.trim().split(" ").collect();
        let cmd_name = String::from(splitted[0]);
        splitted.remove(0);

        let cmd = splitted.join(" ");
        let mut tokenizer = tokenizer::Tokenizer {
            iterator: cmd.chars()
        };

        if cmd_name == "!exit" {
            break;
        } else {
            command_processor.exec_by_tokens(&cmd_name, tokenizer.tokenize());
        }

    }
    
}