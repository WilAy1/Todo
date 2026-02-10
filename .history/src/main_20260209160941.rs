use std::env;

struct Todo {
    id: u32,
    text: String,
    done: bool
}

enum TodoCommands {
    Add(String),
    List,
    Done(u32),
    Remove(u32)
}

struct Config {
    action: String,
    value: String
}

impl Config {
    fn build(args: &[String]) -> Config {
        let action = args[1].clone();
        let value = args[2].clone();

        Config {
            action, value
        }
    }
}

impl Todo {
    fn run(&args){

    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    Todo::run(&args);

}
