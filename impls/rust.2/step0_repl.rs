use rustyline::error::ReadlineError;
use rustyline::Editor;

const REPL_PREFIX: &str = "user> ";
const HISTORY_FILE_PATH: &str = ".mal_repl_history";

fn main() {
    let mut rl = Editor::<()>::new();
    rl.load_history(HISTORY_FILE_PATH);
    loop {
        let readline = rl.readline(REPL_PREFIX);
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                println!("{}", line);
            },
            Err(ReadlineError::Interrupted) => {
                break
            },
            Err(ReadlineError::Eof) => {
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    rl.save_history(HISTORY_FILE_PATH).unwrap();
}

pub fn rep() {
    todo!("")
}

pub fn read(buff: String) -> String {
    todo!("")
}

pub fn eval() {}

pub fn print(buff: String) {
    println!("{}", buff);
}
