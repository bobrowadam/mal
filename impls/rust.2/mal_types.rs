#[derive(Clone, Debug)]
pub enum MalType {
    MalList(Vec::<MalType>),
    MalVector(Vec::<MalType>),
    MalHashMap(Vec::<MalType>), // TODO this probably should be a hash map...
    MalSymbol(String),
    MalString(String),
    MalNumber(i64),
    MalPlus,
    MalMinus,
    MalMul,
    MalDev,
    MalNil,
    MalQuote(Box::<MalType>),
    MalQuasiQuote(Box::<MalType>),
    MalUnQuote(Box::<MalType>),
    MalSpliceUnQuote(Box::<MalType>),
    MalDeref(Box::<MalType>),
    MalWithMeta(Box::<MalType>, Box::<MalType>),
}
