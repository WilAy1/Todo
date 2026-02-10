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

}


fn main() {
    println!("Hello, world!");
}
