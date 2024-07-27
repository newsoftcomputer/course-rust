
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn square(side: u32) -> Rectangle {
        Rectangle {
            width: side,
            height: side,
        }
    }
}

fn main() {
   //Multiple bloque de interpretacion
   let rect1 = Rectangle::square(5);
   let area = rect1.area();
   println!("Area: {}", area);
}