use {
    convert::to_i64,
    nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{digit1, multispace0},
        multi::many0,
        sequence::delimited,
        IResult,
    },
    std::collections::VecDeque,
};

mod convert;
mod tests;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    IntLiteral(i64),
    BoolLiteral(bool),

    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

impl From<bool> for Token {
    fn from(v: bool) -> Self {
        Token::BoolLiteral(v)
    }
}

impl From<i64> for Token {
    fn from(v: i64) -> Self {
        Token::IntLiteral(v)
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct Unit {
    tokens: VecDeque<Token>,
}

fn lex_integer(source: &[u8]) -> IResult<&[u8], Token> {
    let negative = lex_operator_sub(source);
    let negative_flag;

    let remain;
    match negative {
        Ok((r, _)) => {
            negative_flag = true;
            remain = r;
        }
        Err(_) => {
            negative_flag = false;
            remain = source;
        }
    }

    let (remain, res) = digit1(remain)?;

    let res = to_i64(res);
    let res = if negative_flag { -res } else { res };

    Ok((remain, Token::IntLiteral(res)))
}

fn lex_bool_true(source: &[u8]) -> IResult<&[u8], Token> {
    let (remain, _) = tag("true")(source)?;
    Ok((remain, Token::BoolLiteral(true)))
}

fn lex_bool_false(source: &[u8]) -> IResult<&[u8], Token> {
    let (remain, _) = tag("false")(source)?;
    Ok((remain, Token::BoolLiteral(false)))
}

fn lex_bool(source: &[u8]) -> IResult<&[u8], Token> {
    alt((lex_bool_true, lex_bool_false))(source)
}

fn lex_operator_add(source: &[u8]) -> IResult<&[u8], Token> {
    let (remain, _) = tag("+")(source)?;
    Ok((remain, Token::Add))
}

fn lex_operator_sub(source: &[u8]) -> IResult<&[u8], Token> {
    let (remain, _) = tag("-")(source)?;
    Ok((remain, Token::Sub))
}

fn lex_operator_mul(source: &[u8]) -> IResult<&[u8], Token> {
    let (remain, _) = tag("*")(source)?;
    Ok((remain, Token::Mul))
}

fn lex_operator_div(source: &[u8]) -> IResult<&[u8], Token> {
    let (remain, _) = tag("/")(source)?;
    Ok((remain, Token::Div))
}

fn lex_operator_mod(source: &[u8]) -> IResult<&[u8], Token> {
    let (remain, _) = tag("%")(source)?;
    Ok((remain, Token::Mod))
}

fn lex_arithmetic_operator(source: &[u8]) -> IResult<&[u8], Token> {
    alt((
        lex_operator_add,
        lex_operator_sub,
        lex_operator_mul,
        lex_operator_div,
        lex_operator_mod,
    ))(source)
}

fn lex_token(source: &[u8]) -> IResult<&[u8], Token> {
    alt((lex_integer, lex_bool, lex_arithmetic_operator))(source)
}

fn skip_spaces(source: &[u8]) -> IResult<&[u8], ()> {
    let (remain, _) = multispace0(source)?;
    Ok((remain, ()))
}

fn lex_token_uni(source: &[u8]) -> IResult<&[u8], Token> {
    delimited(skip_spaces, lex_token, skip_spaces)(source)
}

impl Unit {
    pub fn new() -> Self {
        Self {
            tokens: VecDeque::new(),
        }
    }

    pub fn cur_token(&self) -> Option<&Token> {
        self.tokens.front()
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.tokens.pop_front()
    }

    pub fn add(&mut self, source: &String) -> Result<(), ()> {
        let token_list_res: IResult<&[u8], Vec<Token>> = many0(lex_token_uni)(source.as_bytes());
        if let Ok((_, token_list)) = token_list_res {
            for element in token_list {
                self.tokens.push_back(element);
            }
            Ok(())
        } else {
            Err(())
        }
    }
}
