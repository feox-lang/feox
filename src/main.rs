use feox::eval::Env;
use feox::parser::ParserEnv;
use feox::{eval, parser};
use std::cell::RefCell;
use std::io::{self, Write};
use std::rc::Rc;
use std::{env, fs};

fn run_line(line: &str, env: Rc<RefCell<Env>>, parser_env: Rc<RefCell<ParserEnv>>) {
    if line.trim().is_empty() {
        return;
    }

    let ast = &match parser::parse(line, parser_env) {
        Ok(ast) => ast,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };
    let result = eval::eval(&eval::Expr::Block(ast.clone()), env);
    match result {
        Ok(ok) => println!("{}", ok),
        Err(err) => eprintln!("{}", err),
    }
}

fn run_file(path: &str, env: Rc<RefCell<Env>>, parser_env: Rc<RefCell<ParserEnv>>) {
    let content = fs::read_to_string(path).expect("failed to read file");

    let ast = &match parser::parse(content.as_str(), parser_env) {
        Ok(ast) => ast,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };
    let result = eval::eval(&eval::Expr::Block(ast.clone()), env);
    match result {
        Ok(ok) => println!("{}", ok),
        Err(err) => eprintln!("{}", err),
    }
}

fn repl(env: &Rc<RefCell<Env>>, parser_env: &Rc<RefCell<ParserEnv>>) {
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

        run_line(line, env.clone(), parser_env.clone());
    }
}

fn main() {
    let env = Rc::new(RefCell::new(Env::new()));
    let parser_env = Rc::new(RefCell::new(ParserEnv::new()));

    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => repl(&env, &parser_env),

        2 => run_file(&args[1], env, parser_env),

        _ => {
            eprintln!("Usage: feox [script.feox]");
        }
    }
}
