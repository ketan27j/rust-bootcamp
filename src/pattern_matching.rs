mod enums;
use crate::Shape;

pub fn cal_area(shape: Shape) -> f32 {
    match shape {
        Shape::Circle(radius) => std::f32::consts::PI * radius as f32 * radius as f32,
        Shape::Square(side) => side as f32 * side as f32,
        Shape::Rectangle(length, breadth) => length as f32 * breadth as f32,
    }
}

/*
mod enums;
mod pattern_matching;
use enums::Shape;
use pattern_matching::cal_area;

fn main() {
    let circle = Shape::Circle(5);
    let rectangle = Shape::Rectangle(5, 10);
    let square = Shape::Square(5);
    let area1 = cal_area(circle);

    print!("circle area{}", area1);
    let area2 = cal_area(rectangle);
    print!("rectangle area{}", area2);
    let area3 = cal_area(square);
    print!("square area{}", area3);
}


*/