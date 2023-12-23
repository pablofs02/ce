use crate::ErrExpr;

use super::{literal::Literal, token::Token, LiteralTipo};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Unario {
    pub operador: UnarioTipo,
    pub hijo: Option<Box<Token>>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum UnarioTipo {
    Positivo,
    Negativo,
}

impl Unario {
    pub fn base(operador: UnarioTipo) -> Self {
        Self {
            operador,
            hijo: None,
        }
    }

    pub fn aplicar(&self, valor: Literal) -> Literal {
        match self.operador {
            UnarioTipo::Positivo => valor,
            UnarioTipo::Negativo => match valor.valor {
                LiteralTipo::Entero(n) => Literal::base(LiteralTipo::Entero(-n)),
                LiteralTipo::Flotante(f) => Literal::base(LiteralTipo::Flotante(-f)),
            },
        }
    }

    pub fn heredar(&mut self, token: Token) -> Result<(), ErrExpr> {
        if let Some(ref mut valor) = self.hijo {
            valor.heredar(token)?;
        } else {
            self.hijo = Some(Box::new(token));
        }
        Ok(())
    }
}
