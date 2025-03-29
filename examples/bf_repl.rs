use std::io::{Write, stdin, stdout};

use libbf_rs::{error::BfErrorable, interprer::Runtime, parser::parse};

fn main() {
    let mut rumtime = Runtime::new();
    let mut line = String::new();

    loop {
        print!("> ");
        stdout().flush().unwrap();

        line.clear();

        stdin().read_line(&mut line).unwrap();

        line.pop();

        let code = parse(&line).inspect_err(|e| println!("{}", BfErrorable(e.clone())));
        if code.is_err() {
            continue;
        }

        _ = rumtime
            .exec(&code.unwrap())
            .inspect_err(|e| println!("{}", BfErrorable(e.clone())));
        stdout().flush().unwrap();
    }
}
