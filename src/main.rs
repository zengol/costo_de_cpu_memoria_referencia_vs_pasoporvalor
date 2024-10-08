use std::time::Instant;

trait Rectangular {
    fn ancho(&self) -> f64;
    fn alto(&self) -> f64;
}

#[derive(Clone)]
struct PequenoRectangulo {
    ancho: f64,
    alto: f64,
}

impl Rectangular for PequenoRectangulo {
    fn ancho(&self) -> f64 { self.ancho }
    fn alto(&self) -> f64 { self.alto }
}

#[derive(Clone)]
struct GranRectangulo {
    ancho: f64,
    alto: f64,
    datos: [u8; 784],
}

impl Rectangular for GranRectangulo {
    fn ancho(&self) -> f64 { self.ancho }
    fn alto(&self) -> f64 { self.alto }
}

fn area_por_valor<T: Rectangular>(rect: T) -> f64 {
    rect.ancho() * rect.alto()
}

fn area_por_referencia<T: Rectangular>(rect: &T) -> f64 {
    rect.ancho() * rect.alto()
}

fn main() {
    let pequeno = PequenoRectangulo { ancho: 5.0, alto: 3.0 };
    let grande = GranRectangulo { ancho: 5.0, alto: 3.0, datos: [0; 784] };

    let inicio = Instant::now();
    for _ in 0..1_000_000 {
        area_por_valor(pequeno.clone());
    }
    let duracion_pequeno_valor = inicio.elapsed();

    let inicio = Instant::now();
    for _ in 0..1_000_000 {
        area_por_referencia(&pequeno);
    }
    let duracion_pequeno_ref = inicio.elapsed();

    let inicio = Instant::now();
    for _ in 0..1_000_000 {
        area_por_valor(grande.clone());
    }
    let duracion_grande_valor = inicio.elapsed();

    let inicio = Instant::now();
    for _ in 0..1_000_000 {
        area_por_referencia(&grande);
    }
    let duracion_grande_ref = inicio.elapsed();

    println!("Pequeño por valor: {:?}", duracion_pequeno_valor);
    println!("Pequeño por referencia: {:?}", duracion_pequeno_ref);
    println!("Grande por valor: {:?}", duracion_grande_valor);
    println!("Grande por referencia: {:?}", duracion_grande_ref);
}
