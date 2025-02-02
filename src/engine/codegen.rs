use super::{parser::AST, Instruction};
use create::helper::safe_add;
use std::{
    error::Error,
    fmt::{self, Display},
};

#[derive(Debug)]
pub enum CodeGenError {
    PCOverFlow,
    FailStar,
    FailOr,
    FailQuestion,
}

impl Display for CodeGenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Code Generation Error: {:?}", self)?
    }
}

impl Error for CodeGenError {}

#[derive(Debug)]
struct Generator {
    pc: usize,
    insts: Vec<Instruction>,
}

fn inc_pc(&mut self) -> Result<(), CodeGenError> {
    safe_add(&mut self.pc, &1, || CodeGenError::POverFlow)
}

fn gen_expr(&mut self, ast: &AST) -> Result<(), CodeGenError> {
    match ast {
        AST::Char(c) => self.gen_char(*c)?,
        AST::Or(e1, e2) => self.gen_or(e1, e2)?,
        AST::Plus(e) => self.gen_plus(e)?,
        AST::Star(e) => self.gen_star(e)?,
        AST::Question(e) => self.gen_question(e)?,
        AST::Seq(v) => self.gen_seq(v)?,
    }
    Ok(())
}

fn gen_seq(&mut self, exprs: &[AST]) -> Result<(), CodeGenError> {
    for e in exprs {
        self.gen_expr(e)?;
    }
    Ok(())
}

fn gen_char(&mut self, c: char) -> Result<(), CodeGenError> {
    let inst = Instruction::Char(c);
    self.insts.push(inst);
    self.inc_pc()?;
    Ok(())
}
