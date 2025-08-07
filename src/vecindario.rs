use crate::pieza::{Pieza, Rotacion};
use crate::solucion::Solucion;

pub trait GeneracionVecinos {
    fn generar_vecinos(&self, solucion: &mut Solucion) -> Vec<Solucion>;
}

pub struct Sencillo;

impl GeneracionVecinos for Sencillo {
    fn generar_vecinos(&self, solucion: &mut Solucion) -> Vec<Solucion> {
        let mut vecindario = Vec::new();

        // MODIFICACIÓN: Agregar una nueva pieza
        if solucion.matriz.casillas_vacias() != 0 {
            let mut nueva_pieza = Pieza::new(solucion.piezas.len() as i8 + 1, 0, 0, Rotacion::Deg0);
            for (x, y) in solucion.casilleros_libres() {
                nueva_pieza.mover(x, y);
                for _ in 0..5 {
                    if solucion.matriz.es_valido(&nueva_pieza) {
                        let mut solucion_temp = solucion.clone();
                        solucion_temp.agregar_pieza(nueva_pieza.clone());
                        vecindario.push(solucion_temp);
                    }
                    nueva_pieza.rotar();
                }
            }
        }

        for i in 0..solucion.piezas.len() {
            let (pieza_copy, solucion_copy) = solucion.eliminar_pieza(i);

            // MODIFICACION: Rotar pieza actual
            let mut temp_pieza = pieza_copy.clone();
            temp_pieza.id = solucion_copy.piezas.len() as i8 + 1;

            for _ in 0..4 {
                temp_pieza.rotar();
                if solucion_copy.matriz.es_valido(&temp_pieza) {
                    let mut solucion_temp = solucion_copy.clone();
                    solucion_temp.agregar_pieza(temp_pieza.clone());
                    vecindario.push(solucion_temp);
                }
                temp_pieza.rotar();
            }

            // MODIFICACION: Mover una pieza
            temp_pieza = pieza_copy.clone();

            for (x, y) in solucion_copy.casilleros_libres() {
                temp_pieza.mover(x, y);
                if solucion_copy.matriz.es_valido(&temp_pieza) {
                    let mut solucion_temp = solucion_copy.clone();
                    solucion_temp.agregar_pieza(temp_pieza.clone());
                    vecindario.push(solucion_temp);
                }
            }
        }
        vecindario
    }
}

pub struct Mejora;

impl GeneracionVecinos for Mejora {
    fn generar_vecinos(&self, solucion: &mut Solucion) -> Vec<Solucion> {
        todo!();
    }
}
