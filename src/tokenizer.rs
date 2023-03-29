use nom::{
    bytes::complete::{tag},
    character::complete::{alpha1, alphanumeric1,digit1, multispace0, multispace1},
    IResult,
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
    FunctionName(String),
}



fn tokenize_open_brace(input: &str) -> IResult<&str, Token> {
    let (remaining_input, _) = tag("{")(input)?;
    Ok((remaining_input, Token::OpenBrace))
}

fn tokenize_close_brace(input: &str) -> IResult<&str, Token> {
    let (remaining_input, _) = tag("}")(input)?;
    Ok((remaining_input, Token::CloseBrace))
}


fn tokenize_number(input: &str) -> IResult<&str, Token> {
    let (remaining_input, number) = digit1(input)?;
    Ok((remaining_input, Token::Number(number.to_string())))
}


fn tokenize_identifier(input: &str) -> IResult<&str, Token> {
    let (remaining_input, identifier) = alpha1(input)?;

    if identifier == "def" {
        let (remaining_input, _) = multispace1(remaining_input)?;
        let (remaining_input, function_name) = alphanumeric1(remaining_input)?;
        Ok((remaining_input, Token::FunctionName(function_name.to_string())))
    } else {
        Ok((remaining_input, Token::Identifier(identifier.to_string())))
    }
}



fn tokenize_comma(input: &str) -> IResult<&str, Token> {
    let (remaining_input, _) = tag(",")(input)?;
    Ok((remaining_input, Token::Comma))
}


fn tokenize_operator(input: &str) -> IResult<&str, Token> {
    let (remaining_input, op) = nom::branch::alt((
        tag("+"),
        tag("-"),
        tag("*"),
        tag("/"),
        tag("("),
        tag(")")
    ))(input)?;

    let token = match op {
        "+" => Token::Plus,
        "-" => Token::Minus,
        "*" => Token::Multiply,
        "/" => Token::Divide,
        "(" => Token::OpenParenthesis,
        ")" => Token::CloseParenthesis,
        _ => unreachable!(),
    };

    Ok((remaining_input, token))
}

pub fn tokenize(input: &str) -> IResult<&str, Vec<Token>> {
    let mut tokens = Vec::new();

    let mut current_input = input;

    while let Ok((remaining_input, _)) = multispace0::<_, nom::error::Error<&str>>(current_input) {
        if remaining_input.is_empty() {
            break;
        }

        if let Ok((remaining_input, token)) = nom::branch::alt((
            tokenize_number,
            tokenize_identifier,
            tokenize_operator,
            tokenize_comma,
            tokenize_open_brace,
            tokenize_close_brace,
          
        ))(remaining_input)
        {
            tokens.push(token);
            current_input = remaining_input;
        } else {
            // トークナイザーが失敗した場合、エラー処理を行う
            return Err(nom::Err::Error(nom::error::Error::new(
                remaining_input,
                nom::error::ErrorKind::Tag,
            )));
        }
    }

    Ok((current_input, tokens))
}

#[allow(dead_code)]
fn main() {
    
    let input = "fn test_function(x, y) { x + 42 * y }";
    let (_, tokens) = tokenize(input).unwrap();

    println!("{:?}", tokens);
}