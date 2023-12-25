mod binario;
mod literal;
mod token;
mod unario;

pub use self::{
    binario::{Binario, BinarioTipo},
    literal::{Literal, LiteralTipo},
    token::Token,
    unario::{Unario, UnarioTipo},
};

#[derive(Debug)]
pub enum ErrExpr {
    LiteralComoPadre,
    BinarioInvalido,
    SinLiteral,
    MalTokenFinal,
    NoHayValor,
    DivisorCero,
}

#[derive(Debug)]
pub struct Expr {
    base: Option<Token>,
}

impl Expr {
    pub fn base() -> Self {
        Self { base: None }
    }

    pub fn insertar(&mut self, token: Token) -> Result<(), ErrExpr> {
        if let Some(ref mut valor) = self.base {
            if debe_heredar(valor, &token) {
                valor.heredar(token)?;
            } else {
                self.intercambiar(token)?;
            }
        } else {
            self.base = Some(token);
        }
        Ok(())
    }

    pub fn operar(&self) -> Result<Literal, ErrExpr> {
        if let Some(valor) = &self.base {
            valor.operar()
        } else {
            Err(ErrExpr::NoHayValor)
        }
    }

    fn intercambiar(&mut self, mut token: Token) -> Result<(), ErrExpr> {
        let ant = self.base.take().unwrap();
        match token {
            Token::Binario(ref mut bin) => {
                bin.izq = Some(Box::new(ant));
            }
            Token::Unario(ref mut un) => {
                un.hijo = Some(Box::new(ant));
            }
            Token::Literal(_) => return Err(ErrExpr::LiteralComoPadre),
        }
        self.base = Some(token);
        Ok(())
    }
}

pub fn debe_heredar(pref: &mut Token, token: &Token) -> bool {
    match (pref, token) {
        (Token::Literal(_), Token::Binario(_)) => false,
        (Token::Unario(_), Token::Binario(_)) => false,
        (Token::Binario(b1), Token::Binario(b2)) if *b1 >= *b2 => false,
        (Token::Binario(_), Token::Binario(_)) => true,
        (_, Token::Literal(_)) => true,
        (_, Token::Unario(_)) => true,
    }
}
