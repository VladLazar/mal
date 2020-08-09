use super::types::LispTerm;

pub fn print_str(data: &LispTerm) -> String {
    match data {
        LispTerm::Symbol(name) => name.to_string(),
        LispTerm::Number(num) => num.to_string(),
        LispTerm::List(list) => format!("({})", list.iter().map(|x| print_str(x)).collect::<Vec<_>>().join(" ")),
        LispTerm::Vector(vector) => format!("[{}]", vector.iter().map(|x| print_str(x)).collect::<Vec<_>>().join(" ")),
        LispTerm::Map(map) => format!("{{{}}}", map.iter().map(|(k, v)| format!("{} {}", print_map_key(k), print_str(v))).collect::<Vec<_>>().join(" ")),
        LispTerm::Str(string) => format!("\"{}\"", string),
        LispTerm::Keyword(keyword) => format!(":{}", keyword),
        LispTerm::Boolean(val) => format!("{}", val),
        LispTerm::Quote(form) => format!("(quote {})", print_str(form)),
        LispTerm::QuasiQuote(form) => format!("(quasiquote {})", print_str(form)),
        LispTerm::Unquote(form) => format!("(unquote {})", print_str(form)),
        LispTerm::SpliceUnquote(form) => format!("(splice-unquote {})", print_str(form)),
        LispTerm::Func(_) => "#<function>".to_string(),
        LispTerm::Nil => String::from("nil")
    }
}

fn print_map_key(key: &str) -> String {
    if key.starts_with(":") {
        key.to_string()
    } else {
        format!("\"{}\"", key)
    }
}
