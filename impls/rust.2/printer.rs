use crate::mal_types::MalAtom::{
    Deref, Func, MString, Nil, Number, QuasiQuote, Quote, SpliceUnQuote, Symbol, KeyWord, UnQuote, WithMeta,
};
use crate::mal_types::MalSeq::{MHashMap, List, Vector};
use crate::mal_types::MalType;
use crate::MalType::MalAtom;
use crate::MalType::MalSeq;
use itertools::join;

pub fn print_str(mal: MalType) -> String {
    match mal {
        MalAtom(Nil) => String::from("Nil"),
        MalAtom(Func(f)) => format!("fn {:?}", f),
        MalAtom(MString(val)) => format!("{}", val.to_string()),
        MalAtom(Number(val)) => format!("{}", val.to_string()),
        MalAtom(Symbol(val)) => format!("{}", val.to_string()),
        MalAtom(KeyWord(val)) => format!("{}", val.to_string()),
        MalSeq(List(list)) => format!(
            "({})",
            join(list.into_iter().map(|val| print_str(val)), " ")
        ),
        MalSeq(Vector(vector)) => format!(
            "[{}]",
            join(vector.into_iter().map(|val| print_str(val)), " ")
        ),
        MalSeq(MHashMap(hashmap)) => format!(
            "{{{}}}",
            join(hashmap.into_iter().map(|val| print_str(val)), " ")
        ),
        MalAtom(Quote(v)) => format!("(quote {})", print_str(*v)),
        MalAtom(QuasiQuote(v)) => format!("(quasiquote {})", print_str(*v)),
        MalAtom(UnQuote(v)) => format!("(unquote {})", print_str(*v)),
        MalAtom(SpliceUnQuote(v)) => format!("(splice-unquote {})", print_str(*v)),
        MalAtom(Deref(v)) => format!("(deref {})", print_str(*v)),
        MalAtom(WithMeta(first, rest)) => {
            format!("(with-meta {} {})", print_str(*rest), print_str(*first))
        }
        
    }
}
