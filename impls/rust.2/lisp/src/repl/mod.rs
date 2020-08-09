use std::collections::HashMap;

mod reader;
mod printer;
mod types;
pub mod env;

use types::LispTerm;

pub fn rep(input: String, env: &mut env::Env) -> String {
    let res = read(input);
    match res {
        Ok(data) => print(eval(&data, env)),
        Err(err) => print(Err(err))
    }
}

fn read(input: String) -> Result<LispTerm, String> {
    Ok(reader::read_str(&input)?)
}

fn print(input: Result<LispTerm, String>) -> String {
    match input {
        Ok(data) => printer::print_str(&data),
        Err(err) => err
    }
}

fn eval(ast: &LispTerm, env: &mut env::Env) -> Result<LispTerm, String> {
    match ast {
        LispTerm::List(list) if list.is_empty() => Ok(ast.clone()),
        LispTerm::List(list) if is_def(&list) => {
            match (&list[1], &list[2]) {
                (LispTerm::Symbol(name), value) => {
                    let evaluated_value = eval(value, env)?;
                    env.set(name, &evaluated_value);

                    Ok(evaluated_value)
                },
                _ => Err("Failed to evaluate def! term".to_string())
            }
        },
        LispTerm::List(list) if is_let(&list) => eval_let(&list, env),
        LispTerm::List(_) => {
            if let LispTerm::List(list) = eval_ast(ast, env)? {
                match list.split_first() {
                    Some((LispTerm::Func(f), args)) => f(args),
                    _ => Err("Illegal function call".to_string())
                }
            } else {
                Err("Illegal function call".to_string())
            }
        },
        _ => eval_ast(ast, env)
    }
}

fn eval_ast(ast: &LispTerm, env: &mut env::Env) -> Result<LispTerm, String> {
    match ast {
        LispTerm::Symbol(sym) => env.get(sym).cloned().ok_or(format!("Symbol {} not found in env", sym)),
        LispTerm::List(list)  => list.iter().map(|ast| eval(ast, env)).collect::<Result<Vec<_>, String>>().map(|l| LispTerm::List(l)),
        LispTerm::Vector(vec) => vec.iter().map(|ast| eval(ast, env)).collect::<Result<Vec<_>, String>>().map(|l| LispTerm::Vector(l)),
        LispTerm::Map(map)    => map.iter().map(|(key, val)| match eval(val, env) {
                                     Ok(ast) => Ok((key.clone(), ast)),
                                     Err(err) => Err(err)
                                 }).collect::<Result<HashMap<_, _>, String>>().map(|l| LispTerm::Map(l)),
        _                     => Ok(ast.clone())
    }
}

fn eval_let(let_list: &Vec<LispTerm>, env: &mut env::Env) -> Result<LispTerm, String> {
    assert!(is_let(&let_list));


    let term = &let_list[2];
    let bindings = match &let_list[1] {
        LispTerm::List(bindings)   => Ok(bindings),
        LispTerm::Vector(bindings) => Ok(bindings),
        _ => Err("Bindings list missing in let? term".to_string())
    };

    match bindings {
        Ok(bindings) if bindings.len() % 2 != 0 => Err("Odd number of elements in bindings list".to_string()),
        Ok(bindings) => {
            env.push_frame();
            for i in (0..bindings.len() - 1).step_by(2) {
                match (&bindings[i], &bindings[i + 1]) {
                    (LispTerm::Symbol(name), term) => {
                        let eval_term = eval(&term, env)?;
                        env.set(name, &eval_term);
                    },
                    _ => return Err("Failed to evaluate let* term".to_string())
                }
            }

            let res = eval(&term, env);
            env.pop_frame();

            res
        },
        Err(err) => Err(err)
    }
}

fn is_def(list: &Vec<LispTerm>) -> bool {
    list.len() == 3 && match list.first() {
        Some(LispTerm::Symbol(sym)) if sym == "def!" => true,
        _ => false
    }
}
fn is_let(list: &Vec<LispTerm>) -> bool {
    list.len() == 3 && match list.first() {
        Some(LispTerm::Symbol(sym)) if sym == "let*" => true,
        _ => false
    }
}
