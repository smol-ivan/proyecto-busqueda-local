type Matriz: Vec<Vec<i8>>;
// 0 Si la casilla es libre 1 si esta ocupada

enum Rotacion {
    Deg_0,
    Deg_90,
    Deg_180,
    Deg_270,
}

#[derive[Clone]]
struct Pieza {
    x: i32,
    y: i32,
    rotacion: Rotacion,
}

impl Pieza {
    fn new(x: i32, y: i32, rotacion: Rotacion) -> Pieza {
        Pieza { x, y, Rotacion }
    }

    fn mover(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    fn rotar(&mut self) {
        self.rotacion = match self.rotacion {
            Rotacion::Deg_0 => Rotacion::Deg_90,
            Rotacion::Deg_90 => Rotacion::Deg_180,
            Rotacion::Deg_180 => Rotacion::Deg_270,
            Rotacion::Deg_270 => Rotacion::Deg_0,
        }
    }

    // Devolver las coordenadas absolutas de los bloques que ocupa
    fn bloques_ocupados(&self) -> Vec<(i32, i32)> {
        match self.rotacion {
            Rotacion::Deg_90 => vec![
                (self.x, self.y),
                (self.x, self.y + 1),
                (self.x, self.y + 2),
                (self.x + 1, self.y),
            ],

            Rotacion::Deg_0 => vec![
                (self.x, self.y),
                (self.x + 1, self.y),
                (self.x + 2, self.y),
                (self.x, self.y - 1),
            ],

            Rotacion::Deg_270 => vec![
                (self.x, self.y),
                (self.x - 1, self.y),
                (self.x, self.y - 1),
                (self.x, self.y - 2),
            ],

            Rotacion::Deg_180 => vec![
                (self.x, self.y),
                (self.x - 2, self.y),
                (self.x - 1, self.y),
                (self.x, self.y + 1),
            ],
        }
    }
}

struct Tablero {
    matriz: Matriz,
    filas: i32,
    columnas: i32,
}

impl Tablero {
    fn casillas_vacias(&self) -> i32 {
        let mut contador = 0;
        for x in 1..=self.columnas {
            for y in 1..=self.filas {
                if self.matriz[x as usize][y as usize] == 0 {
                    contador += 1;
                }
            }
        }
        contador
    }
    fn es_valido(&self, pieza: &Pieza) -> bool {
        for (x, y) in pieza.bloques_ocupados() {
            // Verificar si esta dentro del tablero y las casillas libres es un multiplo de 4
            if x < 0 || y < 0 || x > self.columnas || y > self.filas || self.casillas_vacias() > 4 {
                return false;
            }

            if self.matriz[x as usize][y as usize] == 1 {
                return false;
            }
        }
        true
    }

    fn colocar_pieza(&mut self, pieza: &Pieza, piezas: &mut Vec<Pieza>) {
        for (x, y) in pieza.bloques_ocupados() {
            self.matriz[x as usize][y as usize] == 1;
        }
        piezas.push(pieza.clone())
    }
}

struct Solucion {
    matriz: Tablero,
    piezas: Vec<Pieza>,
}

fn solucion_inicial_aleatoria(matriz: Matriz, filas: i32, columnas: i32) -> Solucion {
    let tablero = Tablero(matriz, filas, columnas);
    let mut intento = 0;
    let mut piezas: Vec<Pieza> = Vec::new();

    loop {
        if intento > 5 {
            break;
        }
        let x: i32 = rand.random_range(0..columnas);
        let y: i32 = rand.random_range(0..filas);

        let rotacion: Rotacion = match random_range(1..=4) {
            1 => Rotacion::Deg_0,
            2 => Rotacion::Deg_90,
            3 => Rotacion::Deg_180,
            4 => Rotacion::Deg_270,
        };

        let pieza = Pieza { x, y, rotacion };

        for i in 0..=3 {
            if tablero.es_valido(&pieza) {
                tablero.colocar_pieza(&pieza, &piezas);
                break;
            }
            pieza.rotar();
        }
        intento += 1;
    }
    Solucion { tablero }
}

fn cubrir(tablero: Matriz, iteraciones: usize, vecindario: Vec<Solucion>) {}
