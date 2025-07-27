use crate::busqueda_local::Matriz;

use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone)]
pub struct Caso {
    pub id: i32,
    pub filas: i32,
    pub columnas: i32,
    pub tablero: Matriz,
}

impl Caso {
    pub fn display(&self) {
        for fila in &self.tablero {
            println!("{:?}", fila);
        }
        println!();
    }
}

pub fn display_casos(casos: &Vec<Caso>, print: bool, num_case: i8) {
    if print {
        if num_case != -1 {
            let caso = &casos[num_case as usize];
            println!("Caso {} ({}x{}): ", caso.id, caso.filas, caso.columnas);
            caso.display();
        } else {
            for caso in casos {
                println!("Caso {} ({}x{}): ", caso.id, caso.filas, caso.columnas);
                caso.display();
            }
        }
    }
}

pub fn leer_casos(path: &String) -> Vec<Caso> {
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
        let dimensiones: Vec<i32> = linea
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
                    .map(|c| if c == '#' { -1 } else { 0 })
                    .collect::<Vec<i8>>();
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
