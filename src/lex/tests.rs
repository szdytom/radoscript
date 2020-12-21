#[cfg(test)]
#[test]
fn test_lex_integer() {
    use super::{lex_integer, Token};
    assert_eq!(
        lex_integer("01234".as_bytes()),
        Ok(("".as_bytes(), Token::from(1234)))
    );
    assert_eq!(
        lex_integer("-1234".as_bytes()),
        Ok(("".as_bytes(), Token::from(-1234)))
    );
}

#[test]
fn test_lex_bool() {
    use super::{lex_bool, Token};
    assert_eq!(
        lex_bool("true".as_bytes()),
        Ok(("".as_bytes(), Token::from(true)))
    );
    assert_eq!(
        lex_bool("false".as_bytes()),
        Ok(("".as_bytes(), Token::from(false)))
    );
}

#[test]
fn test_lex_arithmetic_operator() {
    use super::{lex_arithmetic_operator, Token};
    assert_eq!(
        lex_arithmetic_operator("+".as_bytes()),
        Ok(("".as_bytes(), Token::Add))
    );
    assert_eq!(
        lex_arithmetic_operator("-".as_bytes()),
        Ok(("".as_bytes(), Token::Sub))
    );
    assert_eq!(
        lex_arithmetic_operator("*".as_bytes()),
        Ok(("".as_bytes(), Token::Mul))
    );
    assert_eq!(
        lex_arithmetic_operator("/".as_bytes()),
        Ok(("".as_bytes(), Token::Div))
    );
    assert_eq!(
        lex_arithmetic_operator("%".as_bytes()),
        Ok(("".as_bytes(), Token::Mod))
    );
}

#[test]
fn test_lex_token_uni() {
    use super::{lex_token_uni, Token};
    assert_eq!(
        lex_token_uni(" %\n".as_bytes()),
        Ok(("".as_bytes(), Token::Mod))
    );
    assert_eq!(
        lex_token_uni("*  \r ".as_bytes()),
        Ok(("".as_bytes(), Token::Mul))
    );
    assert_eq!(
        lex_token_uni("17   \t".as_bytes()),
        Ok(("".as_bytes(), Token::from(17)))
    );
    assert_eq!(
        lex_token_uni(" \tfalse  ".as_bytes()),
        Ok(("".as_bytes(), Token::from(false)))
    );
}

#[test]
fn test_lexer1() {
    use super::{Token, Unit};
    let mut lexer = Unit::new();
    lexer.add(&String::from("7 + 2 *5-  12%\t\t\t6")).unwrap();
    assert_eq!(lexer.next_token(), Some(Token::from(7)));
    assert_eq!(lexer.next_token(), Some(Token::Add));
    assert_eq!(lexer.next_token(), Some(Token::from(2)));
    assert_eq!(lexer.next_token(), Some(Token::Mul));
    assert_eq!(lexer.next_token(), Some(Token::from(5)));
    assert_eq!(lexer.next_token(), Some(Token::Sub));
    assert_eq!(lexer.next_token(), Some(Token::from(12)));
    assert_eq!(lexer.next_token(), Some(Token::Mod));
    assert_eq!(lexer.next_token(), Some(Token::from(6)));
    assert_eq!(lexer.next_token(), None);
}

#[test]
fn test_lexer2() {
    use super::{Token, Unit};
    let mut lexer = Unit::new();
    lexer.add(&String::from("2+3+-4--5")).unwrap();
    assert_eq!(lexer.next_token(), Some(Token::from(2)));
    assert_eq!(lexer.next_token(), Some(Token::Add));
    assert_eq!(lexer.next_token(), Some(Token::from(3)));
    assert_eq!(lexer.next_token(), Some(Token::Add));
    assert_eq!(lexer.next_token(), Some(Token::from(-4)));
    assert_eq!(lexer.next_token(), Some(Token::Sub));
    assert_eq!(lexer.next_token(), Some(Token::from(-5)));
    assert_eq!(lexer.next_token(), None);
}
