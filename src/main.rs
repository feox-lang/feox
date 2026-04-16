use std::cell::RefCell;
use std::io::{self, Write};
use std::rc::Rc;
use std::{env, fs};

use feox::{eval, parser};
use feox::eval::Env;

fn run_line(line: &str, env: &Rc<RefCell<Env>>) { 
    if line.trim().is_empty() {
        return;
    }

    let ast = &parser::parse((line.to_string()).as_str())[0];
    let result = eval::eval(ast, env.clone());
    println!("{:#?}", result);
}

fn run_file(path: &str, env: &Rc<RefCell<Env>>) {
    let content = fs::read_to_string(path)
        .expect("failed to read file");

    let ast = &parser::parse(content.as_str());
    let result = eval::eval(&eval::Expr::Block(ast.clone()), env.clone());
    println!("{:#?}", result);
}

fn repl(env: &Rc<RefCell<Env>>) {
    println!("Feox REPL (type 'exit' to quit)");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        let line = line.trim();

        if line == "exit" {
            break;
        }

        run_line(line, env);
    }
}

fn main() {
    let env = Rc::new(RefCell::new(Env::new()));

    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => repl(&env),

        2 => run_file(&args[1], &env),

        _ => {
            eprintln!("Usage: feox [script.feox]");
        }
    }
}