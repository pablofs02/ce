use calexp::evaluar;

fn main() {
    let argos: Vec<String> = std::env::args().collect();
    if argos.len() > 1 {
        println!("{}", evaluar(&argos.get(1..).unwrap().join(" ")).unwrap());
    }
}
