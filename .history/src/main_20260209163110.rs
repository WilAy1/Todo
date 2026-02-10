use std::env;

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
        let value = if args[1] == "list" {
            None
        } else {
            Some(args[2].clone())
        };

        let action = match args[1].as_str() {
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
    fn run(){

    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args);

    Todo::run()

}
