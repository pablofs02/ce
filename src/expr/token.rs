use crate::ErrExpr;

use super::{binario::Binario, literal::Literal, unario::Unario};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Token {
    Literal(Literal),
    Unario(Unario),
    Binario(Binario),
}

impl Token {
    pub fn operar(&self) -> Result<Literal, ErrExpr> {
        match self {
            Token::Literal(l) => Ok(l.clone()),
            Token::Unario(u) => Ok(u.aplicar(u.hijo.as_ref().unwrap().operar()?)),
            Token::Binario(b) => b.aplicar(
                b.izq.as_ref().unwrap().operar()?,
                b.der.as_ref().unwrap().operar()?,
            ),
        }
    }

    pub fn heredar(&mut self, token: Token) -> Result<(), ErrExpr> {
        match self {
            Token::Literal(_) => return Err(ErrExpr::MalToken),
            Token::Unario(ref mut u) => u.heredar(token)?,
            Token::Binario(ref mut b) => b.heredar(token)?,
        }
        Ok(())
    }
}
