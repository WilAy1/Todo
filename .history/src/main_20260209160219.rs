struct Todo {
    id: u32,
    text: String,
    done: bool
}

struct Config {

}

enum TodoCommands {
    Add(String),
    List,
    Done (u32),
    Remove(u32)
}

impl Config {
    fn run(&args) {

    }
}


fn main() {
    println!("Hello, world!");
}
