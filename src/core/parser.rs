use super::token::Token;
use super::types::{ Int, Float };
use super::symbols::{ FALSE, TRUE };

#[derive(Debug)]
pub struct Parser;

impl Parser {
    fn tokenize<'a>(&self, text: &'a str) -> std::str::SplitWhitespace<'a> {
        // TODO : Clever tokenization to handle operators split, etc.
        text.split_whitespace()
    }

    fn parse_token<'a>(&self, token: &'a str) -> Token<'a> {
        // Try to parse as an int, or a float, or a token
        // If not possible, ignore
        // Token is assumed to be clean
        match token.parse::<Int>() {
            Ok(i) => Token::new_integer(i),
            Err(_) => match token.parse::<Float>() {
                Ok(f) => Token::new_float(f),
                Err(_) => {
                    // Cache the value to avoid checking twice
                    let is_true = token == TRUE;

                    if is_true || (token == FALSE) {
                        Token::new_bool(is_true)
                    } else {
                        Token::new_operator(token).unwrap_or(Token::new_ignored())
                    }
                }
            },
        }
    }

    pub fn parse<'a>(&self, text: &'a str) -> Vec<Token<'a>> {
        // TODO : Force uppercase
        self
            .tokenize(text)
            .map(|t| self.parse_token(t))
            .filter(|t| t.is_legit())
            .collect::<Vec<Token>>()
    }
}

#[cfg(test)]
mod tests {
    use crate::core::{ Parser, Token };
    use crate::core::symbols::{ ADD, SUB, MUL, DIV };

    #[test]
    fn test_tokenize_one_token() {
        let p = Parser;
        let text = "test";
        let tokens = p.tokenize(text).collect::<Vec<&str>>();
        let expected = vec!["test"];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_tokenize_many_tokens() {
        let p = Parser;
        let text = "test of   tokenization";
        let tokens = p.tokenize(text).collect::<Vec<&str>>();
        let expected = vec!["test", "of", "tokenization"];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_tokenize_trailing_spaces() {
        let p = Parser;
        let text = "  test of tokenization   ";
        let tokens = p.tokenize(text).collect::<Vec<&str>>();
        let expected = vec!["test", "of", "tokenization"];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_tokenize_no_token() {
        let p = Parser;
        let text = "";
        let tokens = p.tokenize(text).collect::<Vec<&str>>();

        assert_eq!(tokens.is_empty(), true);
    }

    #[test]
    fn test_parse_token_positive_int() {
        let p = Parser;
        let token = "42";
        let parsed = p.parse_token(token);
        let expected = Token::new_integer(42);

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_token_negative_int() {
        let p = Parser;
        let token = "-42";
        let parsed = p.parse_token(token);
        let expected = Token::new_integer(-42);

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_token_positive_float() {
        let p = Parser;
        let token = "13.37";
        let parsed = p.parse_token(token);
        let expected = Token::new_float(13.37);

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_token_negative_float() {
        let p = Parser;
        let token = "-13.37";
        let parsed = p.parse_token(token);
        let expected = Token::new_float(-13.37);

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_token_operator() {
        let p = Parser;
        let mut token = "+";
        let mut parsed = p.parse_token(token);
        let mut expected = Token::new_operator(ADD).unwrap();

        assert_eq!(parsed, expected);

        token = "-";
        parsed = p.parse_token(token);
        expected = Token::new_operator(SUB).unwrap();

        assert_eq!(parsed, expected);

        token = "*";
        parsed = p.parse_token(token);
        expected = Token::new_operator(MUL).unwrap();

        assert_eq!(parsed, expected);

        token = "/";
        parsed = p.parse_token(token);
        expected = Token::new_operator(DIV).unwrap();

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_token_ignored() {
        let p = Parser;
        let token = "text";
        let parsed = p.parse_token(token);
        let expected = Token::new_ignored();

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse() {
        let p = Parser;
        let text = "1 + 2.0 - 5.5    text ";
        let parsed = p.parse(text);
        let expected = vec![
            Token::new_integer(1),
            Token::new_operator(ADD).unwrap(),
            Token::new_integer(2),
            Token::new_operator(SUB).unwrap(),
            Token::new_float(5.5)
        ];

        assert_eq!(parsed, expected);
    }
}
