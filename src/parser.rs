use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use pest::iterators::{Pairs, Pair};
use pest::{Parser};
use pest_derive::Parser;
use crate::ast::{BinOp, Expr, UnaryOp};

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct FeoxParser;

type ParserEnvRef = Rc<RefCell<ParserEnv>>;

pub struct ParserEnv {
    pub vars: HashMap<String, usize>,
    pub counter: usize
}

impl ParserEnv {
    
    pub fn new() -> ParserEnv {
        ParserEnv {
            vars: HashMap::new(),
            counter: 0
        }
    }
    pub fn idx(&mut self, name: &str) -> usize {
        if let Some(idx ) = self.vars.get(name) {
            *idx
        } else {
            let cur = self.counter;
            self.counter += 1;
            self.vars.insert(name.to_string(), cur);
            cur
        }
    }
}

pub fn parse(source: &str, env: ParserEnvRef) -> Vec<Expr> {
    let pairs = FeoxParser::parse(Rule::program, source).unwrap();
    parse_program(pairs, env)
}

fn parse_program(pairs: Pairs<Rule>, env: ParserEnvRef) -> Vec<Expr> {
    pairs
        .filter(|p| p.as_rule() != Rule::EOI)
        .map(|p| parse_expr(p, env.clone()))
        .collect()
}

fn parse_expr(pair: Pair<Rule>, env: ParserEnvRef) -> Expr {
    match pair.as_rule() {
        Rule::assign => parse_assign(pair, env),
        Rule::lambda => parse_lambda(pair, env),
        Rule::if_ => parse_if(pair, env),
        Rule::while_ => parse_while(pair, env),
        Rule::for_ => parse_for(pair, env),
        Rule::mod_ => parse_mod(pair,env),
        Rule::return_ => parse_return(pair, env),
        Rule::break_ => Expr::Break,
        Rule::continue_ => Expr::Continue,

        Rule::or
        | Rule::and
        | Rule::xor
        | Rule::cmp
        | Rule::range
        | Rule::add
        | Rule::mul
        | Rule::pow => parse_binary_chain(pair, env),

        Rule::unary => parse_unary(pair, env),
        Rule::postfix => parse_postfix(pair, env),
        Rule::primary => parse_primary(pair, env),

        _ => unreachable!("{:?}", pair.as_rule()),
    }
}

fn parse_postfix(pair: Pair<Rule>, env: ParserEnvRef) -> Expr {
    let mut inner = pair.into_inner();

    let mut expr = parse_primary(inner.next().unwrap(), env.clone());

    for p in inner {
        match p.as_rule() {
            Rule::call => {
                let args = p
                    .into_inner()
                    .map(|p| parse_expr(p, env.clone()))
                    .collect();

                expr = Expr::Call {
                    func: Box::new(expr),
                    args,
                };
            }

            Rule::index => {
                let idx = parse_expr(p.into_inner().next().unwrap(), env.clone());

                expr = Expr::Index {
                    object: Box::new(expr),
                    index: Box::new(idx),
                };
            }

            Rule::method_call => {
                let mut mc = p.into_inner();
                let method = mc.next().unwrap().as_str().to_string();
                let args = mc.map(|p| parse_expr(p, env.clone())).collect();

                expr = Expr::Call {
                    func: Box::new(Expr::Index {
                        object: Box::new(expr),
                        index: Box::new(Expr::String(method)),
                    }),
                    args,
                };
            }

            _ => unreachable!(),
        }
    }

    expr
}

fn parse_mod(pair: Pair<Rule>, env: ParserEnvRef) -> Expr {
    let mut inner = pair.into_inner();

    let modulus =  parse_expr(inner.next().unwrap(), env.clone());
    let block = parse_block(inner.next().unwrap(), env.clone());

    Expr::Mod {
        modulus: Box::new(modulus),
        body: Box::new(block),
    }
}

fn parse_return(pair: Pair<Rule>, env: ParserEnvRef) -> Expr {
    let mut inner = pair.into_inner();

    let value = inner.next().map(|p| parse_expr(p, env));

    Expr::Return(value.map(Box::new))
}
fn parse_assign(pair: Pair<Rule>, env: ParserEnvRef) -> Expr {
    let mut inner = pair.into_inner();

    let name = inner.next().unwrap().as_str().to_string();
    let id = env.borrow_mut().idx(&name);
    let value  = parse_expr(inner.next().unwrap(), env);

    Expr::Assign {id, name, value: Box::new(value) }
}

fn parse_binary_chain(pair: Pair<Rule>, env: ParserEnvRef) -> Expr {
    let mut inner = pair.into_inner();

    let mut expr = parse_expr(inner.next().unwrap(), env.clone());

    while let Some(op) = inner.next() {
        let rhs = parse_expr(inner.next().unwrap(), env.clone());
        if op.as_str() == ".." || op.as_str() == "..=" {
            expr = Expr::Range {start: Box::new(expr), end: Box::new(rhs), inclusive: op.as_str() == "..="}
        } else {
            expr = Expr::BinOp {
                op: match op.as_str() {
                    "+" => BinOp::Add,
                    "-" => BinOp::Sub,
                    "*" => BinOp::Mul,
                    "/" => BinOp::Div,
                    "%" => BinOp::Rem,
                    "&" => BinOp::And,
                    "^" => BinOp::Xor,
                    "|" => BinOp::Or,
                    "**" => BinOp::Pow,
                    "==" => BinOp::Eq,
                    "!=" => BinOp::Neq,
                    "<=" => BinOp::Le,
                    "<" => BinOp::Lt,
                    ">=" => BinOp::Ge,
                    ">" => BinOp::Gt,
                    _ => unreachable!("{}", op.as_str()),
                },
                left: Box::new(expr),
                right: Box::new(rhs),
            };
        }
    }

    expr
}

fn parse_unary(pair: Pair<Rule>, env: ParserEnvRef) -> Expr {
    let mut inner = pair.clone().into_inner();

    if pair.as_str().starts_with('-') || pair.as_str().starts_with('!') {
        let op = pair.as_str().chars().next().unwrap();
        let expr = parse_unary(inner.next().unwrap(), env);

        Expr::UnaryOp {
            op: match op {
                '-' => UnaryOp::Neg,
                '!' => UnaryOp::Not,
                _ => unreachable!(),
            },
            expr: Box::new(expr)
        }
    } else {
        parse_postfix(inner.next().unwrap(), env)
    }
}

fn parse_primary(pair: Pair<Rule>, env: ParserEnvRef) -> Expr {
    let inner = pair.into_inner().next().unwrap();

    match inner.as_rule() {
        Rule::number => {
            let n = inner.as_str().parse::<i64>().unwrap();
            Expr::Number(n)
        }

        Rule::string => {
            let s = inner.as_str();
            Expr::String(s[1..s.len()-1].to_string())
        }

        Rule::bool => Expr::Bool(inner.as_str() == "true"),

        Rule::nil => Expr::Nil,

        Rule::ident => Expr::Ident(*env.borrow().vars.get(inner.as_str()).unwrap()),

        Rule::expr => parse_expr(inner, env),

        Rule::array => parse_array(inner, env),
        Rule::block => parse_block(inner, env),
        Rule::bignum => {
            let s = inner.as_str();
            let (base, exp) = s.split_once('e').unwrap();
            let n = 10_i64.pow( exp.parse::<i64>().unwrap().try_into().unwrap()) * base.parse::<i64>().unwrap();
            Expr::Number(n)
        }


        _ => parse_expr(inner, env)
    }
}

fn parse_array(pair: Pair<Rule>, env: ParserEnvRef) -> Expr {
    let elems = pair
        .into_inner()
        .map(|p| parse_expr(p, env.clone()))
        .collect();

    Expr::Array(elems)
}

fn parse_block(pair: Pair<Rule>, env: ParserEnvRef) -> Expr {
    let stmts = pair
        .into_inner()
        .map(|p| parse_expr(p, env.clone()))
        .collect();

    Expr::Block(stmts)
}

fn parse_if(pair: Pair<Rule>, env: ParserEnvRef) -> Expr {
    let mut inner = pair.into_inner();

    let cond  = parse_expr(inner.next().unwrap(), env.clone());
    let then  = parse_block(inner.next().unwrap(), env.clone());
    let else_ = inner.next().map(|p| parse_block(p, env.clone()));

    Expr::If {
        cond: Box::new(cond),
        then: Box::new(then),
        else_: else_.map(Box::new),
    }
}

fn parse_lambda(pair: Pair<Rule>, env: ParserEnvRef) -> Expr {
    let mut inner = pair.into_inner();

    let args_pair = inner.next().unwrap(); 
    let args = args_pair
        .into_inner()
        .map(|p| (env.borrow_mut().idx(p.as_str()), p.as_str().to_string()))
        .collect();

    let body = parse_expr(inner.next().unwrap(), env);

    Expr::Lambda {
        args,
        body: Box::new(body),
    }
}

fn parse_while(pair: Pair<Rule>, env: ParserEnvRef) -> Expr {
    let mut inner = pair.into_inner();

    let cond = parse_expr(inner.next().unwrap(), env.clone());
    let body = parse_block(inner.next().unwrap(), env.clone());

    Expr::While {
        cond: Box::new(cond),
        body: Box::new(body),
    }
}

fn parse_for(pair: Pair<Rule>, env: ParserEnvRef) -> Expr {
    let mut inner = pair.into_inner();

    let var  = inner.next().unwrap().as_str().to_string();
    let id = env.borrow_mut().idx(var.as_str());
    let iter = parse_expr(inner.next().unwrap(), env.clone());
    let body = parse_block(inner.next().unwrap(), env.clone());

    Expr::For {
        var,
        id,
        iter: Box::new(iter),
        body: Box::new(body),
    }
}
