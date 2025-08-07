use crate::busqueda_local::Matriz;
use crate::pieza::Pieza;

#[derive(Debug, Clone)]
pub struct Tablero {
    pub matriz: Matriz,
    pub filas: i32,
    pub columnas: i32,
}

impl Tablero {
    pub fn casillas_vacias(&self) -> i32 {
        let mut contador = 0;
        for y in 0..self.filas {
            for x in 0..self.columnas {
                if self.matriz[y as usize][x as usize] == 0 {
                    contador += 1;
                }
            }
        }
        contador
    }

    // Verificamos si no hay colision
    pub fn es_valido(&self, pieza: &Pieza) -> bool {
        for (x, y) in pieza.bloques_ocupados() {
            // Verificar si esta dentro del tablero
            if x < 0 || y < 0 || x >= self.columnas || y >= self.filas {
                return false;
            }

            // Vericar que este libre la casilla
            if self.matriz[y as usize][x as usize] != 0 {
                return false;
            }
        }
        true
    }

    pub fn agregar_pieza(&mut self, pieza: &Pieza) {
        for (x, y) in pieza.bloques_ocupados() {
            self.matriz[y as usize][x as usize] = pieza.id;
        }
    }

    pub fn eliminar_pieza(&mut self, pieza: &Pieza) {
        for (x, y) in pieza.bloques_ocupados() {
            self.matriz[y as usize][x as usize] = 0;
        }
    }
}
