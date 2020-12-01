use std::io::{self, Write};

use cmd_parser::CommandParser;

fn echo(argv: Vec<&str>) {
    println!("{}", argv[1..].join(" "));
}

fn main() {
    let mut cmdp = CommandParser::new();
    cmdp.insert("echo", echo).expect("This command already exists!");
    cmdp.insert("print", |x| println!("{}", x[1..].concat())).expect("");
    loop {
        print!("> ");
        io::stdout().flush().expect("Unable to flush");
        let mut cmd_buf = String::new();
        std::io::stdin().read_line(&mut cmd_buf).unwrap();
        if let Err(e) = cmdp.call(&cmd_buf[..]) {
            eprintln!("Failed to execute command: {}", e);
        }
    }
}
