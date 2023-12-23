use crate::ErrExpr;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Literal {
    pub valor: LiteralTipo,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum LiteralTipo {
    Entero(i128),
    Flotante(f64),
}

impl Literal {
    pub fn base(valor: LiteralTipo) -> Self {
        Self { valor }
    }

    pub fn sumar(&self, otro: Literal) -> Literal {
        match self.valor {
            LiteralTipo::Entero(n1) => match otro.valor {
                LiteralTipo::Entero(n2) => Literal::base(LiteralTipo::Entero(n1 + n2)),
                LiteralTipo::Flotante(f2) => Literal::base(LiteralTipo::Flotante(n1 as f64 + f2)),
            },
            LiteralTipo::Flotante(f1) => match otro.valor {
                LiteralTipo::Entero(n2) => Literal::base(LiteralTipo::Flotante(f1 + n2 as f64)),
                LiteralTipo::Flotante(f2) => Literal::base(LiteralTipo::Flotante(f1 + f2)),
            },
        }
    }

    pub fn restar(&self, otro: Literal) -> Literal {
        match self.valor {
            LiteralTipo::Entero(n1) => match otro.valor {
                LiteralTipo::Entero(n2) => Literal::base(LiteralTipo::Entero(n1 - n2)),
                LiteralTipo::Flotante(f2) => Literal::base(LiteralTipo::Flotante(n1 as f64 - f2)),
            },
            LiteralTipo::Flotante(f1) => match otro.valor {
                LiteralTipo::Entero(n2) => Literal::base(LiteralTipo::Flotante(f1 - n2 as f64)),
                LiteralTipo::Flotante(f2) => Literal::base(LiteralTipo::Flotante(f2 - f2)),
            },
        }
    }

    pub fn multiplicar(&self, otro: Literal) -> Literal {
        match self.valor {
            LiteralTipo::Entero(n1) => match otro.valor {
                LiteralTipo::Entero(n2) => Literal::base(LiteralTipo::Entero(n1 * n2)),
                LiteralTipo::Flotante(f2) => Literal::base(LiteralTipo::Flotante(n1 as f64 * f2)),
            },
            LiteralTipo::Flotante(f1) => match otro.valor {
                LiteralTipo::Entero(n2) => Literal::base(LiteralTipo::Flotante(f1 * n2 as f64)),
                LiteralTipo::Flotante(f2) => Literal::base(LiteralTipo::Flotante(f1 * f2)),
            },
        }
    }

    pub fn dividir(&self, otro: Literal) -> Result<Literal, ErrExpr> {
        if let LiteralTipo::Entero(0) = otro.valor {
            return Err(ErrExpr::DivisorCero);
        }
        match self.valor {
            LiteralTipo::Entero(n1) => match otro.valor {
                LiteralTipo::Entero(n2) if n1 % n2 == 0 => {
                    Ok(Literal::base(LiteralTipo::Entero(n1 / n2)))
                }
                LiteralTipo::Entero(n2) => {
                    Ok(Literal::base(LiteralTipo::Flotante(n1 as f64 / n2 as f64)))
                }
                LiteralTipo::Flotante(f2) => {
                    Ok(Literal::base(LiteralTipo::Flotante(n1 as f64 / f2)))
                }
            },
            LiteralTipo::Flotante(f1) => match otro.valor {
                LiteralTipo::Entero(n2) => Ok(Literal::base(LiteralTipo::Flotante(f1 / n2 as f64))),
                LiteralTipo::Flotante(f2) => Ok(Literal::base(LiteralTipo::Flotante(f1 / f2))),
            },
        }
    }

    pub fn modular(&self, otro: Literal) -> Result<Literal, ErrExpr> {
        match self.valor {
            LiteralTipo::Entero(n1) => match otro.valor {
                LiteralTipo::Entero(n2) => Ok(Literal::base(LiteralTipo::Entero(n1 % n2))),
                LiteralTipo::Flotante(f2) => {
                    Ok(Literal::base(LiteralTipo::Flotante(n1 as f64 % f2)))
                }
            },
            LiteralTipo::Flotante(f1) => match otro.valor {
                LiteralTipo::Entero(n2) => Ok(Literal::base(LiteralTipo::Flotante(f1 % n2 as f64))),
                LiteralTipo::Flotante(f2) => Ok(Literal::base(LiteralTipo::Flotante(f1 % f2))),
            },
        }
    }
}
