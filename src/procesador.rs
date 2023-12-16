use crate::{token::{Token, Literal, Unario, Binario}, ErrExpr, expr::Expr};

enum Buscando {
    LiteralOUnario,
    Entero,
    Flotante,
    Binario,
}

pub fn tokenizar(cad: &str) -> Result<Vec<Token>, ErrExpr> {
    let mut estado = Buscando::LiteralOUnario;
    let mut token = String::new();
    let mut vec = vec![];
    for c in cad.chars() {
        clasificar_char(c, &mut estado, &mut vec, &mut token)?;
    }
    match estado {
        Buscando::Entero => vec.push(Token::Literal(Literal::Entero(token.parse().unwrap()))),
        Buscando::Flotante => vec.push(Token::Literal(Literal::Flotante(token.parse().unwrap()))),
        _ => return Err(ErrExpr::MalToken),
    }
    Ok(vec)
}

pub fn calcular(vec: Vec<Token>) -> Result<String, ErrExpr> {
    let mut val = Expr::base();
    for elem in vec {
        val.insertar(elem);
    }
    val.operar()
}

fn clasificar_char(
    c: char,
    estado: &mut Buscando,
    vec: &mut Vec<Token>,
    token: &mut String,
) -> Result<(), ErrExpr> {
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
            '+' => vec.push(Token::Unario(Unario::Positivo)),
            '-' => vec.push(Token::Unario(Unario::Negativo)),
            ' ' => return Ok(()),
            _ => return Err(ErrExpr::MalToken),
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
                vec.push(Token::Literal(Literal::Entero(token.parse().unwrap())));
                *token = String::new();
                *estado = Buscando::Binario;
                clasificar_char(c, estado, vec, token)?;
            }
        },
        Buscando::Flotante => match c {
            '0'..='9' => {
                token.push(c);
                *estado = Buscando::Entero;
            }
            _ => {
                vec.push(Token::Literal(Literal::Flotante(token.parse().unwrap())));
                *token = String::new();
                *estado = Buscando::Binario;
                clasificar_char(c, estado, vec, token)?;
            }
        },
        Buscando::Binario => {
            let bin = match c {
                ' ' => return Ok(()),
                '+' => Binario::Suma,
                '-' => Binario::Resta,
                '*' => Binario::Producto,
                '/' => Binario::Cociente,
                '%' => Binario::Resto,
                _ => return Err(ErrExpr::MalToken),
            };
            vec.push(Token::Binario(bin));
            *estado = Buscando::LiteralOUnario;
        }
    }
    Ok(())
}
