use super::types::LispData;

pub fn print_str(data: &LispData) -> String {
    match data {
        LispData::Symbol(name) => name.to_string(),
        LispData::Number(num) => num.to_string(),
        LispData::List(list) => format!("({})", list.iter().map(|x| print_str(x)).collect::<Vec<_>>().join(" ")),
        LispData::Str(string) => format!("\"{}\"", string)
    }
}
