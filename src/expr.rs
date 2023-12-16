use crate::token::{Literal, Token};

#[derive(Debug)]
pub enum ErrExpr {
    MalToken,
    SinSuficientesHijos,
    NoHayValor,
}

#[derive(Debug)]
pub struct Expr {
    base: Option<Nodo>,
}

#[derive(Debug)]
struct Nodo {
    val: Token,
    hijos: Vec<Nodo>,
}

impl Expr {
    pub fn base() -> Self {
        Self { base: None }
    }

    pub fn insertar(&mut self, token: Token) {
        if let Some(ref mut nodo) = self.base {
            if let Token::Binario(_) = token {
                if token >= nodo.val {
                    self.intercambiar(token);
                    return;
                }
            }
            nodo.insertar(token);
        } else {
            self.base = Some(Nodo::base(token));
        }
    }

    pub fn operar(&self) -> Result<String, ErrExpr> {
        if let Some(nodo) = &self.base {
            match nodo.operar() {
                Ok(Literal::Entero(n)) => Ok(n.to_string()),
                Ok(Literal::Flotante(f)) => Ok(f.to_string()),
                Err(err) => Err(err),
            }
        } else {
            Err(ErrExpr::NoHayValor)
        }
    }

    fn intercambiar(&mut self, token: Token) {
        let mut nodo = Nodo::base(token);
        let ant = self.base.take().unwrap();
        nodo.hijos.push(ant);
        self.base = Some(nodo);
    }
}

impl Nodo {
    pub fn base(token: Token) -> Self {
        Self {
            val: token,
            hijos: Vec::new(),
        }
    }

    // Self:Token
    // ----------
    // U   :L -> Bajar
    // U   :U -> Bajar
    // B   :L -> Bajar
    // B   :U -> Bajar
    // B   :B -> Cambio si <= sino bajar
    pub fn insertar(&mut self, token: Token) {
        if let Token::Unario(_) = &self.val {
            match self.hijos.get_mut(0) {
                Some(hijo) => {
                    hijo.insertar(token);
                }
                None => self.hijos.push(Nodo::base(token)),
            }
        } else if let Token::Binario(_) = self.val {
            match self.hijos.get_mut(1) {
                Some(hijo) => {
                    hijo.insertar(token);
                }
                None => self.hijos.push(Nodo::base(token)),
            }
        }
    }

    pub fn operar(&self) -> Result<Literal, ErrExpr> {
        match &self.val {
            Token::Literal(lit) => Ok(lit.clone()),
            Token::Unario(un) => match self.hijos.get(0) {
                Some(hijo) => Ok(un.aplicar(hijo.operar()?)),
                None => Err(ErrExpr::SinSuficientesHijos),
            },
            Token::Binario(bi) => match self.hijos.get(0) {
                Some(hijo_izq) => match self.hijos.get(1) {
                    Some(hijo_der) => Ok(bi.aplicar(hijo_izq.operar()?, hijo_der.operar()?)),
                    None => Err(ErrExpr::SinSuficientesHijos),
                },
                None => Err(ErrExpr::SinSuficientesHijos),
            },
        }
    }
}
