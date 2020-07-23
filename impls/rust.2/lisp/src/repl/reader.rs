use regex::Regex;

use super::types::LispData;

type Token = String;

struct ParsedOutput<'a> {
    data: LispData,
    tokens_left: &'a [Token]
}

pub fn read_str(input: &str) -> Result<LispData, String> {
    let tokens = tokenize(&input);
    println!("{:?}", tokens);
    Ok(read_form(&tokens)?.data)
}

fn read_form(tokens: &[Token]) -> Result<ParsedOutput, String> {
    if let Some((first, rest)) = tokens.split_first() {
        match first_char(first) {
            '(' => read_list(rest),
            _   => read_atom(tokens)
        } 
    } else {
        Err(String::from("Failed to parse form"))
    }
}

fn read_list(tokens: &[Token]) -> Result<ParsedOutput, String> {
    let mut tokens_left = tokens;
    let mut parsed_forms = Vec::new();

    while tokens_left.len() > 1 && first_char(tokens_left.first().unwrap()) != ')' {
        let parsed_output  = read_form(tokens_left)?;
        parsed_forms.push(parsed_output.data);
        tokens_left = parsed_output.tokens_left;
    }

    if tokens_left.len() > 1 {
        Ok(ParsedOutput {
            data: LispData::List(parsed_forms),
            tokens_left: &tokens_left[1..]
        })
    } else {
        Err(String::from("Failed to parse unbalanced list"))
    }
}

fn read_atom(tokens: &[Token]) -> Result<ParsedOutput, String> {
    if tokens.is_empty() {
        return Err(String::from("Failed to parse atom"));
    } else if tokens.first().unwrap().is_empty() {
        return Err(String::from("Failed to parse empty token"));
    }

    let token = tokens.first().unwrap();
    match token {
        token if token.parse::<isize>().is_ok() => Ok(ParsedOutput {
            data: LispData::Number(token.parse::<isize>().unwrap()),
            tokens_left: &tokens[1..]
        }),
        token if first_char(token) == '"' => Ok(ParsedOutput {
            data: LispData::Str(read_string(token)?),
            tokens_left: &tokens[1..]
        }),
        token => Ok(ParsedOutput {
            data: LispData::Symbol(String::from(token)),
            tokens_left: &tokens[1..]
        })
    }
}

fn read_string(token: &Token) -> Result<String, String> {
    // Check if the string is terminated by a non escaped double quote.
    // Do this by counting the number of contigous backslashes and ensuring it is even.
    if token.ends_with("\"") && token.chars().rev().skip(1).take_while(|c| *c == '\\').count() % 2 == 0 {
        Ok(token.trim_matches('"').to_string())
    } else {
        Err("Failed to parse unbalanced string".to_string())
    }
}

fn tokenize(input: &str) -> Vec<Token> {
    let raw = r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"#;
    let re = Regex::new(raw).unwrap();

    re.captures_iter(&input).map(|x| String::from(x[1].trim())).collect()
}

fn first_char(token: &Token) -> char {
    token.chars().next().unwrap()
}
