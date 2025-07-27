use crate::utils::Caso;

use rand::prelude::*;

pub type Matriz = Vec<Vec<i8>>;
// 0 Si la casilla es libre 1 si esta ocupada

const MAX_INTENTOS: i32 = 30;

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
        self.x += dx;
        self.y += dy;
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

#[derive(Debug)]
struct Tablero {
    matriz: Matriz,
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
    fn es_valido(&self, pieza: &Pieza) -> bool {
        for (x, y) in pieza.bloques_ocupados() {
            // Verificar si esta dentro del tablero
            if x < 0 || y < 0 || x >= self.columnas || y >= self.filas {
                return false;
            }

            // Si esta ocupado la casilla
            if self.matriz[y as usize][x as usize] != 0 {
                return false;
            }
        }
        true
    }

    fn colocar_pieza(&mut self, pieza: &Pieza, piezas: &mut Vec<Pieza>) {
        for (x, y) in pieza.bloques_ocupados() {
            self.matriz[y as usize][x as usize] = pieza.id;
        }
        piezas.push(pieza.clone())
    }
}

struct Solucion {
    matriz: Tablero,
    piezas: Vec<Pieza>,
}

impl Solucion {
    fn display(&self) {
        for fila in &self.matriz.matriz {
            println!("{:?}", fila);
        }
        println!("Piezas: {:?}", self.piezas);
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
    loop {
        if intento > MAX_INTENTOS {
            break;
        }
        let x: i32 = rng.random_range(0..columnas);
        let y: i32 = rng.random_range(0..filas);

        let rotacion: Rotacion = match rng.random_range(1..=4) {
            1 => Rotacion::Deg0,
            2 => Rotacion::Deg90,
            3 => Rotacion::Deg180,
            4 => Rotacion::Deg270,
            _ => panic!("Algo salio mal"),
        };

        let mut pieza = Pieza::new(id, x, y, rotacion);

        for _ in 0..=3 {
            if tablero.es_valido(&pieza) {
                tablero.colocar_pieza(&pieza, &mut piezas);
                id += 1;
                break;
            }
            pieza.rotar();
        }
        intento += 1;
    }
    Solucion {
        matriz: tablero,
        piezas,
    }
}

fn celdas_cubiertas(solucion: &Solucion) -> i32 {
    let tablero = &solucion.tablero;
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

fn generar_vecinos(solucion: &Solucion) -> Vec<Solucion> {
    let mut vecindario: Vec<Solucion> = Vec::new();
    let mut intentos = 0;
    let mut rng = rand::rng();
    while intentos < MAX_INTENTOS {
        // TODO: Eliminar la pieza del tablero antes de hacer pruebas
        let solucion_temp = solucion.clone();
        // Rotar
        let indice: i32 = rng.random_range(0..solucion.piezas.len());
        let mut pieza = solucion_temp.piezas[indice].clone();

        let mut copia_pieza = pieza.clone();

        for _ in 0..=3 {
            if tablero.es_valido(&copia_pieza) {
                // Generar nuevo vecino y agregar el vecindario
                // TODO
                tablero.colocar_pieza(&copia_pieza, &mut vecindario);
                continue;
            }
            pieza.rotar();
        }

        // Mover
        // Verificar que la pieza no este en el tablero
        copia_pieza = pieza.clone();
        // LIsta  de posiciones disponibles
        let posiciones_disponibles = buscar_posiciones_disponibles();
        for (x, y) in posiciones_disponibles {
            // Probar cada piza en el tablero
            copia_pieza.mover(x, y);
            if tablero.es_valido(&copia_pieza) {
                // Generar nuevo vecino y agregar el vecindario
                // TODO
                tablero.colocar_pieza(&copia_pieza, &mut vecindario);
                continue;
            }
        }

        // Agregar
        // buscar en las posiciones disponibles, escoger al azar y donde entren
    }
}

pub fn cubrir(caso: Caso, iteraciones: usize) {
    // Solucion inicial aleatoria
    let solucion = solucion_inicial_aleatoria(caso.tablero, caso.filas, caso.columnas);
    let solucion_mejor = solucion.clone();
    solucion.display();
    for i in 1..=iteraciones {
        let mut vecino_mejor = solucion_mejor.clone();
        // Generar vecinos de S
        let mut vecinos = generar_vecinos(&solucion_mejor);
        for vecino in vecinos {
            if celdas_cubiertas(&vecino) > celdas_cubiertas(&vecino_mejor) {
                vecino_mejor = vecino;
            }
        }

        // Si se encuentra un vecino mejor que el de hasta ahora se actualiza
        if celdas_cubiertas(&vecino_mejor) > celdas_cubiertas(&solucion) {
            solucion = vecino_mejor;
            // Si el mejor vecino es mejor que la solcion actual se avanca hacia el
            if celdas_cubiertas(&solucion) > celdas_cubiertas(&solucion_mejor) {
                solucion_mejor = solucion;
            }
        } else {
            // Terminar busqueda
            break;
        }
    }
}
