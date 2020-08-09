use std::collections::HashMap;

use super::types::LispTerm;

type LocalEnv = HashMap<String, LispTerm>;

pub struct Env {
    frames: Vec<LocalEnv>    
}

impl Env {
    pub fn new() -> Env {
        Env {
            frames: vec![HashMap::new()]
        }
    }

    pub fn set(&mut self, name: &str, term: &LispTerm) {
        self.frames.last_mut().unwrap().insert(name.to_string(), term.clone());
    }

    pub fn get(&self, name: &str) -> Option<&LispTerm> {
        for local_env in self.frames.iter().rev() {
            if let Some(term) = local_env.get(name) {
                return Some(term);
            }
        }

        None
    }
    
    pub fn push_frame(&mut self) {
        self.frames.push(HashMap::new());
    }

    pub fn pop_frame(&mut self) {
        self.frames.pop();
    }
}

pub fn default_env() -> Env {
        let mut env = Env::new();
        env.set("+", &LispTerm::Func(|args| {
            match args {
                [LispTerm::Number(x), LispTerm::Number(y)] => Ok(LispTerm::Number(x + y)),
                _ => Err("Type error".to_string())
            }
        }));

        env.set("-", &LispTerm::Func(|args| {
            match args {
                [LispTerm::Number(x), LispTerm::Number(y)] => Ok(LispTerm::Number(x - y)),
                _ => Err("Type error".to_string())
            }
        }));

        env.set("*", &LispTerm::Func(|args| {
            match args {
                [LispTerm::Number(x), LispTerm::Number(y)] => Ok(LispTerm::Number(x * y)),
                _ => Err("Type error".to_string())
            }
        }));

        env.set("/", &LispTerm::Func(|args| {
            match args {
                [LispTerm::Number(x), LispTerm::Number(y)] => Ok(LispTerm::Number(x / y)),
                _ => Err("Type error".to_string())
            }
        }));

        env
}
