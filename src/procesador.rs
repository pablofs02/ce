use crate::{
    expr::{Binario, BinarioTipo, Expr, Literal, LiteralTipo, Token, Unario, UnarioTipo},
    ErrExpr,
};

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
    let mut estado = Buscando::LiteralOUnario;
    let mut token = String::new();
    let mut vec_tok = vec![];
    let mut vec_sub = vec![];
    for c in cad.chars() {
        clasificar_char(c, &mut estado, &mut vec_tok, &mut vec_sub, &mut token)?;
    }
    match estado {
        _ if !vec_tok.is_empty() && !token.is_empty() => {
            if token.contains('.') {
                vec_tok.push(Token::Literal(Literal::base(LiteralTipo::Flotante(
                    token.parse().unwrap(),
                ))))
            } else {
                vec_tok.push(Token::Literal(Literal::base(LiteralTipo::Entero(
                    token.parse().unwrap(),
                ))))
            }
        }
        Buscando::Entero => vec_tok.push(Token::Literal(Literal::base(LiteralTipo::Entero(
            token.parse().unwrap(),
        )))),
        Buscando::Flotante => vec_tok.push(Token::Literal(Literal::base(LiteralTipo::Flotante(
            token.parse().unwrap(),
        )))),
        _ => (),
    }
    Ok(vec_tok)
}

fn calcular(vec: Vec<Token>) -> Result<Literal, ErrExpr> {
    let mut val = Expr::base();
    for elem in vec {
        val.insertar(elem)?;
    }
    val.operar()
}

fn clasificar_char(
    c: char,
    estado: &mut Buscando,
    vec_tok: &mut Vec<Token>,
    vec_sub: &mut Vec<String>,
    token: &mut String,
) -> Result<(), ErrExpr> {
    if !vec_sub.is_empty() {
        match c {
            ')' => {
                *estado = Buscando::Binario;
                let sub = vec_sub.pop().unwrap();
                if vec_sub.is_empty() {
                    vec_tok.push(Token::Literal(calcular(tokenizar(sub.as_str())?)?));
                    return Ok(());
                }
                vec_sub
                    .last_mut()
                    .unwrap()
                    .push_str(&evaluar(&sub.as_str())?);
                return Ok(());
            }
            '(' => vec_sub.push(String::new()),
            _ => vec_sub.last_mut().unwrap().push(c),
        }
        return Ok(());
    }
    match estado {
        Buscando::LiteralOUnario => match c {
            '0'..='9' => {
                token.push(c);
                *estado = Buscando::Entero;
            }
            '.' => {
                token.push('.');
                *estado = Buscando::Flotante;
            }
            '+' => vec_tok.push(Token::Unario(Unario::base(UnarioTipo::Positivo))),
            '-' => vec_tok.push(Token::Unario(Unario::base(UnarioTipo::Negativo))),
            '(' => vec_sub.push(String::new()),
            ' ' => return Ok(()),
            _ => return Err(ErrExpr::SinLiteral),
        },
        Buscando::Entero => match c {
            '0'..='9' => {
                token.push(c);
                *estado = Buscando::Entero;
            }
            '.' | ',' => {
                token.push('.');
                *estado = Buscando::Flotante;
            }
            _ => {
                vec_tok.push(Token::Literal(Literal::base(LiteralTipo::Entero(
                    token.parse().unwrap(),
                ))));
                *token = String::new();
                *estado = Buscando::Binario;
                clasificar_char(c, estado, vec_tok, vec_sub, token)?;
            }
        },
        Buscando::Flotante => match c {
            '0'..='9' => {
                token.push(c);
            }
            _ => {
                vec_tok.push(Token::Literal(Literal::base(LiteralTipo::Flotante(
                    token.parse().unwrap(),
                ))));
                *token = String::new();
                *estado = Buscando::Binario;
                clasificar_char(c, estado, vec_tok, vec_sub, token)?;
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
                    vec_sub.push(String::new());
                    Binario::base(BinarioTipo::Producto)
                }
                _ => return Err(ErrExpr::BinarioInvalido),
            };
            vec_tok.push(Token::Binario(bin));
            *estado = Buscando::LiteralOUnario;
        }
    }
    Ok(())
}
