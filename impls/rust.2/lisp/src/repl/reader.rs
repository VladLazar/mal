use regex::Regex;
use std::collections::HashMap;

use super::types::LispTerm;

type Token = String;

struct ParsedOutput<'a> {
    data: LispTerm,
    tokens_left: &'a [Token]
}

enum SequenceType {
    List,
    Vector,
    Map
}

enum QuoteType {
    Quote,
    QuasiQuote,
    Unquote,
    SpliceUnquote
}

pub fn read_str(input: &str) -> Result<LispTerm, String> {
    let tokens = tokenize(&input);
    Ok(read_form(&tokens)?.data)
}

fn read_form(tokens: &[Token]) -> Result<ParsedOutput, String> {
    if let Some((first, rest)) = tokens.split_first() {
        match first.as_ref() {
            "("  => read_list(rest),
            "["  => read_vector(rest),
            "{"  => read_map(rest),
            "'"  => read_quote(rest, QuoteType::Quote),
            "`"  => read_quote(rest, QuoteType::QuasiQuote),
            "~"  => read_quote(rest, QuoteType::Unquote),
            "~@" => read_quote(rest, QuoteType::SpliceUnquote),
            _    => read_atom(tokens)
        } 
    } else {
        Err(String::from("Failed to parse form"))
    }
}

fn read_quote(tokens: &[Token], quote_type: QuoteType) -> Result<ParsedOutput, String> {
    let next_form = read_form(tokens)?;

    match quote_type {
        QuoteType::Quote => Ok(ParsedOutput {
            data: LispTerm::Quote(Box::new(next_form.data)),
            tokens_left: next_form.tokens_left
        }),
        QuoteType::Unquote => Ok(ParsedOutput {
            data: LispTerm::Unquote(Box::new(next_form.data)),
            tokens_left: next_form.tokens_left
        }),
        QuoteType::QuasiQuote => Ok(ParsedOutput {
            data: LispTerm::QuasiQuote(Box::new(next_form.data)),
            tokens_left: next_form.tokens_left
        }),
        QuoteType::SpliceUnquote => Ok(ParsedOutput {
            data: LispTerm::SpliceUnquote(Box::new(next_form.data)),
            tokens_left: next_form.tokens_left
        })
    }
}

fn read_list(tokens: &[Token]) -> Result<ParsedOutput, String> {
    read_sequence(tokens, SequenceType::List)    
}

fn read_vector(tokens: &[Token]) -> Result<ParsedOutput, String> {
    read_sequence(tokens, SequenceType::Vector)    
} 

fn read_map(tokens: &[Token]) -> Result<ParsedOutput, String> {
    read_sequence(tokens, SequenceType::Map)    
} 

fn read_sequence(tokens: &[Token], sequence_type: SequenceType) -> Result<ParsedOutput, String> {
    let mut tokens_left = tokens;
    let mut parsed_forms = Vec::new();

    let delimiter = match sequence_type {
        SequenceType::List => ')',
        SequenceType::Vector => ']',
        SequenceType::Map => '}'
    };

    while tokens_left.len() > 1 && first_char(tokens_left.first().unwrap()) != delimiter {
        let parsed_output  = read_form(tokens_left)?;
        parsed_forms.push(parsed_output.data);
        tokens_left = parsed_output.tokens_left;
    }

    if tokens_left.len() > 1 {
        let data = match sequence_type {
            SequenceType::List => LispTerm::List(parsed_forms),
            SequenceType::Vector => LispTerm::Vector(parsed_forms),
            SequenceType::Map => LispTerm::Map(construct_map(&parsed_forms)?)
        };

        Ok(ParsedOutput {
            data,
            tokens_left: &tokens_left[1..]
        })
    } else {
        Err(String::from("Failed to parse unbalanced list"))
    }
}

fn construct_map(forms: &Vec<LispTerm>) -> Result<HashMap<String, LispTerm>, String> {
    if forms.len() % 2 != 0 {
        return Err(String::from("Odd number of elements in map"));
    }

    let keys    = forms.iter().enumerate().filter(|(idx, _)| idx % 2 == 0).map(|tuple| tuple.1).cloned();
    let values  = forms.iter().enumerate().filter(|(idx, _)| idx % 2 == 1).map(|tuple| tuple.1).cloned();

    let mut map = HashMap::new();
    for (key, value) in keys.zip(values) {
        match key {
            LispTerm::Str(string_key)  => { map.insert(string_key, value); },
            LispTerm::Keyword(keyword) => { map.insert(format!(":{}", keyword), value); },
            _ => { return Err(String::from("Non string used as key in map")); }
        }
    }

    Ok(map)
}

fn read_atom(tokens: &[Token]) -> Result<ParsedOutput, String> {
    if tokens.is_empty() {
        return Err(String::from("Failed to parse atom"));
    } else if tokens.first().unwrap().is_empty() {
        return Err(String::from("Failed to parse empty token"));
    }

    let token = tokens.first().unwrap();
    match token.as_ref() {
        token if token.parse::<isize>().is_ok() => Ok(ParsedOutput {
            data: LispTerm::Number(token.parse::<isize>().unwrap()),
            tokens_left: &tokens[1..]
        }),
        token if first_char(token) == '"' => Ok(ParsedOutput {
            data: LispTerm::Str(read_string(token)?),
            tokens_left: &tokens[1..]
        }),
        token if first_char(token) == ':' => Ok(ParsedOutput {
            data: LispTerm::Keyword(read_keyword(token)?),
            tokens_left: &tokens[1..]
        }),
        "nil" => Ok(ParsedOutput {
            data: LispTerm::Nil,
            tokens_left: &tokens[1..]
        }),
        "true" => Ok(ParsedOutput {
            data: LispTerm::Boolean(true),
            tokens_left: &tokens[1..]
        }),
        "false" => Ok(ParsedOutput {
            data: LispTerm::Boolean(false),
            tokens_left: &tokens[1..]
        }),
        token => Ok(ParsedOutput {
            data: LispTerm::Symbol(String::from(token)),
            tokens_left: &tokens[1..]
        })
    }
}

fn read_string(token: &str) -> Result<String, String> {
    // Check if the string is terminated by a non escaped double quote.
    // Do this by counting the number of contigous backslashes and ensuring it is even.
    if token.len() >= 2 && token.ends_with("\"") && token.chars().rev().skip(1).take_while(|c| *c == '\\').count() % 2 == 0 {
        Ok(token.trim_matches('"').to_string())
    } else {
        Err("Failed to parse unbalanced string".to_string())
    }
}

fn read_keyword(token: &str) -> Result<String, String> {
    match token.strip_prefix(':') {
        Some(keyword) => Ok(keyword.to_string()),
        None          => Err("Token did not start with :".to_string())
    }
}

fn tokenize(input: &str) -> Vec<Token> {
    let raw = r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"#;
    let re = Regex::new(raw).unwrap();

    re.captures_iter(&input).map(|x| String::from(x[1].trim())).collect()
}

fn first_char(token: &str) -> char {
    token.chars().next().unwrap()
}
