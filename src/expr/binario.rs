use crate::ErrExpr;

use super::{binario_preferente, literal::Literal, token::Token};
use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct Binario {
    pub operador: BinarioTipo,
    pub izq: Option<Box<Token>>,
    pub der: Option<Box<Token>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinarioTipo {
    Incremento,
    Diferencia,
    Producto,
    Cociente,
    Resto,
}

impl Binario {
    pub fn base(operador: BinarioTipo) -> Self {
        Self {
            operador,
            izq: None,
            der: None,
        }
    }

    pub fn aplicar(&self, a: Literal, b: Literal) -> Result<Literal, ErrExpr> {
        match self.operador {
            BinarioTipo::Incremento => Ok(a.sumar(b)),
            BinarioTipo::Diferencia => Ok(a.restar(b)),
            BinarioTipo::Producto => Ok(a.multiplicar(b)),
            BinarioTipo::Cociente => a.dividir(b),
            BinarioTipo::Resto => a.modular(b),
        }
    }

    pub fn precedencia(&self) -> u8 {
        match self.operador {
            BinarioTipo::Incremento => 0,
            BinarioTipo::Diferencia => 0,
            BinarioTipo::Producto => 1,
            BinarioTipo::Cociente => 1,
            BinarioTipo::Resto => 1,
        }
    }

    pub fn heredar(&mut self, token: Token) -> Result<(), ErrExpr> {
        if let Some(ref mut valor) = self.der {
            if binario_preferente(valor, &token) {
                // QUEHACER
            } else {
                valor.heredar(token)?;
            }
        } else {
            self.der = Some(Box::new(token));
        }
        Ok(())
    }
}

impl Eq for Binario {}
impl PartialEq for Binario {
    fn eq(&self, otro: &Self) -> bool {
        self.operador == otro.operador
    }
}

impl PartialOrd for Binario {
    fn partial_cmp(&self, otro: &Self) -> Option<Ordering> {
        Some(self.cmp(otro))
    }
}

impl Ord for Binario {
    fn cmp(&self, otro: &Self) -> Ordering {
        if self.precedencia() > otro.precedencia() {
            Ordering::Greater
        } else if self.precedencia() == otro.precedencia() {
            Ordering::Equal
        } else {
            Ordering::Less
        }
    }
}
