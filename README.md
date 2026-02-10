# Todo
A simple command-line todo app written in Rust.

## Commands

Tasks are stored locally in `~/todo.json`.
Tasks are referenced by the ID shown in `todo list`.

```bash
todo add <text>     # Add a new task
todo list           # List all tasks
todo done <id>...   # Mark one or more tasks as done
todo remove <id>    # Remove a task
todo search <query> # Search tasks
```

## Running
```bash
cargo run -- <command>
```

## Example
```bash
$ cargo run -- todo list
1 [ ] Buy milk
2 [x] Finish README
```
