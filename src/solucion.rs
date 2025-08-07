use crate::pieza::Pieza;
use crate::tablero::Tablero;

#[derive(Clone)]
pub struct Solucion {
    pub matriz: Tablero,
    pub piezas: Vec<Pieza>,
}

impl Solucion {
    pub fn eliminar_pieza(&mut self, index: usize) -> (Pieza, Solucion) {
        let mut solucion = self.clone();
        let pieza = solucion.piezas.remove(index);
        solucion.matriz.eliminar_pieza(&pieza);
        (pieza, solucion)
    }

    pub fn agregar_pieza(&mut self, pieza: Pieza) {
        self.matriz.agregar_pieza(&pieza);
        self.piezas.push(pieza);
    }

    pub fn casilleros_libres(&self) -> Vec<(i32, i32)> {
        let mut vec = Vec::new();
        for y in 0..self.matriz.filas {
            for x in 0..self.matriz.columnas {
                if self.matriz.matriz[y as usize][x as usize] == 0 {
                    vec.push((x, y));
                }
            }
        }
        vec
    }

    /// FUNCION OBJETIVO
    pub fn celdas_cubiertas(&self) -> i32 {
        let tablero = &self.matriz;
        let mut contador = 0;
        for y in 0..tablero.filas {
            for x in 0..tablero.columnas {
                if tablero.matriz[y as usize][x as usize] != 0 {
                    contador += 1;
                }
            }
        }
        contador
    }
}
