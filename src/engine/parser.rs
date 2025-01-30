//! 正規表現の式をパースし、ASTに変換する
use std::{
    error::Error,
    fmt::{self, Display},
    mem::take,
};

#[derive(Debug)]
pub enum AST {
    Char(char),
    Plus(Box<AST>),
    Star(Box<AST>),
    Question(Box<AST>),
    Or(Box<AST>, Box<AST>),
    Seq(Vec<AST>),
}

#[derive(Debug)]
pub enum ParseError {
    InvalidEscape(usize, char),
    InvalidRightParen(usize),
    NoPrev(usize),
    NoRightParen,
    Empty,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidEscape(pos, ch) => {
                write!(f, "ParseError: invalid escape: pos = {pos}, char = '{ch}'")
            }
            ParseError::InvalidRightParen(pos) => {
                write!(f, "ParseError: invalid right paren: pos = {pos}")
            }
            ParseError::NoPrev(pos) => write!(f, "ParseError: no previous AST: pos = {pos}"),
            ParseError::NoRightParen => write!(f, "ParseError: no right parenthesis"),
            ParseError::Empty => write!(f, "ParseError: empty expression"),
        }
    }
}

impl Error for ParseError {}
