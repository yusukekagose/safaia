use nom::{
    bytes::complete::{tag},
    character::complete::{alpha1, alphanumeric0, digit1, multispace0},
    combinator::recognize,
    IResult,
    sequence::tuple,
    branch::alt,
};

#[derive(Debug, PartialEq)]
pub enum Token {
    Number(String),
    Identifier(String),
    Plus,
    Minus,
    Multiply,
    Divide,
    OpenParenthesis,
    CloseParenthesis,
    OpenBrace,
    CloseBrace,
    Comma,
    DefKeyword,
}

fn tokenize_open_brace(input: &str) -> IResult<&str, Token> {
    let (remaining_input, _) = tag("{")(input)?;
    Ok((remaining_input, Token::OpenBrace))
}

fn tokenize_close_brace(input: &str) -> IResult<&str, Token> {
    let (remaining_input, _) = tag("}")(input)?;
    Ok((remaining_input, Token::CloseBrace))
}

fn tokenize_open_parenthesis(input: &str) -> IResult<&str, Token> {
    let (remaining_input, _) = tag("(")(input)?;
    Ok((remaining_input, Token::OpenParenthesis))
}

fn tokenize_close_parenthesis(input: &str) -> IResult<&str, Token> {
    let (remaining_input, _) = tag(")")(input)?;
    Ok((remaining_input, Token::CloseParenthesis))
}


fn tokenize_number(input: &str) -> IResult<&str, Token> {
    let (remaining_input, number) = digit1(input)?;
    Ok((remaining_input, Token::Number(number.to_string())))
}


fn tokenize_identifier(input: &str) -> IResult<&str, Token> {
    let (remaining_input, identifier) = recognize(tuple((alpha1, alphanumeric0)))(input)?;

    let token = match identifier {
        "def" => Token::DefKeyword,
        _ => Token::Identifier(identifier.to_string()),
    };

    Ok((remaining_input, token))
}


fn tokenize_comma(input: &str) -> IResult<&str, Token> {
    let (remaining_input, _) = tag(",")(input)?;
    Ok((remaining_input, Token::Comma))
}

fn tokenize_def_keyword(input: &str) -> IResult<&str, Token> {
    let (remaining_input, _) = tag("def")(input)?;
    Ok((remaining_input, Token::DefKeyword))
}



fn tokenize_operator(input: &str) -> IResult<&str, Token> {
    let (remaining_input, op) = nom::branch::alt((
        tag("+"),
        tag("-"),
        tag("*"),
        tag("/")
    ))(input)?;

    let token = match op {
        "+" => Token::Plus,
        "-" => Token::Minus,
        "*" => Token::Multiply,
        "/" => Token::Divide,
        _ => unreachable!(),
    };

    Ok((remaining_input, token))
}

fn tokenize_token(input: &str) -> IResult<&str, Token> {
    alt((
        tokenize_number,
        tokenize_def_keyword,
        tokenize_identifier,
        tokenize_operator,
        tokenize_comma,
        tokenize_open_parenthesis,
        tokenize_close_parenthesis,
        tokenize_open_brace,
        tokenize_close_brace,
    ))(input)
}

pub fn tokenize(input: &str) -> IResult<&str, Vec<Token>> {
    let mut remaining_input = input;
    let mut tokens = Vec::new();

    while !remaining_input.trim().is_empty() {
        let (new_remaining_input, _) = multispace0(remaining_input)?;
        remaining_input = new_remaining_input;

        if let Ok((new_remaining_input, token)) = tokenize_token(remaining_input) {
            tokens.push(token);
            remaining_input = new_remaining_input;
        } else {
            return Err(nom::Err::Error(nom::error::Error::new(remaining_input, nom::error::ErrorKind::Tag)));
        }
    }

    Ok((remaining_input, tokens))
}


#[allow(dead_code)]
fn main() {
    
    let input = "fn test_function(x, y) { x + 42 * y }";
    let (_, tokens) = tokenize(input).unwrap();

    println!("{:?}", tokens);
}