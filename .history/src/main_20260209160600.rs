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

impl Todo {
    fn run(&args){
        
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    Todo::run(&args);

}
