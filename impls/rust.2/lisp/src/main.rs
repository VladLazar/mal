use std::io::{self, Write};

mod repl;

fn main() {
    let mut env = repl::env::default_env();

    loop {
        print!("user> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        let bytes_read = io::stdin().read_line(&mut input).unwrap();
        if bytes_read == 0 {
            break;
        }

        println!("{}", repl::rep(input, &mut env));
    }
}

