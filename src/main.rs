use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Caso {
    id: u8,
    filas: u8,
    columnas: u8,
    tablero: Vec<Vec<char>>,
}

impl Caso {
    fn display(&self) {
        for fila in &self.tablero {
            println!("{:?}", fila);
        }
        println!();
    }
}

fn display_casos(casos: &Vec<Caso>) {
    for caso in casos {
        println!("Caso {} ({}x{}): ", caso.id, caso.filas, caso.columnas);
        caso.display();
    }
}

fn leer_casos(path: &String) -> Vec<Caso> {
    let file = File::open(path).expect("No se pudo abrir el archivo de casos.");
    let reader = BufReader::new(file);

    let mut casos = Vec::new();
    let mut id = 1;
    let mut lineas = reader.lines().filter_map(Result::ok).peekable();

    while let Some(linea) = lineas.next() {
        let linea = linea.trim();

        // Ignorar comentarios y lineas vacias
        if linea.is_empty() || linea.starts_with("# Caso") {
            continue;
        }

        // Linea con dimensiones
        let dimensiones: Vec<u8> = linea
            .split_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect();

        if dimensiones.len() != 2 {
            panic!(
                "Hubo un error leyendo las dimensiones del problema. {:?}",
                dimensiones
            );
        }

        let (filas, columnas) = (dimensiones[0], dimensiones[1]);

        let mut tablero = Vec::new();
        for _ in 0..filas {
            if let Some(linea_tablero) = lineas.next() {
                let fila = linea_tablero
                    .trim()
                    .chars()
                    .map(|c| if c == '#' { '*' } else { 'o' })
                    .collect::<Vec<char>>();
                if fila.len() != columnas as usize {
                    panic!("La fila del tablero no tiene la cantidad esperada de columnas");
                }
                tablero.push(fila);
            } else {
                panic!("Salida temprana");
            }
        }

        casos.push(Caso {
            id,
            filas,
            columnas,
            tablero,
        });

        id += 1;
    }

    casos
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Uso: ${} <file_path_to_cases>", args[0]);
        return;
    }
    println!("\x1b[1;31mHello, im red\x1b[0m");
    println!("Hello, im not");
    let casos = leer_casos(&args[1]);
    display_casos(&casos);
}
