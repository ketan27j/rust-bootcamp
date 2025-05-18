trait Shape {
    fn area(&self) -> f32;
}
#[derive(Clone)]
struct Rect {
    width : f32,
    height : f32,
}

impl Shape for Rect {
    fn area(&self) -> f32 {
        return self.height * self.width;
    }
}

fn get_area(shape: impl Shape) -> f32 {
    return shape.area();
}
fn get_area_2<T:Shape>(shape : T)->f32 {
    return shape.area();
}

fn main(){
    let r = Rect {
        height : 20.0,
        width : 10.0,
    };
    println!("{}",r.area());

    println!("{}",get_area(r.clone()));

    println!("{}",get_area_2(r));

}