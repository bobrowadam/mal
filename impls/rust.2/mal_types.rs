use std::fmt::Display;

#[derive(Clone, Debug)]
pub enum MalType {
    MalSeq(MalSeq),
    MalAtom(MalAtom),
}

#[derive(Clone, Debug)]
pub enum MalSeq {
    List(Vec::<MalType>),
    Vector(Vec::<MalType>),
    MHashMap(Vec::<MalType>), // TODO this probably should be a hash map...
}

pub type MalFunc = fn (MalType) -> MalResult;
#[derive(Clone, Debug)]
pub enum MalAtom {
    Func(MalFunc),
    Symbol(String),
    MString(String),
    Number(i64),
    Nil,
    Quote(Box::<MalType>),
    QuasiQuote(Box::<MalType>),
    UnQuote(Box::<MalType>),
    SpliceUnQuote(Box::<MalType>),
    Deref(Box::<MalType>),
    WithMeta(Box::<MalType>, Box::<MalType>),
    KeyWord(String),
}

pub type MalResult = Result<MalType, MalError>;

#[derive(Debug)]
pub enum MalError {
    TypeError(String),
    UnknownSymbol(String),
    EvalError(String),
}

impl Display for MalError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
           MalError::TypeError(s) => write!(formatter, "ParseError: {}", s),
           MalError::UnknownSymbol(s) => write!(formatter, "UnknownSymbol: {}", s),
           MalError::EvalError(s) => write!(formatter, "EvalError: {}", s),
        }
    }
}
