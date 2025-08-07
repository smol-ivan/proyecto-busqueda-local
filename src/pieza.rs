#[derive(Clone, Debug)]
pub enum Rotacion {
    Deg0,
    Deg90,
    Deg180,
    Deg270,
}

#[derive(Clone, Debug)]
pub struct Pieza {
    pub id: i8,
    x: i32,
    y: i32,
    rotacion: Rotacion,
}

impl Pieza {
    pub fn new(id: i8, x: i32, y: i32, rotacion: Rotacion) -> Pieza {
        Pieza { id, x, y, rotacion }
    }

    pub fn mover(&mut self, dx: i32, dy: i32) {
        self.x = dx;
        self.y = dy;
    }

    pub fn rotar(&mut self) {
        self.rotacion = match self.rotacion {
            Rotacion::Deg0 => Rotacion::Deg90,
            Rotacion::Deg90 => Rotacion::Deg180,
            Rotacion::Deg180 => Rotacion::Deg270,
            Rotacion::Deg270 => Rotacion::Deg0,
        }
    }

    // Devolver las coordenadas absolutas de los bloques que ocupa
    pub fn bloques_ocupados(&self) -> Vec<(i32, i32)> {
        match self.rotacion {
            Rotacion::Deg0 => vec![
                (self.x, self.y),
                (self.x, self.y - 1),
                (self.x, self.y - 2),
                (self.x + 1, self.y),
            ],

            Rotacion::Deg90 => vec![
                (self.x, self.y),
                (self.x + 1, self.y),
                (self.x + 2, self.y),
                (self.x, self.y + 1),
            ],

            Rotacion::Deg180 => vec![
                (self.x, self.y),
                (self.x, self.y + 1),
                (self.x, self.y + 2),
                (self.x - 1, self.y),
            ],

            Rotacion::Deg270 => vec![
                (self.x, self.y),
                (self.x - 1, self.y),
                (self.x - 2, self.y),
                (self.x, self.y - 1),
            ],
        }
    }
}
