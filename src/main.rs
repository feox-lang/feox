use std::cell::RefCell;
use std::io::{self, Write};
use std::rc::Rc;
use std::{env, fs};
use feox::{eval, parser};
use feox::eval::Env;
use feox::parser::ParserEnv;

fn run_line(line: &str, env: Rc<RefCell<Env>>, parser_env: Rc<RefCell<ParserEnv>>) { 
    if line.trim().is_empty() {
        return;
    }

    let ast = &parser::parse((line.to_string()).as_str(), parser_env)[0];
    let result = eval::eval(ast, env);
    println!("{:#?}", result);
}

fn run_file(path: &str, env: Rc<RefCell<Env>>,  parser_env: Rc<RefCell<ParserEnv>>) {
    let content = fs::read_to_string(path)
        .expect("failed to read file");

    let ast = &parser::parse(content.as_str(), parser_env);
    let result = eval::eval(&eval::Expr::Block(ast.clone()), env);
    println!("{:#?}", result);
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