use core::task;
use std::{env, fs::File, io::{ErrorKind, Read}, process::exit};

use serde_json::Value;

struct Todo {
    id: u32,
    text: String,
    done: bool
}

enum Action {
    Add,
    List,
    Done,
    Remove
}

struct Config {
    action: Action,
    value: Option<String>
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            panic!("at least two commands are needed");
        }
        let value = if args[2] == "list" {
            None
        } else {
            Some(args[3].clone())
        };

        let action = match args[2].as_str() {
            "add" => Action::Add,
            "list" => Action::List,
            "done" => Action::Done,
            "remove" => Action::Remove,
            _ => return Err("Invalid action provided")
        };

        Ok(Config {
            action, value
        })
    }
}

impl Todo {
    fn run(action: Action, value: String){

    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|e| {
        eprint!("Error: {}", e);
        exit(0);
    });

    let task_file_result = File::open("todo.json");

    let task_file = match task_file_result {
        Ok(file) => file,
        Err(err) => {
            match err.kind() {
                ErrorKind::NotFound => {
                    match File::create("todo.json") {
                        Ok(fc) => fc,
                        Err (_) => panic!("Problem occured while creating file")
                    }
                },
                _ => panic!("Problem opening the file {err:?}")
            }
        }
    };

    let mut task_file_content = String::new();
    task_file.read_to_string(&mut task_file_content);

    let s_json: Value = serde_json::from_str(&task_file_content);


    let mut tasks: Vec<Todo> = Vec::new();
    

    match config.action {
        Action::Add => {
            if let Some(task_text) = config.value {
                tasks.push(Todo { id: task_text.len() as u32 + 1 , text: task_text.clone(), done: false });
                println!("Added task \"{}\"", task_text);
            }
        },
        Action::Done => {
            // if let Some(task) = config.value {
            //     if let Ok(id) = task.parse()
            // }
        },
        Action::List => {
            for task in tasks {
                println!("Task {}: {} \n Done: {}", task.id, task.text, task.done);
            }
        },
        Action::Remove => {

        }
    }



}
