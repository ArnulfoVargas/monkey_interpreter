use crate::token::{Token, TokenKind};

struct Lexer{
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch : char,
}

impl Lexer{
    pub fn new(input: &str) -> Lexer{
        let mut lex = Lexer{
            position: 0,
            read_position: 0,
            input: input.chars().collect(),
            ch: Default::default(),
        };

        lex.read_char();

        lex
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len(){
            self.ch = '\0'   
        }
        else {
            self.ch = self.input[self.read_position]
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn next_token(&mut self) -> Token {
        let token = match self.ch {
            '=' => Lexer::new_token(TokenKind::Assign, self.ch),
            '+' => Lexer::new_token(TokenKind::Plus, self.ch),
            ';' => Lexer::new_token(TokenKind::Semicolon, self.ch),
            '(' => Lexer::new_token(TokenKind::Lparen, self.ch),
            ')' => Lexer::new_token(TokenKind::Rparen, self.ch),
            '{' => Lexer::new_token(TokenKind::Lbraces, self.ch),
            '}' => Lexer::new_token(TokenKind::Rbraces, self.ch),
            ',' => Lexer::new_token(TokenKind::Comma, self.ch),
            '\0' => Token { kind: TokenKind::Eof, literal: "".to_string() },
            _ => Lexer::new_token(TokenKind::Illegal, self.ch),
        };

        self.read_char();

        token
    }

    fn new_token(kind: TokenKind, ch : char) -> Token{
        Token { kind: kind, literal: ch.to_string() }
    }
}

#[cfg(test)]
mod test {
    use crate::token::{Token, TokenKind};

    use super::Lexer;

    #[test]
    fn test_next_token(){
        let input = "=+(){},;";

        let expected: Vec<Token> = vec![
            Token{
                kind: TokenKind::Assign,
                literal: "=".to_string()
            },
            Token{
                kind: TokenKind::Plus,
                literal: "+".to_string()
            },
            Token{
                kind: TokenKind::Lparen,
                literal: "(".to_string()
            },
            Token{
                kind: TokenKind::Rparen,
                literal: ")".to_string()
            },
            Token{
                kind: TokenKind::Lbraces,
                literal: "{".to_string()
            },
            Token{
                kind: TokenKind::Rbraces,
                literal: "}".to_string()
            },
            Token{
                kind: TokenKind::Comma,
                literal: ",".to_string()
            },
            Token{
                kind: TokenKind::Semicolon,
                literal: ";".to_string()
            },
            Token{
                kind: TokenKind::Eof,
                literal: "".to_string()
            },
        ];

        let mut lexer = Lexer::new(input);

        for (idx, exp_token) in expected.into_iter().enumerate(){
            let recv_token = lexer.next_token();

            assert_eq!(exp_token.kind, recv_token.kind, 
                "Test of {idx} - token type wrong.\nExpected {}, but got {}", 
                exp_token.kind, recv_token.kind);

            assert_eq!(exp_token.literal, recv_token.literal,
                "Test of {idx} - Literal wrong.\nExpected {}, but got {}",
                exp_token.literal, recv_token.literal);
        }
    }
}