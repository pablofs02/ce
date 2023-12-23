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
    MalToken,
    SinSuficientesHijos,
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

    pub fn operar(&self) -> Result<String, ErrExpr> {
        if let Some(valor) = &self.base {
            match valor.operar() {
                Ok(lit) => match lit.valor {
                    LiteralTipo::Entero(n) => Ok(n.to_string()),
                    LiteralTipo::Flotante(f) => Ok(f.to_string()),
                },
                Err(err) => Err(err),
            }
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
            Token::Literal(_) => return Err(ErrExpr::MalToken),
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
