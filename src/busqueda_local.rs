use crate::pieza::{Pieza, Rotacion};
use crate::solucion::Solucion;
use crate::tablero::Tablero;
use crate::utils::{Caso, imprimir_tablero};
use crate::vecindario::GeneracionVecinos;

use rand::prelude::*;

pub type Matriz = Vec<Vec<i8>>;
// 0 Si la casilla es libre 1 si esta ocupada

const MAX_INTENTOS: i32 = 30;

fn solucion_inicial_aleatoria(matriz: Matriz, filas: i32, columnas: i32) -> Solucion {
    let mut tablero = Tablero {
        matriz,
        filas,
        columnas,
    };
    let mut intento: i32 = 0;
    let mut piezas: Vec<Pieza> = Vec::new();
    let mut id = 1;

    let mut rng = rand::rng();
    while intento < MAX_INTENTOS {
        let x: i32 = rng.random_range(0..columnas);
        let y: i32 = rng.random_range(0..filas);

        let rotacion: Rotacion = match rng.random_range(1..=4) {
            1 => Rotacion::Deg0,
            2 => Rotacion::Deg90,
            3 => Rotacion::Deg180,
            4 => Rotacion::Deg270,
            _ => panic!("Algo salio mal"),
        };

        let pieza = Pieza::new(id, x, y, rotacion);

        if tablero.es_valido(&pieza) {
            tablero.agregar_pieza(&pieza);
            piezas.push(pieza);
            id += 1;
        }
        intento += 1;
    }

    imprimir_tablero(&tablero.matriz, 2);
    Solucion {
        matriz: tablero,
        piezas,
    }
}

pub fn cubrir(
    caso: Caso,
    iteraciones: usize,
    generacion_vecinos: &dyn GeneracionVecinos,
) -> Solucion {
    let mut solucion = solucion_inicial_aleatoria(caso.tablero, caso.filas, caso.columnas);
    let mut solucion_mejor = solucion.clone();

    for i in 1..=iteraciones {
        let mut mejor_vecino = solucion.clone();

        let vecinos = generacion_vecinos.generar_vecinos(&mut solucion);

        for vecino in vecinos {
            if vecino.celdas_cubiertas() > mejor_vecino.celdas_cubiertas() {
                mejor_vecino = vecino;
            }
        }

        // Hay algun lugar visible que sea mas alto que donde estoy?
        // Vale la pena moverme?
        if mejor_vecino.celdas_cubiertas() > solucion.celdas_cubiertas() {
            solucion = mejor_vecino;

            // Esta nueva posicion es mas alta que mi record personal?
            if solucion.celdas_cubiertas() > solucion_mejor.celdas_cubiertas() {
                solucion_mejor = solucion.clone();
            }
        } else {
            println!("Maximo local alcanzado en iteracion {}", i);
            break;
        }
    }
    solucion_mejor
}
