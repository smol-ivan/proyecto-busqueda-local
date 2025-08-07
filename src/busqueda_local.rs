use crate::utils::{Caso, imprimir_tablero};

use rand::prelude::*;

pub type Matriz = Vec<Vec<i8>>;
// 0 Si la casilla es libre 1 si esta ocupada

const MAX_INTENTOS: i32 = 10;

#[derive(Clone, Debug)]
enum Rotacion {
    Deg0,
    Deg90,
    Deg180,
    Deg270,
}

#[derive(Clone, Debug)]
struct Pieza {
    id: i8,
    x: i32,
    y: i32,
    rotacion: Rotacion,
}

impl Pieza {
    fn new(id: i8, x: i32, y: i32, rotacion: Rotacion) -> Pieza {
        Pieza { id, x, y, rotacion }
    }

    fn mover(&mut self, dx: i32, dy: i32) {
        self.x = dx;
        self.y = dy;
    }

    fn rotar(&mut self) {
        self.rotacion = match self.rotacion {
            Rotacion::Deg0 => Rotacion::Deg90,
            Rotacion::Deg90 => Rotacion::Deg180,
            Rotacion::Deg180 => Rotacion::Deg270,
            Rotacion::Deg270 => Rotacion::Deg0,
        }
    }

    // Devolver las coordenadas absolutas de los bloques que ocupa
    fn bloques_ocupados(&self) -> Vec<(i32, i32)> {
        match self.rotacion {
            Rotacion::Deg90 => vec![
                (self.x, self.y),
                (self.x, self.y + 1),
                (self.x, self.y + 2),
                (self.x + 1, self.y),
            ],

            Rotacion::Deg0 => vec![
                (self.x, self.y),
                (self.x + 1, self.y),
                (self.x + 2, self.y),
                (self.x, self.y - 1),
            ],

            Rotacion::Deg270 => vec![
                (self.x, self.y),
                (self.x - 1, self.y),
                (self.x, self.y - 1),
                (self.x, self.y - 2),
            ],

            Rotacion::Deg180 => vec![
                (self.x, self.y),
                (self.x - 2, self.y),
                (self.x - 1, self.y),
                (self.x, self.y + 1),
            ],
        }
    }
}

#[derive(Debug, Clone)]
pub struct Tablero {
    pub matriz: Matriz,
    filas: i32,
    columnas: i32,
}

impl Tablero {
    fn casillas_vacias(&self) -> i32 {
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
    fn es_valido(&self, pieza: &Pieza) -> bool {
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

    fn agregar_pieza(&mut self, pieza: &Pieza) {
        for (x, y) in pieza.bloques_ocupados() {
            self.matriz[y as usize][x as usize] = pieza.id;
        }
    }

    fn eliminar_pieza(&mut self, pieza: &Pieza) {
        for (x, y) in pieza.bloques_ocupados() {
            self.matriz[y as usize][x as usize] = 0;
        }
    }
}

#[derive(Clone)]
pub struct Solucion {
    pub matriz: Tablero,
    piezas: Vec<Pieza>,
}

impl Solucion {
    fn eliminar_pieza(&mut self, index: usize) -> (Pieza, Solucion) {
        let mut solucion = self.clone();
        let pieza = solucion.piezas.remove(index);
        solucion.matriz.eliminar_pieza(&pieza);
        (pieza, solucion)
    }

    fn agregar_pieza(&mut self, pieza: Pieza) {
        self.matriz.agregar_pieza(&pieza);
        self.piezas.push(pieza);
    }

    fn casilleros_libres(&self) -> Vec<(i32, i32)> {
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
}

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
                for _ in 0..4 {
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

fn celdas_cubiertas(solucion: &Solucion) -> i32 {
    let tablero = &solucion.matriz;
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

pub fn cubrir(
    caso: Caso,
    iteraciones: usize,
    generacion_vecinos: &dyn GeneracionVecinos,
) -> Solucion {
    let mut solucion = solucion_inicial_aleatoria(caso.tablero, caso.filas, caso.columnas);
    let mut solucion_mejor = solucion.clone();

    for _i in 1..=iteraciones {
        let mut mejor_vecino = solucion.clone();

        let vecinos = generacion_vecinos.generar_vecinos(&mut solucion);

        for vecino in vecinos {
            if celdas_cubiertas(&vecino) > celdas_cubiertas(&mejor_vecino) {
                mejor_vecino = vecino;
            }
        }

        // Hay algun lugar visible que sea mas alto que donde estoy?
        // Vale la pena moverme?
        if celdas_cubiertas(&mejor_vecino) > celdas_cubiertas(&solucion) {
            solucion = mejor_vecino;

            // Esta nueva posicion es mas alta que mi record personal?
            if celdas_cubiertas(&solucion) > celdas_cubiertas(&solucion_mejor) {
                solucion_mejor = solucion.clone();
            }
        } else {
            break;
        }
    }
    solucion_mejor
}
