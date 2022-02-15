use crate::MalSeq::MHashMap;
use crate::MalSeq::Vector;
use crate::mal_types::MalError;
use crate::mal_types::MalResult;
use crate::MalAtom::Func;
use crate::MalAtom::Number;
use crate::MalAtom::Symbol;
use crate::MalSeq::List;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::collections::HashMap;

mod printer;
mod reader;

mod mal_types;
use mal_types::{MalAtom, MalSeq, MalType};

const REPL_PREFIX: &str = "user> ";
const HISTORY_FILE_PATH: &str = ".mal_repl_history";

type ReplEnv = HashMap<String, mal_types::MalFunc>;

fn main() {
    rep();
}

pub fn rep() {
    let mut rl = Editor::<()>::new();
    if rl.load_history(HISTORY_FILE_PATH).is_err() {
        println!("No previous history.");
    }

    let mut repl_env: ReplEnv = HashMap::new();
    repl_env.insert("+".to_string(), |args| match args {
        MalType::MalSeq(List(args_list)) => {
            let zero = Ok(MalType::MalAtom(Number(0)));
            args_list
                .into_iter()
                .fold(zero, |acc, mal_num| match mal_num {
                    MalType::MalAtom(Number(n)) => match acc {
                        Ok(MalType::MalAtom(Number(sumed))) => Ok(MalType::MalAtom(Number(sumed + n))),
                        Err(e) => Err(MalError::TypeError(format!("{:?}", e))),
                        _ => Err(MalError::TypeError(format!("{:?}", acc))),
                    },
                    _ => Err(MalError::TypeError(format!("{:?}", acc))),
                })
        }
        _ => Err(MalError::TypeError(format!("{:?}", args))),
    });

    repl_env.insert("-".to_string(), |args| match args {
        MalType::MalSeq(List(args_list)) => {
            let first_arg = args_list.get(0).ok_or(format!("{:?}", args_list));
            let zero = match first_arg {
                Ok(MalType::MalAtom(Number(n))) => Ok(MalType::MalAtom(Number(n + n))),
                Err(s) => Err(MalError::TypeError(s)),
                _ => Err(MalError::TypeError(format!("{:?}", first_arg))),
            };
            args_list
                .into_iter()
                .fold(zero, |acc, mal_num| match mal_num {
                    MalType::MalAtom(Number(n)) => match acc {
                        Ok(MalType::MalAtom(Number(sumed))) => Ok(MalType::MalAtom(Number(sumed - n))),
                        Err(e) => Err(MalError::TypeError(format!("{:?}", e))),
                        _ => Err(MalError::TypeError(format!("{:?}", acc))),
                    },
                    _ => Err(MalError::TypeError(format!("{:?}", acc))),
                })
        }
        _ => Err(MalError::TypeError(format!("{:?}", args))),
    });

    repl_env.insert("*".to_string(), |args| match args {
        MalType::MalSeq(List(args_list)) => {
            let zero = Ok(MalType::MalAtom(Number(1)));
            args_list
                .into_iter()
                .fold(zero, |acc, mal_num| match mal_num {
                    MalType::MalAtom(Number(n)) => match acc {
                        Ok(MalType::MalAtom(Number(sumed))) => Ok(MalType::MalAtom(Number(sumed * n))),
                        Err(e) => Err(MalError::TypeError(format!("{:?}", e))),
                        _ => Err(MalError::TypeError(format!("{:?}", acc))),
                    },
                    _ => Err(MalError::TypeError(format!("{:?}", acc))),
                })
        }
        _ => Err(MalError::TypeError(format!("{:?}", args))),
    });

    repl_env.insert("/".to_string(), |args| match args {
        MalType::MalSeq(List(ref args_list)) => {
            let first_arg = args_list.get(0).ok_or(format!("{:?}", args_list));
            let zero = match first_arg {
                Ok(MalType::MalAtom(Number(n))) => Ok(MalType::MalAtom(Number(n + n))),
                Err(s) => Err(MalError::TypeError(s)),
                _ => Err(MalError::TypeError(format!("{:?}", first_arg))),
            };
            args_list
                .into_iter()
                .fold(zero, |acc, mal_num| match mal_num {
                    MalType::MalAtom(Number(n)) => match acc {
                        Ok(MalType::MalAtom(Number(sumed))) => Ok(MalType::MalAtom(Number(sumed / n))),
                        Err(e) => Err(MalError::TypeError(format!("{:?}", e))),
                        _ => Err(MalError::TypeError(format!("{:?}", acc))),
                    },
                    _ => Err(MalError::TypeError(format!("{:?}", args))),
                })
        }
        _ => Err(MalError::TypeError(format!("{:?}", args))),
    });

    loop {
        let readline = rl.readline(REPL_PREFIX);
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                match read(line) {
                    Ok(v) => println!("{}", print(eval(Ok(v), &repl_env))),
                    Err(e) => println!("{}", e),
                }
            }
            Err(ReadlineError::Interrupted) => break,
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history(HISTORY_FILE_PATH).unwrap();
}

pub fn read(input: String) -> Result<MalType, String> {
    reader::read_str(input)
}

fn eval(mal_val: MalResult, repl_env: &ReplEnv) -> MalResult {
    match &mal_val {
        Ok(MalType::MalSeq(List(list))) => {
            if list.len() == 0 {
                mal_val
            } else {
                let evaluated_list = eval_ast(mal_val, repl_env);
                match evaluated_list {
                    Ok(MalType::MalSeq(List(list))) => {
                        let first = list.iter().take(1).next().unwrap().clone();
                        let args: Vec<MalType> = list.into_iter().skip(1).collect();
                        match first {
                            MalType::MalAtom(Func(f)) => f(MalType::MalSeq(List(args))),
                            _ => Err(MalError::TypeError(format!("Expected a function got {:?}", first))),
                        }
                    }
                    _ => Err(MalError::TypeError(format!("Expected a List got {:?}", evaluated_list))),
                }
            }
        }
        _ => eval_ast(mal_val, repl_env),
    }
}

fn eval_ast(mal_val: MalResult, repl_env: &HashMap<String, mal_types::MalFunc>) -> MalResult {
    match mal_val {
        Ok(MalType::MalAtom(Symbol(v))) => match repl_env.get(&v) {
            Some(f) => Ok(MalType::MalAtom(Func(*f))),
            None => Err(MalError::UnknownSymbol(format!("Unknown symbol - {}", v))),
        },
        Ok(MalType::MalSeq(List(mal_list))) => {
            let res: Result<Vec<MalType>, MalError> = mal_list
                .into_iter()
                .map(|v| eval(Ok(v), repl_env))
                .collect();
            res.and_then(|v| Ok(MalType::MalSeq(List(v))))
        },
        Ok(MalType::MalSeq(Vector(mal_list))) => {
            let res: Result<Vec<MalType>, MalError> = mal_list
                .into_iter()
                .map(|v| eval(Ok(v), repl_env))
                .collect();
            res.and_then(|v| Ok(MalType::MalSeq(Vector(v))))
        },
        Ok(MalType::MalSeq(MHashMap(mal_list))) => {
            let res: Result<Vec<MalType>, MalError> = mal_list
                .into_iter()
                .map(|v| eval(Ok(v), repl_env))
                .collect();
            res.and_then(|v| Ok(MalType::MalSeq(MHashMap(v))))
        },
        _ => mal_val,
    }
}

pub fn print(mal: MalResult) -> String {
    match mal {
        Ok(v) => printer::print_str(v),
        Err(e) => format!("{}", e),
    }
}
