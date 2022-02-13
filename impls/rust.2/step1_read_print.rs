use rustyline::error::ReadlineError;
use rustyline::Editor;

mod printer;
mod reader;

#[macro_use]
#[allow(dead_code)]
mod mal_types;
use mal_types::{MalType};

const REPL_PREFIX: &str = "user> ";
const HISTORY_FILE_PATH: &str = ".mal_repl_history";

fn main() {
    rep();
}

pub fn rep() {
    let mut rl = Editor::<()>::new();
    if rl.load_history(HISTORY_FILE_PATH).is_err() {
        println!("No previous history.");
    }

    loop {
        let readline = rl.readline(REPL_PREFIX);
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                match read(line) {
                    Ok(v) => println!("{}", print(eval(v))),
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

pub fn eval(mal_val: MalType) -> MalType { mal_val }

pub fn print(mal: MalType) -> String {
    printer::print_str(mal)
}
