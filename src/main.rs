mod busqueda_local;
mod utils;

use crate::busqueda_local::*;
use crate::utils::*;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!(
            "Uso: ${} <file_path_to_cases> <print> <#_case> <iteraciones> <generar_vecinos>",
            args[0]
        );
        return;
    }
    let print: bool = args[2].parse().expect("bool true~false");
    let num_case: i8 = args[3]
        .parse()
        .expect("Expected a number of case. Up to 20");
    let iteraciones: i32 = args[4].parse().expect("Expected a number");
    let generar_vecinos: usize = args[5]
        .parse()
        .expect("Generacion de vecinos: 1- Sencillo, 2- Mejora");
    let generar_vecinos: Box<dyn GeneracionVecinos> = match generar_vecinos {
        1 => Box::new(Sencillo),
        2 => Box::new(Mejora),
        _ => panic!("Generacion de vecinos: 1- Sencillo, 2- Mejora"),
    };

    let casos = leer_casos(&args[1]);
    display_casos(&casos, print, num_case - 1);

    match num_case {
        -1 => {
            for i in 0..casos.len() {
                let _ = cubrir(
                    casos[i].clone(),
                    iteraciones as usize,
                    generar_vecinos.as_ref(),
                );
            }
        }
        _ => {
            let _ = cubrir(
                casos[(num_case - 1) as usize].clone(),
                iteraciones as usize,
                generar_vecinos.as_ref(),
            );
        }
    }
}
