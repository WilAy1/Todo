use std::{env, fs::{File, OpenOptions}, io::{Error, ErrorKind, Read, Seek, SeekFrom, Write}, process::exit};

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
    Remove,
    Empty,
    Search
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
        let value = if args[2] == "list" || args[2] == "empty"  {
            None
        } else {
            Some(args[3].clone())
        };

        let action = match args[2].as_str() {
            "add" => Action::Add,
            "list" => Action::List,
            "done" => Action::Done,
            "remove" => Action::Remove,
            "empty" => Action::Empty,
            "search" => Action::Search,
            _ => return Err("Invalid action provided")
        };

        Ok(Config {
            action, value
        })
    }
}

// impl Todo {
//     fn run(action: Action, value: String){

//     }
// }

struct FileData {
    file: File,
    content: Value
}

fn load_file() -> Result<FileData, Error> {
        let task_file_result = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("todo.json");

    let mut task_file = match task_file_result {
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
    task_file.read_to_string(&mut task_file_content)?;

    if task_file_content.trim().is_empty() {
        task_file_content = "[]".to_string();
    }


    let s_json: Value = serde_json::from_str(&task_file_content)?;

    Ok(FileData { file: task_file, content: s_json })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|e| {
        eprint!("Error: {}", e);
        exit(0);
    });

    let mut file_data = load_file().unwrap();     

    match config.action {
        Action::Add => {
            if let Some(task_text) = config.value {
                let next_id = if let Value::Array(list) = &mut file_data.content {
                    match list.last() {
                        Some(task) => {
                            task["id"].as_u64()
                            .map(|id| id as u32 + 1)
                            .unwrap_or(1)
                        }
                        None => 1
                     }
                } else {
                    1
                };
                let todo_task = Todo { id: next_id , text: task_text.clone(), done: false };
                let json_data = format!(r#"{{"id": {}, "text": "{}", "done": {} }}"#,
                    todo_task.id, todo_task.text, todo_task.done
                );

                if let Value::Array(list) = &mut file_data.content {
                    list.push(serde_json::from_str(&json_data).unwrap());
                }

                let pretty_json = serde_json::to_string_pretty(&file_data.content).expect("Failed to serialize json");

                file_data.file.set_len(0).unwrap();
                file_data.file.seek(SeekFrom::Start(0)).unwrap();
                file_data.file.write_all(pretty_json.as_bytes()).unwrap();
                file_data.file.flush().unwrap();


                println!("Added task \"{}\"", task_text);
            }
        },
        Action::Done => {
            if let Some(id_str) = config.value {
                let id: u32 = match id_str.trim().parse() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Invalid task id");
                        return;
                    }
                };
            
                let mut found = false;
            
                if let Value::Array(list) = &mut file_data.content {
                    for task in list.iter_mut() {
                        if task["id"].as_u64() == Some(id as u64) {
                            task["done"] = Value::Bool(true);
                            found = true;
                            break;
                        }
                    }
                }
            
                if !found {
                    println!("No task found with id {}", id);
                    return;
                }
            
                let pretty_json =
                    serde_json::to_string_pretty(&file_data.content)
                        .expect("Failed to serialize json");
            
                file_data.file.set_len(0).unwrap();
                file_data.file.seek(SeekFrom::Start(0)).unwrap();
                file_data.file.write_all(pretty_json.as_bytes()).unwrap();
                file_data.file.flush().unwrap();
            
                println!("✓ Task {} marked as done", id);
            }
        },

        Action::List => {
            if let Value::Array(list) = &mut file_data.content {
                if list.is_empty() {
                    println!("No tasks yet ✨\nAdd one with: todo add \"your task\"");
                    return;
                }
            
                println!("Pending\n-------");
                let mut has_pending = false;
            
                for task in list.iter() {
                    if task["done"].as_bool() == Some(false) {
                        has_pending = true;
                        println!("[ ] {} {}", task["id"], task["text"]);
                    }
                }
            
                if !has_pending {
                    println!("(none)");
                }
            
                println!("\nDone\n----");
                let mut has_done = false;
            
                for task in list.iter() {
                    if task["done"].as_bool() == Some(true) {
                        has_done = true;
                        println!("[✓] {} {}", task["id"], task["text"]);
                    }
                }
            
                if !has_done {
                    println!("(none)");
                }
            }


        },
        Action::Remove => {
            if let Some(id_str) = config.value {
                let id: u32 = match id_str.trim().parse() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Invalid task id");
                        return;
                    }
                };
            

                if let Value::Array(list) = &mut file_data.content {
                    // find task in list, remove task from list if it exists
                    let index = &list.iter().position(|task| (task["id"].as_u64() == Some(id as u64)));
                    if index.is_some() {
                        list.remove(index.unwrap());
                    
                        let pretty_json = serde_json::to_string_pretty(&file_data.content)
                            .expect("Failed to serialize json");
                        
                        file_data.file.set_len(0).unwrap();
                        file_data.file.seek(SeekFrom::Start(0)).unwrap();
                        file_data.file.write_all(pretty_json.as_bytes()).unwrap();
                        file_data.file.flush().unwrap();

                        println!("Todo {} successfully remmoved", id);
                        return;
                    }

                    println!("Invalid todo ID");
                }
            }
            
        },
        Action::Empty => {
            // file_data.file.set_len(0).expect("Unable to truncate file");
            // file_data.file.seek(SeekFrom::Start(0)).expect("Unable to seek to start");
        },
        Action::Search => {
            if let Some(query) = config.value {
                if let Value::Array(list) = &mut file_data.content {

                    let mut tasks: Vec<String> = Vec:: new();
                    for task in list {
                        if task["text"].to_string().contains(&query) {
                            tasks.push(task["text"].to_string());
                        }
                    }

                    if tasks.len() > 0 {

                    } else {
                        eprintln!("No task found with \"{}\"", query);
                    }
                }
            }
        }
    }



}
