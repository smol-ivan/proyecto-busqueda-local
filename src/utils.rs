use crate::busqueda_local::Matriz;

use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone)]
pub struct Caso {
    pub filas: i32,
    pub columnas: i32,
    pub tablero: Matriz,
}

impl Caso {
    pub fn se_puede_resolver(&self) -> bool {
        let mut contador = 0;
        for y in 0..self.filas {
            for x in 0..self.columnas {
                if self.tablero[y as usize][x as usize] == 0 {
                    contador += 1;
                }
            }
        }
        contador % 4 == 0
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
            filas,
            columnas,
            tablero,
        });

        id += 1;
    }

    casos
}

pub fn imprimir_tablero(tablero: &Matriz, tipo: u8) {
    let filas = tablero.len();
    let columnas = tablero[0].len();
    let etiqueta = match tipo {
        1 => "TABLERO_INICIAL",
        2 => "SOLUCION_INICIAL",
        3 => "SOLUCION_FINAL",
        4 => "NO_SOLUCION",
        _ => panic!("Tipo de tablero inválido: debe ser 1, 2 , 3 o 4"),
    };

    println!("{}_INICIO", etiqueta);
    println!("{}:{}", filas, columnas);
    for fila in tablero {
        for &celda in fila {
            print!("{} ", celda);
        }
        println!();
    }
    println!("{}_FIN", etiqueta);
}
