pub enum LispData {
    List(Vec<LispData>),
    Number(isize),
    Symbol(String),
    Str(String)
}
