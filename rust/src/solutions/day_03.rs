use std::{iter::Peekable, str::Chars};

use crate::solvable::Solvable;

pub struct Day03;

impl Solvable for Day03 {
    fn first(&self, input: &str) -> i32 {
        let mut lexer = Lexer::new(input);
        let mut iter = lexer.lex().into_iter().peekable();

        let mut sum = 0;

        while let Some(token) = iter.next() {
            if let Token::Mul = token {
                if iter.peek() == Some(&Token::LeftParen) {
                    iter.next();

                    if let Some(Token::Int(x)) = iter.next() {
                        if iter.peek() == Some(&Token::Comma) {
                            iter.next();

                            if let Some(Token::Int(y)) = iter.next() {
                                if iter.peek() == Some(&Token::RightParen) {
                                    iter.next();
                                    sum += x * y;
                                }
                            }
                        }
                    }
                }
            }
        }
        sum
    }

    fn second(&self, input: &str) -> i32 {
        let mut lexer = Lexer::new(input);
        let mut iter = lexer.lex().into_iter().peekable();

        let mut disabled = false;
        let mut sum = 0;

        while let Some(token) = iter.next() {
            if let Token::Do = token {
                disabled = false;
            }
            if let Token::Dont = token {
                disabled = true;
            }
            if let Token::Mul = token {
                if iter.peek() == Some(&Token::LeftParen) {
                    iter.next();

                    if let Some(Token::Int(x)) = iter.next() {
                        if iter.peek() == Some(&Token::Comma) {
                            iter.next();

                            if let Some(Token::Int(y)) = iter.next() {
                                if iter.peek() == Some(&Token::RightParen) {
                                    iter.next();
                                    if !disabled {
                                        sum += x * y;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        sum
    }
}

struct Lexer<'a> {
    source: Peekable<Chars<'a>>,
    eof: bool,
}

impl<'a> Lexer<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            source: source.chars().peekable(),
            eof: false,
        }
    }

    fn lex(&mut self) -> Vec<Token> {
        self.into_iter().collect()
    }

    fn handle_number(&mut self, c: char) -> Token {
        let mut value = String::from(c);

        while let Some(ch) = self.source.peek() {
            if ch.is_numeric() {
                value.push(*ch);
                self.source.next();
            } else {
                break;
            }
        }

        Token::Int(value.parse::<i32>().unwrap())
    }

    fn handle_word(&mut self, c: char) -> Token {
        let mut value = String::from(c);

        while let Some(ch) = self.source.peek() {
            if !ch.is_alphabetic() && *ch != '\'' {
                break;
            } else {
                value.push(*ch);
                self.source.next();
            }
        }

        if value == "mul" {
            Token::Mul
        } else if value == "do" {
            Token::Do
        } else if value == "don't" {
            Token::Dont
        } else {
            Token::Invalid
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.eof {
            None
        } else {
            let token = match self.source.next() {
                Some('(') => Token::LeftParen,
                Some(')') => Token::RightParen,
                Some(',') => Token::Comma,
                Some(c) => {
                    if c.is_numeric() {
                        self.handle_number(c)
                    } else if c.is_alphabetic() {
                        self.handle_word(c)
                    } else {
                        Token::Invalid
                    }
                }
                None => {
                    self.eof = true;
                    Token::Eof
                }
            };
            Some(token)
        }
    }
}

#[derive(PartialEq)]
enum Token {
    LeftParen,
    RightParen,
    Comma,
    Int(i32),
    Mul,
    Do,
    Dont,
    Invalid,
    Eof,
}
