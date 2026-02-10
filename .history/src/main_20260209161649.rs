use std::env;

struct Todo {
    id: u32,
    text: String,
    done: bool
}



struct Config {
    action: String,
    value: String
}

impl Config {
    fn build(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("at least one command is needed");
        }
        let action = args[1].clone();
        let value = args[2].clone();

        Config {
            action, value
        }
    }
}

impl Todo {
    fn run(){

    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args);



}
