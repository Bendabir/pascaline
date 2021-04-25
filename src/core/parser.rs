use super::token::Token;
use super::types::{ Int, Float };
use super::symbols::OPERATORS;

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
                Err(_) => if OPERATORS.contains(&token) {
                    Token::new_operator(token)
                } else {
                    Token::new_ignored()
                },
            },
        }
    }

    pub fn parse<'a>(&self, text: &'a str) -> Vec<Token<'a>> {
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
        let expected = Token::Integer(42);

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_token_negative_int() {
        let p = Parser;
        let token = "-42";
        let parsed = p.parse_token(token);
        let expected = Token::Integer(-42);

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_token_positive_float() {
        let p = Parser;
        let token = "13.37";
        let parsed = p.parse_token(token);
        let expected = Token::Float(13.37);

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_token_negative_float() {
        let p = Parser;
        let token = "-13.37";
        let parsed = p.parse_token(token);
        let expected = Token::Float(-13.37);

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_token_operator() {
        let p = Parser;
        let mut token = "+";
        let mut parsed = p.parse_token(token);
        let mut expected = Token::Operator(ADD);

        assert_eq!(parsed, expected);

        token = "-";
        parsed = p.parse_token(token);
        expected = Token::Operator(SUB);

        assert_eq!(parsed, expected);

        token = "*";
        parsed = p.parse_token(token);
        expected = Token::Operator(MUL);

        assert_eq!(parsed, expected);

        token = "/";
        parsed = p.parse_token(token);
        expected = Token::Operator(DIV);

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_token_ignored() {
        let p = Parser;
        let token = "text";
        let parsed = p.parse_token(token);
        let expected = Token::Ignored;

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse() {
        let p = Parser;
        let text = "1 + 2.0 - 5.5    text ";
        let parsed = p.parse(text);
        let expected = vec![
            Token::Integer(1),
            Token::Operator(ADD),
            Token::Integer(2),
            Token::Operator(SUB),
            Token::Float(5.5)
        ];

        assert_eq!(parsed, expected);
    }
}
