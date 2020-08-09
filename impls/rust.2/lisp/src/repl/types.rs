use std::collections::HashMap;

#[derive(Clone)]
pub enum LispTerm {
    List(Vec<LispTerm>),
    Vector(Vec<LispTerm>),
    Map(HashMap<String, LispTerm>),
    Number(isize),
    Boolean(bool),
    Symbol(String),
    Str(String),
    Keyword(String),
    Quote(Box<LispTerm>),
    QuasiQuote(Box<LispTerm>),
    Unquote(Box<LispTerm>),
    SpliceUnquote(Box<LispTerm>),
    Func(fn(&[LispTerm]) -> Result<LispTerm, String>),
    Nil
}
