use std::io::stdin;
use rand::Rng;
use cli_table::{Cell, Style, Table, WithTitle, format::Justify};

#[derive(Table)]
struct Todo {
    #[table(title = "ID", justify = "Justify::Left")]
    id: i32,

    #[table(title = "Todo")]
    todo: String,

    #[table(title = "Completed", justify = "Justify::Right")]
    completed: bool
}

impl Todo {
    fn new(todo: String) -> Todo {
        let mut rng = rand::thread_rng();

        return Todo {
            id: rng.gen_range(1..50),
            todo,
            completed: false
        }
    }
}

fn main() {
    let mut todos: Vec<Todo> = vec![];
    let mut input = String::new();

    println!("Welcome to TODO Manager!");

    loop {
        println!("What would you like to do? ");

        input = String::from("");
        stdin().read_line(&mut input)
            .ok()
            .expect("Could not read input");

        match input.as_str().trim() {
            "view todos" => {
                println!("{}", todos
                    .with_title()
                    .display()
                    .unwrap()
                );

                continue
            }
            "add todo" => {
                println!("What do you need to do? ");
                input = String::from("");
                stdin().read_line(&mut input)
                    .ok()
                    .expect("Could not read input");

                todos.push(Todo::new(input));
                continue;
            }
            "complete todo" => {
                let mut todo_id = String::new();
                stdin().read_line(&mut todo_id)
                    .ok()
                    .expect("Could not read input");

                let id: i32 = todo_id
                    .trim()
                    .parse()
                    .expect("Expect integer");

                todos.retain(|todo| todo.id != id);
            }
            _ => {
                println!("{}", input)
            }
        }
    }
}
