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

// TODO thiserrorに置き換えてみる
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

/// 特殊文字のエスケープ
fn parse_escape(pos: usize, c: char) -> Result<AST, ParseError> {
    match c {
        '\\' | '(' | ')' | '|' | '+' | '*' | '?' => Ok(AST::Char(c)),
        _ => {
            let err = ParseError::InvalidEscape(pos, c);
            Err(err)
        }
    }
}

enum PSQ {
    Plus,
    Star,
    Question,
}

fn parse_plus_star_question(
    seq: &mut Vec<AST>,
    ast_type: PSQ,
    pos: usize,
) -> Result<(), ParseError> {
    if let Some(prev) = seq.pop() {
        let ast = match ast_type {
            PSQ::Plus => AST::Plus(Box::new(prev)),
            PSQ::Star => AST::Star(Box::new(prev)),
            PSQ::Question => AST::Question(Box::new(prev)),
        };
        seq.push(ast);
        Ok(())
    } else {
        Err(ParseError::NoPrev(pos))
    }
}

fn fold_or(mut seq_or: Vec<AST>) -> Option<AST> {
    if seq_or.len() > 1 {
        let mut ast = seq_or.pop().unwrap();
        seq_or.reverse();
        for s in seq_or {
            ast = AST::Or(Box::new(s), Box::new(ast))
        }
        Some(ast)
    } else {
        seq_or.pop()
    }
}
