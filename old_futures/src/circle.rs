#[derive(Debug)]
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

trait HasArea {
    fn area(&self) -> f64;

//    fn is_larger(&self, &Self) -> bool;

    fn increase_radius(self: &mut Self, increase_by: f64);
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }

//    fn is_larger(&self, other: &Self) -> bool {
//        self.area() > other.area()
//    }

    // "&mut self" is syntactic sugar for "self: &mut Self"
    fn increase_radius(&mut self, increase_by: f64) {
        self.radius += increase_by
    }
}


pub fn play_with_traits()
{
    let mut c = Circle { x: 0.2, y: 0.3, radius: 2.0 };
    println!("Circle c = {:?}", c);
    c.increase_radius(10.5);
    println!("Circle c = {:?}", c);
}