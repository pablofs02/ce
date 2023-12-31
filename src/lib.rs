mod expr;
mod procesador;

pub use expr::ErrExpr;
pub use procesador::evaluar;

#[cfg(test)]
mod insertar {
    use crate::evaluar;

    #[test]
    fn valor() {
        assert_eq!(evaluar("1").unwrap(), "1".to_owned());
    }

    #[test]
    fn negativo() {
        assert_eq!(evaluar("- 2").unwrap(), "-2".to_owned());
    }

    #[test]
    fn suma() {
        assert_eq!(evaluar("2 + 2").unwrap(), "4".to_owned());
    }

    #[test]
    fn resta() {
        assert_eq!(evaluar("3 - 2").unwrap(), "1".to_owned());
    }

    #[test]
    fn producto() {
        assert_eq!(evaluar("3 * 2").unwrap(), "6".to_owned());
    }

    #[test]
    fn div_entera() {
        assert_eq!(evaluar("18 / 2").unwrap(), "9".to_owned());
    }

    #[test]
    fn varios() {
        assert_eq!(evaluar("3 - 2 + 3").unwrap(), "4".to_owned());
    }

    #[test]
    fn orden() {
        assert_eq!(evaluar("3 - 2 * 3").unwrap(), "-3".to_owned());
    }

    #[test]
    fn varios_con_orden() {
        assert_eq!(evaluar("1 * 2 + 3 * 4").unwrap(), "14".to_owned());
    }
}
