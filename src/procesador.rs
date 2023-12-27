use crate::{expr::*, ErrExpr};

#[derive(Debug)]
struct Procesador {
    estado: Buscando,
    token: String,
    vec_tok: Vec<Token>,
    vec_sub: Vec<String>,
    binario: bool,
}

#[derive(Debug, PartialEq)]
enum Buscando {
    LiteralOUnario,
    Entero,
    Flotante,
    Binario,
}

pub fn evaluar(cad: &str) -> Result<String, ErrExpr> {
    let toks = tokenizar(cad)?;
    let res = calcular(toks)?;
    match res.valor {
        LiteralTipo::Entero(n) => Ok(n.to_string()),
        LiteralTipo::Flotante(f) => Ok(f.to_string()),
    }
}

fn tokenizar(cad: &str) -> Result<Vec<Token>, ErrExpr> {
    let mut procesador = Procesador {
        estado: Buscando::LiteralOUnario,
        token: String::new(),
        vec_tok: vec![],
        vec_sub: vec![],
        binario: false,
    };
    for c in cad.chars() {
        clasificar_char(c, &mut procesador)?;
    }
    match procesador.estado {
        _ if !procesador.vec_tok.is_empty() && !procesador.token.is_empty() => {
            if procesador.token.contains('.') {
                procesador
                    .vec_tok
                    .push(Token::Literal(Literal::base(LiteralTipo::Flotante(
                        procesador.token.parse().unwrap(),
                    ))))
            } else {
                procesador
                    .vec_tok
                    .push(Token::Literal(Literal::base(LiteralTipo::Entero(
                        procesador.token.parse().unwrap(),
                    ))))
            }
        }
        Buscando::Entero => {
            procesador
                .vec_tok
                .push(Token::Literal(Literal::base(LiteralTipo::Entero(
                    procesador.token.parse().unwrap(),
                ))))
        }
        Buscando::Flotante => {
            procesador
                .vec_tok
                .push(Token::Literal(Literal::base(LiteralTipo::Flotante(
                    procesador.token.parse().unwrap(),
                ))))
        }
        _ => (),
    }
    Ok(procesador.vec_tok)
}

fn calcular(vec: Vec<Token>) -> Result<Literal, ErrExpr> {
    let mut val = Expr::base();
    for elem in vec {
        val.insertar(elem)?;
    }
    val.operar()
}

fn clasificar_char(c: char, proc: &mut Procesador) -> Result<(), ErrExpr> {
    if !proc.vec_sub.is_empty() {
        match c {
            ')' => {
                proc.estado = Buscando::Binario;
                let sub = proc.vec_sub.pop().unwrap();
                if proc.vec_sub.is_empty() {
                    proc.vec_tok
                        .push(Token::Literal(calcular(tokenizar(sub.as_str())?)?));
                    return Ok(());
                }
                proc.vec_sub
                    .last_mut()
                    .unwrap()
                    .push_str(&evaluar(&sub.as_str())?);
                return Ok(());
            }
            '(' => {
                if !proc.binario && proc.estado == Buscando::Binario {
                    proc.vec_sub.last_mut().unwrap().push('*')
                }
                proc.vec_sub.push(String::new())
            }
            '+' | '-' | '*' | '·' | '/' | '%' => {
                proc.binario = true;
                proc.vec_sub.last_mut().unwrap().push(c)
            }
            ' ' => proc.vec_sub.last_mut().unwrap().push(c),
            _ => {
                proc.binario = false;
                proc.vec_sub.last_mut().unwrap().push(c)
            }
        }
        return Ok(());
    }
    match proc.estado {
        Buscando::LiteralOUnario => match c {
            '0'..='9' => {
                proc.token.push(c);
                proc.estado = Buscando::Entero;
            }
            '.' => {
                proc.token.push('.');
                proc.estado = Buscando::Flotante;
            }
            '+' => proc
                .vec_tok
                .push(Token::Unario(Unario::base(UnarioTipo::Positivo))),
            '-' => proc
                .vec_tok
                .push(Token::Unario(Unario::base(UnarioTipo::Negativo))),
            '(' => proc.vec_sub.push(String::new()),
            ' ' => return Ok(()),
            _ => return Err(ErrExpr::SinLiteral),
        },
        Buscando::Entero => match c {
            '0'..='9' => {
                proc.token.push(c);
                proc.estado = Buscando::Entero;
            }
            '.' | ',' => {
                proc.token.push('.');
                proc.estado = Buscando::Flotante;
            }
            _ => {
                proc.vec_tok
                    .push(Token::Literal(Literal::base(LiteralTipo::Entero(
                        proc.token.parse().unwrap(),
                    ))));
                proc.token = String::new();
                proc.estado = Buscando::Binario;
                clasificar_char(c, proc)?;
            }
        },
        Buscando::Flotante => match c {
            '0'..='9' => {
                proc.token.push(c);
            }
            _ => {
                proc.vec_tok
                    .push(Token::Literal(Literal::base(LiteralTipo::Flotante(
                        proc.token.parse().unwrap(),
                    ))));
                proc.token = String::new();
                proc.estado = Buscando::Binario;
                clasificar_char(c, proc)?;
            }
        },
        Buscando::Binario => {
            let bin = match c {
                ' ' => return Ok(()),
                '+' => Binario::base(BinarioTipo::Incremento),
                '-' => Binario::base(BinarioTipo::Diferencia),
                '*' | '·' => Binario::base(BinarioTipo::Producto),
                '/' => Binario::base(BinarioTipo::Cociente),
                '%' => Binario::base(BinarioTipo::Resto),
                '(' => {
                    proc.vec_sub.push(String::new());
                    Binario::base(BinarioTipo::Producto)
                }
                _ => return Err(ErrExpr::BinarioInvalido),
            };
            proc.vec_tok.push(Token::Binario(bin));
            proc.estado = Buscando::LiteralOUnario;
        }
    }
    Ok(())
}
