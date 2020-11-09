use std::cmp;

use rand::Rng;

use crate::logica::dibujo::*;
use crate::logica::mapa::Mapa;

#[derive(Debug)]
pub struct Abeja {
    distancia: f32,
    genes: Vec<usize>,
    dimension: usize,
}

impl Abeja {
    pub fn new(dimension: usize, aleatoria: bool) -> Abeja {
        let mut rng = rand::thread_rng();

        // inicializamos los genes de la abeja
        let mut genes: Vec<usize> = Vec::with_capacity(dimension);
        for index in 0..dimension {
            genes.push(index);
        }

        // hacemos aleatoria su información
        if aleatoria {
            for index in (0..dimension).rev() {
                genes.swap(index, rng.gen_range(0, dimension));
            }
        }

        Abeja { dimension, genes, distancia: 0.0 }
    }


    pub fn calcular_distancia(&self, genes: &Vec<usize>, mapa: &Mapa) -> f32 {
        let mut distancia: f32 = 0.0;
        for ciudad_i in 0..self.dimension - 1 {
            distancia += mapa.get_distancia(
                genes[ciudad_i],
                genes[ciudad_i + 1],
            );
        }
        distancia += mapa.get_distancia(
            genes[self.dimension - 1],
            genes[0],
        );

        distancia
    }


    fn distancia(&self, porcion: &Vec<usize>, mapa: &Mapa) -> f32 {
        let mut distancia: f32 = 0.0;
        for i in 0..porcion.len() - 1 {
            distancia += mapa.get_distancia(porcion[i], porcion[i + 1])
        }
        distancia
    }


    fn obtener_distancia(&self, abeja: &Abeja, ciudad_i: usize, mapa: &Mapa, tam_porcion: usize) -> f32 {
        let posicion = abeja.genes.iter().position(|&ciudad| ciudad == ciudad_i).unwrap();
        // println!("posicion: {}", posicion);

        let mut porcion = Vec::with_capacity(tam_porcion);
        for i in posicion..posicion + tam_porcion {
            porcion.push(abeja.get_gen(i % abeja.dimension))
        }

        return self.distancia(&porcion, mapa);
    }


    fn anticancerigeno(&mut self) {
        // println!("purgando {:?}", self.genes);
        let mut genes_limpios: Vec<usize> = Vec::with_capacity(self.dimension);
        for gen_i in 0..self.dimension {
            if !genes_limpios.contains(&self.genes[gen_i]) {
                genes_limpios.push(self.genes[gen_i]);
            }
        }

        for gen_i in 0..self.dimension {
            if !genes_limpios.contains(&gen_i) {
                genes_limpios.push(gen_i);
            }
        }
        self.genes = genes_limpios;
    }

    pub fn reproducir(&self, otra: &Abeja, mapa: &Mapa, generacion: usize) -> Abeja {
        let mut rng = rand::thread_rng();
        // definimos la porción de información a compartir
        let porcion = cmp::max(2, generacion % (self.dimension / 3));
        let mut hijo = Abeja::new(self.dimension, false);

        for gen_i in 0..self.dimension - porcion {
            let ciudad_i: usize;

            if gen_i == 0 {
                // empezamos en una ciudad aleatoria de la reina si es la primera iteración
                ciudad_i = self.genes[rng.gen_range(0, self.dimension)];
            } else {
                ciudad_i = hijo.get_gen(gen_i)
            }

            // comparamos las porciones
            let distancia_a = self.obtener_distancia(self, ciudad_i, mapa, porcion);
            let distancia_b = self.obtener_distancia(otra, ciudad_i, mapa, porcion);

            // realizamos el intercambio en el hijo
            if distancia_a < distancia_b {
                let posicion = self.genes.iter().position(|&ciudad| ciudad == ciudad_i).unwrap();
                for i in 0..porcion {
                    hijo.set_gen((gen_i + i) % self.dimension, self.genes[(posicion + i) % self.dimension]);
                }
            } else {
                let posicion = otra.genes.iter().position(|&ciudad| ciudad == ciudad_i).unwrap();
                for i in 0..porcion {
                    hijo.set_gen((gen_i + i) % self.dimension, otra.genes[(posicion + i) % self.dimension]);
                }
            }
        }
        // si viene con algún error en la replicación lo corregimos
        hijo.anticancerigeno();

        // calculamos la distancia resultante para el nuevo hijo
        hijo.set_distancia(
            hijo.calcular_distancia(hijo.get_genes(), mapa)
        );

        hijo
    }


    pub fn opt_swap(&self, i: usize, k: usize) -> Vec<usize> {
        let mut nueva_ruta = Vec::with_capacity(self.dimension);
        for i in 0..i {
            nueva_ruta.push(self.genes[i]);
        }
        for i in (i..k).rev() {
            nueva_ruta.push(self.genes[i]);
        }
        for i in k..self.dimension {
            nueva_ruta.push(self.genes[i]);
        }

        nueva_ruta
    }

    pub fn opt2(&mut self, mapa: &Mapa, archivo: &str) {
        for i in 3..self.dimension {
            for k in (i + 1)..self.dimension {
                let new_route = self.opt_swap(i, k);
                let new_distance = self.calcular_distancia(&new_route, mapa);
                if new_distance < self.distancia {
                    self.set_genes(new_route);
                    self.set_distancia(new_distance);
                    let _ = dibujar(self, mapa, archivo);
                }
            }
        }
    }

    pub fn get_gen(&self, indice: usize) -> usize {
        return self.genes[indice];
    }

    pub fn set_gen(&mut self, indice: usize, ciudad_i: usize) {
        self.genes[indice] = ciudad_i;
    }

    pub fn get_genes(&self) -> &Vec<usize> {
        &self.genes
    }

    pub fn set_genes(&mut self, genes: Vec<usize>) {
        self.genes = genes;
    }

    pub fn get_distancia(&self) -> f32 {
        self.distancia
    }

    pub fn set_distancia(&mut self, distancia: f32) {
        self.distancia = distancia;
    }

    pub fn dibujar(&self, mapa: &Mapa, archivo: &str) {
        let _ = dibujar(self, mapa, archivo);
    }

    pub fn get_dimension(&self) -> usize {
        self.dimension
    }
}
