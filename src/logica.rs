use abeja::Abeja;
use mapa::Mapa;

mod abeja;
mod mapa;
mod dibujo;

pub fn ejecutar(archivo: &str, dimension: usize) {
    // Cargamos los datos del mapa
    let mut mapa = Mapa::new(dimension);
    mapa.cargar_datos(archivo);
    println!("{:?}", mapa);

    // Generamos la reina
    let mut reina: Abeja = Abeja::new(dimension, true);
    let distancia_reina = reina.calcular_distancia(reina.get_genes(), &mapa);
    reina.set_distancia(distancia_reina);

    //ejecutaremos
    let bing_bang = 100_000 * dimension;
    let mut generacion = bing_bang;
    while generacion > 0 {
        let mut zangano: Abeja = Abeja::new(dimension, true);
        zangano.set_distancia(
            zangano.calcular_distancia(zangano.get_genes(), &mapa)
        );

        //verificamos que el zángano generado simplemente sea mejor
        if zangano.get_distancia() < reina.get_distancia() {
            reina = zangano;
            generacion = bing_bang;
            println!("{:?}", reina);
            reina.dibujar(&mapa, archivo);
            continue;
        }

        //  generamos un hijo
        let hijo = reina.reproducir(&zangano, &mapa, generacion);
        if hijo.get_distancia() < reina.get_distancia() {
            reina = hijo;
            generacion = bing_bang;
            println!("{:?}", reina);
            reina.dibujar(&mapa, archivo);
        }

        // algoritmo para eliminar posibles cruces cuando estamos finalizando
        if generacion < 3 * dimension {
            reina.opt2(&mapa, archivo);
        }

        generacion -= 1;
    }
    reina.dibujar(&mapa, archivo);

    println!("{:?}", reina);
}