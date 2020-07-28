use super::types::LispData;

pub fn print_str(data: &LispData) -> String {
    match data {
        LispData::Symbol(name) => name.to_string(),
        LispData::Number(num) => num.to_string(),
        LispData::List(list) => format!("({})", list.iter().map(|x| print_str(x)).collect::<Vec<_>>().join(" ")),
        LispData::Vector(vector) => format!("[{}]", vector.iter().map(|x| print_str(x)).collect::<Vec<_>>().join(" ")),
        LispData::Map(map) => format!("{{{}}}", map.iter().map(|(k, v)| format!("{} {}", print_map_key(k), print_str(v))).collect::<Vec<_>>().join(" ")),
        LispData::Str(string) => format!("\"{}\"", string),
        LispData::Keyword(keyword) => format!(":{}", keyword),
        LispData::Boolean(val) => format!("{}", val),
        LispData::Quote(form) => format!("(quote {})", print_str(form)),
        LispData::QuasiQuote(form) => format!("(quasiquote {})", print_str(form)),
        LispData::Unquote(form) => format!("(unquote {})", print_str(form)),
        LispData::SpliceUnquote(form) => format!("(splice-unquote {})", print_str(form)),
        LispData::Nil => String::from("nil")
    }
}

fn print_map_key(key: &str) -> String {
    if key.starts_with(":") {
        key.to_string()
    } else {
        format!("\"{}\"", key)
    }
}
