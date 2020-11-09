use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct Mapa {
    coordenadas: Vec<Vec<f32>>,
    distancias: Vec<Vec<f32>>,
    max_x: f32,
    max_y: f32,
    min_x: f32,
    min_y: f32,
    dimension: usize,
}

impl Mapa {
    pub fn new(dimension: usize) -> Mapa {
        let coordenadas = Vec::with_capacity(dimension);
        let distancias = Vec::with_capacity(dimension);
        Mapa { coordenadas, distancias, dimension, max_x: 0.0, max_y: 0.0, min_x: 0.0, min_y: 0.0 }
    }

    fn calcula_distancia(&self, x_1: f32, y_1: f32, x_2: f32, y_2: f32) -> f32 {
        let dx = x_2 - x_1;
        let dy = y_2 - y_1;
        let r2 = dx * dx + dy * dy;
        r2.sqrt()
    }

    pub fn cargar_datos(&mut self, archivo: &str) {
        // Cargamos el archivo
        let mut m_archivo: String = "src/data/".to_owned();
        m_archivo.push_str(archivo);
        m_archivo.push_str(".tsp");

        let file = File::open(&m_archivo).unwrap();
        let reader = BufReader::new(file);

        // leemos los datos linea por linea
        let mut access: bool = false;
        for (_, linea) in reader.lines().enumerate() {
            let linea = linea.unwrap(); // Ignore errors.
            if linea.contains("TOUR_SECTION") || linea.contains("EOF") {
                break;
            }
            if linea.contains("NODE_COORD_SECTION") {
                access = true;
                continue;
            }
            if access {
                let campos: Vec<&str> = linea.split(" ").collect();
                self.coordenadas.push(vec![
                    campos[1].parse::<f32>().unwrap(),
                    campos[2].parse::<f32>().unwrap()
                ]);
                self.distancias.push(vec![0.0; self.dimension]);
            }
        }

        // calculamos la matriz de distancias
        for ciudad_i in 0..self.dimension {
            for ciudad_j in 0..self.dimension {
                self.distancias[ciudad_i][ciudad_j] = self.calcula_distancia(
                    self.coordenadas[ciudad_i][0], self.coordenadas[ciudad_i][1],
                    self.coordenadas[ciudad_j][0], self.coordenadas[ciudad_j][1],
                );
            }
        }

        // calculamos las bandas del mapa para dibujarlo centrado
        self.min_x = self.coordenadas[0][0];
        self.min_y = self.coordenadas[0][1];
        for i in 0..self.dimension {
            if self.coordenadas[i][0] > self.max_x {
                self.max_x = self.coordenadas[i][0];
            }
            if self.coordenadas[i][1] > self.max_y {
                self.max_y = self.coordenadas[i][1];
            }
            if self.coordenadas[i][0] < self.min_x {
                self.min_x = self.coordenadas[i][0];
            }
            if self.coordenadas[i][1] < self.min_y {
                self.min_y = self.coordenadas[i][1];
            }
        }
    }

    pub fn get_distancia(&self, x: usize, y: usize) -> f32 {
        self.distancias[x][y]
    }

    pub fn get_coordenas(&self, ciudad_i: usize) -> (f32, f32) {
        (self.coordenadas[ciudad_i][0], self.coordenadas[ciudad_i][1])
    }

    pub fn get_max_x(&self) -> f32 {
        self.max_x
    }

    pub fn get_max_y(&self) -> f32 {
        self.max_y
    }

    pub fn get_min_x(&self) -> f32 {
        self.min_x
    }

    pub fn get_min_y(&self) -> f32 {
        self.min_y
    }
}