use std::collections::HashMap;

#[derive(Clone)]
pub enum LispData {
    List(Vec<LispData>),
    Vector(Vec<LispData>),
    Map(HashMap<String, LispData>),
    Number(isize),
    Boolean(bool),
    Symbol(String),
    Str(String),
    Keyword(String),
    Quote(Box<LispData>),
    QuasiQuote(Box<LispData>),
    Unquote(Box<LispData>),
    SpliceUnquote(Box<LispData>),
    Nil
}
