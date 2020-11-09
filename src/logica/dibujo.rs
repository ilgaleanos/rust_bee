use plotters::prelude::*;

use crate::logica::abeja::Abeja;
use crate::logica::mapa::Mapa;

pub fn dibujar(abeja: &Abeja, mapa: &Mapa, archivo: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut m_archivo: String = "src/data/".to_owned();
    m_archivo.push_str(archivo);
    m_archivo.push_str(".png");


    let root = BitMapBackend::new(&m_archivo, (1920, 1080)).into_drawing_area();
    let _ = root.fill(&WHITE);
    let root = root.margin(10, 10, 10, 10);


    let mut chart = ChartBuilder::on(&root)
        .build_cartesian_2d(
            mapa.get_min_x()..mapa.get_max_x(),
            mapa.get_min_y()..mapa.get_max_y())?;


    chart
        .configure_mesh()
        .y_label_formatter(&|x| format!("{:.3}", x))
        .draw()?;


    let mut rutas = Vec::with_capacity(abeja.get_dimension() + 1);
    for ciudad in 0..abeja.get_dimension() {
        rutas.push(mapa.get_coordenas(abeja.get_gen(ciudad)));
    }
    rutas.push(mapa.get_coordenas(abeja.get_gen(0)));


    chart.draw_series(LineSeries::new(rutas, &RED))?;


    Ok(())
}