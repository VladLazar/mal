mod reader;
mod printer;
mod types;

pub fn rep(input: String) -> String {
    let res = read(input);
    match res {
        Ok(data) => print(Ok(eval(data))),
        Err(err) => print(Err(err))
    }
}

fn read(input: String) -> Result<types::LispData, String> {
    Ok(reader::read_str(&input)?)
}

fn eval(input: types::LispData) -> types::LispData {
    input
}

fn print(input: Result<types::LispData, String>) -> String {
    match input {
        Ok(data) => printer::print_str(&data),
        Err(err) => err
    }
}
