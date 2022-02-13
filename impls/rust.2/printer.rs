use itertools::join;
use crate::mal_types::MalType;

pub fn print_str(mal: MalType) -> String {
    match mal {
        MalType::MalNil => String::from("Nil"),
        MalType::MalString(val) => format!("{}", val.to_string()),
        MalType::MalNumber(val) => format!("{}", val.to_string()),
        MalType::MalSymbol(val) => format!("{}", val.to_string()),
        MalType::MalList(list) => format!("({})", join(list.into_iter().map(|val| print_str(val)), " ")),
        MalType::MalVector(vector) => format!("[{}]", join(vector.into_iter().map(|val| print_str(val)), " ")),
        MalType::MalHashMap(hashmap) => format!("{{{}}}", join(hashmap.into_iter().map(|val| print_str(val)), " ")),
        MalType::MalPlus => String::from("+"),
        MalType::MalMinus => String::from("-"),
        MalType::MalMul => String::from("*"),
        MalType::MalDev => String::from("/"),
        MalType::MalQuote(v) => format!("(quote {})", print_str(*v)),
        MalType::MalQuasiQuote(v) => format!("(quasiquote {})", print_str(*v)),
        MalType::MalUnQuote(v) => format!("(unquote {})", print_str(*v)),
        MalType::MalSpliceUnQuote(v) => format!("(splice-unquote {})", print_str(*v)),
        MalType::MalDeref(v) => format!("(deref {})", print_str(*v)),
        MalType::MalWithMeta(first, rest) => format!("(with-meta {} {})", print_str(*rest), print_str(*first))
    }
}
