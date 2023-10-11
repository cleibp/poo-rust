struct Retangulo {
    base: f64,
    altura: f64,
}

struct Circulo {
    raio: f64,
}

trait Forma {
    fn calcular_area(&self) -> f64;
}

impl Forma for Retangulo {
    fn calcular_area(&self) -> f64 {
        self.base * self.altura
    }
}

impl Forma for Circulo {
    fn calcular_area(&self) -> f64 {
        std::f64::consts::PI * self.raio * self.raio
    }
}

fn main() {
    // Criar uma forma de retângulo
    let retangulo = Retangulo {
        base: 5.0,
        altura: 4.0,
    };

    // Criar uma forma de círculo
    let circulo = Circulo { raio: 3.0 };

    // Calcular e imprimir a área do retângulo
    println!("Área do Retângulo: {}", retangulo.calcular_area());

    // Calcular e imprimir a área do círculo
    println!("Área do Círculo: {}", circulo.calcular_area());
}
