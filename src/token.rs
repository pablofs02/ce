use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub enum Token {
    Literal(Literal),
    Unario(Unario),
    Binario(Binario),
}

impl Eq for Token {}
impl PartialEq for Token {
    fn eq(&self, otro: &Token) -> bool {
        match self {
            Token::Literal(_) => match otro {
                Token::Literal(_) => true,
                _ => false,
            },
            Token::Unario(_) => match otro {
                Token::Unario(_) => true,
                _ => false,
            },
            Token::Binario(b1) => match otro {
                Token::Binario(b2) => b1.precedencia() == b2.precedencia(),
                _ => false,
            },
        }
    }
}

impl PartialOrd for Token {
    fn partial_cmp(&self, otro: &Token) -> Option<Ordering> {
        Some(self.cmp(otro))
    }
}

impl Ord for Token {
    // token es mayor si precedencia o comparar con literal
    fn cmp(&self, otro: &Self) -> Ordering {
        match self {
            Token::Binario(b1) => match otro {
                Token::Binario(b2) => {
                    if b1.precedencia() > b2.precedencia() {
                        Ordering::Greater
                    } else if b1.precedencia() == b2.precedencia() {
                        Ordering::Equal
                    } else {
                        Ordering::Less
                    }
                }
                _ => Ordering::Greater,
            },
            _ => Ordering::Less,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Literal {
    Entero(i128),
    Flotante(f64),
}

#[derive(Debug, Clone)]
pub enum Unario {
    Positivo,
    Negativo,
}

#[derive(Debug, Clone)]
pub enum Binario {
    Suma,
    Resta,
    Producto,
    Cociente,
    Resto,
}

impl Literal {
    pub fn sumar(&self, otro: Literal) -> Literal {
        match self {
            Literal::Entero(n1) => match otro {
                Literal::Entero(n2) => Literal::Entero(n1 + n2),
                Literal::Flotante(f2) => Literal::Flotante(*n1 as f64 + f2),
            },
            Literal::Flotante(f1) => match otro {
                Literal::Entero(n2) => Literal::Flotante(f1 + n2 as f64),
                Literal::Flotante(f2) => Literal::Flotante(f1 + f2),
            },
        }
    }

    pub fn restar(&self, otro: Literal) -> Literal {
        match self {
            Literal::Entero(n1) => match otro {
                Literal::Entero(n2) => Literal::Entero(n1 - n2),
                Literal::Flotante(f2) => Literal::Flotante(*n1 as f64 - f2),
            },
            Literal::Flotante(f1) => match otro {
                Literal::Entero(n2) => Literal::Flotante(f1 - n2 as f64),
                Literal::Flotante(f2) => Literal::Flotante(f1 - f2),
            },
        }
    }

    pub fn multiplicar(&self, otro: Literal) -> Literal {
        match self {
            Literal::Entero(n1) => match otro {
                Literal::Entero(n2) => Literal::Entero(n1 * n2),
                Literal::Flotante(f2) => Literal::Flotante(*n1 as f64 * f2),
            },
            Literal::Flotante(f1) => match otro {
                Literal::Entero(n2) => Literal::Flotante(f1 * n2 as f64),
                Literal::Flotante(f2) => Literal::Flotante(f1 * f2),
            },
        }
    }

    pub fn dividir(&self, otro: Literal) -> Literal {
        match self {
            Literal::Entero(n1) => match otro {
                Literal::Entero(n2) if n1 % n2 == 0 => Literal::Entero(n1 / n2),
                Literal::Entero(n2) => Literal::Flotante(*n1 as f64 / n2 as f64),
                Literal::Flotante(f2) => Literal::Flotante(*n1 as f64 / f2),
            },
            Literal::Flotante(f1) => match otro {
                Literal::Entero(n2) => Literal::Flotante(f1 / n2 as f64),
                Literal::Flotante(f2) => Literal::Flotante(f1 / f2),
            },
        }
    }

    pub fn modular(&self, otro: Literal) -> Literal {
        match self {
            Literal::Entero(n1) => match otro {
                Literal::Entero(n2) => Literal::Entero(n1 % n2),
                Literal::Flotante(f2) => Literal::Flotante(*n1 as f64 % f2),
            },
            Literal::Flotante(f1) => match otro {
                Literal::Entero(n2) => Literal::Flotante(f1 % n2 as f64),
                Literal::Flotante(f2) => Literal::Flotante(f1 % f2),
            },
        }
    }
}

impl Unario {
    pub fn aplicar(&self, valor: Literal) -> Literal {
        match self {
            Unario::Positivo => valor,
            Unario::Negativo => match valor {
                Literal::Entero(n) => Literal::Entero(-n),
                Literal::Flotante(f) => Literal::Flotante(-f),
            },
        }
    }
}

impl Binario {
    pub fn aplicar(&self, a: Literal, b: Literal) -> Literal {
        match self {
            Binario::Suma => a.sumar(b),
            Binario::Resta => a.restar(b),
            Binario::Producto => a.multiplicar(b),
            Binario::Cociente => a.dividir(b),
            Binario::Resto => a.modular(b),
        }
    }

    pub fn precedencia(&self) -> u8 {
        match self {
            Binario::Suma => 0,
            Binario::Resta => 0,
            Binario::Producto => 1,
            Binario::Cociente => 1,
            Binario::Resto => 1,
        }
    }
}
