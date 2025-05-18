use std::fmt::Display;

pub fn main() {
    let v1 =vec![1,2,3];
    let v2= vec![String::from("Ketan"), String::from("Juikar")];
    let v3=vec![1.0,2.0,3.0];
    println!("{}",first_element(v1.clone()).unwrap());
    println!("{}",first_element(v2).unwrap());
    println!("{}",first_element(v3).unwrap());
    println!("{}",does_exist(v1, 1));
    display_var(biggest_element(1,2));
}

fn first_element<T>(v: Vec<T>) -> Option<T> {
    return v.into_iter().nth(0)
}

fn does_exist<T: std::cmp::PartialEq>(v: Vec<T>, element: T)-> bool {
    let mut iter = v.iter();
    while let Some(value) = iter.next() {
        if *value == element {
            return true;
        }
    }
    return false;
}

fn biggest_element<T: Ord>(a:T,b:T)-> T {
    if a > b {
        return a;
    }
    return b;
}

fn display_var<T:Display>(a:T){
     println!("{}",a);
}