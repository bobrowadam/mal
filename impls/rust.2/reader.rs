use crate::mal_types::MalAtom::KeyWord;
use crate::mal_types::MalAtom::Symbol;
use crate::mal_types::MalAtom::Number;
use crate::mal_types::MalAtom::MString;
use crate::mal_types::MalAtom::WithMeta;
use crate::mal_types::MalAtom::Deref;
use crate::mal_types::MalAtom::SpliceUnQuote;
use crate::mal_types::MalAtom::UnQuote;
use crate::mal_types::MalAtom::QuasiQuote;
use crate::mal_types::MalAtom::Quote;
use crate::mal_types::MalSeq::MHashMap;
use crate::mal_types::MalSeq::Vector;
use crate::MalType;
use crate::mal_types::MalAtom::Nil;
use crate::mal_types::MalSeq::List;
use crate::MalType::MalAtom;
use crate::MalType::MalSeq;
use lazy_static::__Deref;
use lazy_static::lazy_static;
use regex::Regex;
use std::cell::RefCell;

type Index = usize;

#[derive(Clone, Debug)]
struct Token {
    value: String,
}

#[derive(Clone, Debug)]
pub struct Reader {
    index: RefCell<Index>,
    tokens: Vec<Token>,
}

impl Reader {
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(*self.index.borrow().deref())
    }
    fn next(&self) -> Option<&Token> {
        let current = self.tokens.get(*self.index.borrow().deref());
        self.index.replace_with(|&mut old| old + 1);
        current
    }
    fn new(str: String) -> Self {
        Reader {
            index: RefCell::new(0),
            tokens: tokenize(&str),
        }
    }
}

pub fn read_str(input: String) -> Result<MalType, String> {
    read_form(&Reader::new(input))
}

fn read_form(reader: &Reader) -> Result<MalType, String> {
    let current = match reader.peek() {
        Some(token) => token.value.as_str(),
        None => "None",
    };

    match current {
        "None" => Ok(MalAtom(Nil)),
        "(" => {
            reader.next();
            match read_seq(&reader, MalSeqType::MalList) {
                Ok(v) => Ok(v),
                Err(e) => Err(e),
            }
        }
        "[" => {
            reader.next();
            match read_seq(&reader, MalSeqType::MalVector) {
                Ok(v) => Ok(v),
                Err(e) => Err(e),
            }
        }
        "{" => {
            reader.next();
            match read_seq(&reader, MalSeqType::MalHashMap) {
                Ok(v) => Ok(v),
                Err(e) => Err(e),
            }
        }
        _ => read_atom(&reader),
    }
}

enum MalSeqType {
    MalVector,
    MalList,
    MalHashMap,
}
fn read_seq(reader: &Reader, seq_type: MalSeqType) -> Result<MalType, String> {
    let seq_limitor = match seq_type {
        MalSeqType::MalList => ")",
        MalSeqType::MalVector => "]",
        MalSeqType::MalHashMap => "}",
    };
    let mut res: Vec<MalType> = vec![];
    let mut reached_eof = false;
    let mut is_end_of_list_or_eof = || match reader.peek() {
        Some(Token { value }) => {
            if value == seq_limitor {
                reader.next();
                true
            } else {
                false
            }
        }
        None => {
            reached_eof = true;
            true
        }
    };
    while !is_end_of_list_or_eof() {
        res.push(read_form(reader)?);
    }
    if reached_eof {
        Err(String::from("EOF"))
    } else {
        match seq_type {
            MalSeqType::MalList => Ok(MalSeq(List(res))),
            MalSeqType::MalVector => Ok(MalSeq(Vector(res))),
            MalSeqType::MalHashMap => Ok(MalSeq(MHashMap(res))),
        }
    }
}

fn read_atom(reader: &Reader) -> Result<MalType, String> {
    lazy_static! {
        static ref INT_RE: Regex = Regex::new(r"^-?[0-9]+$").unwrap();
        static ref STR_RE: Regex = Regex::new(r#""(?:\\.|[^\\"])*""#).unwrap();
        static ref UN_TERM_STR: Regex = Regex::new(r#"^".*?([^"]|[^\\][\\]")$"#).unwrap();
    };
    let current_token = reader.next();
    let current_token_value = match current_token {
        Some(token) => token.value.as_str(),
        None => "None",
    };

    match current_token_value {
        "None" => Ok(MalAtom(Nil)),
        "'" => Ok(MalAtom(Quote(Box::new(read_form(reader).unwrap())))),
        "`" => Ok(MalAtom(QuasiQuote(Box::new(read_form(reader).unwrap())))),
        "~" => Ok(MalAtom(UnQuote(Box::new(read_form(reader).unwrap())))),
        "~@" => Ok(MalAtom(SpliceUnQuote(Box::new(read_form(reader).unwrap())))),
        "@" => Ok(MalAtom(Deref(Box::new(read_form(reader).unwrap())))),
        "^" => Ok(MalAtom(WithMeta(
            Box::new(read_form(reader).unwrap()),
            Box::new(read_form(reader).unwrap()),
        ))),
        _ => {
            if INT_RE.is_match(&current_token_value) {
                Ok(MalAtom(Number(current_token_value.parse::<i64>().unwrap())))
            } else if STR_RE.is_match(&current_token_value) {
                Ok(MalAtom(MString(current_token_value.to_string())))
            } else if current_token_value.starts_with(":") {
                Ok(MalAtom(KeyWord(current_token_value.to_string())))
            } else if current_token_value.starts_with("\"") {
                Err("EOF".to_string())
            } else {
                Ok(MalAtom(Symbol(current_token_value.to_string())))
            }
        }
    }
}

fn tokenize(str: &str) -> Vec<Token> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r###"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]+)"###
        )
        .unwrap();
    }

    let mut res = vec![];
    for cap in RE.captures_iter(str) {
        if cap[1].starts_with(";") {
            continue;
        }
        res.push(String::from(&cap[1]));
    }
    res.into_iter()
        .map(|v| Token {
            value: v.clone().to_string(),
        })
        .collect()
}
