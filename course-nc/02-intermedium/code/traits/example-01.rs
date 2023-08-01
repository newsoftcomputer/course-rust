
struct Circulo {
    x: f64,
    y: f64,
    radio: f64,
}

trait Area {
    fn area($self) -> f64;
}

impl Area for Circulo {
    fn area(&self) -> f64 { self.radio }
} 

fn main() {

}