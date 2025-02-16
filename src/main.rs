fn main() {
    let sum: u32 = sum(1,2);
    println!("{}",sum);
    let boo:&str  = return_boolean();
    println!("{}",boo);
    let str:&str  = &return_string();
    println!("{}",str);
    let arr:[i32; 5]  = return_array();
    println!("{:?}",arr);
    let ans: Vec<i32>= return_vector();
    println!("{:?}", ans);
}

fn return_vector() -> Vec<i32> {
    let a: Vec<i32> = vec![1, 2, 3];
    return a;
}

fn return_array() -> [i32; 5] {
    return [1, 2, 3, 4, 5];
}

fn sum(a: u32, b: u32) -> u32 {
    return a + b;
}

fn return_boolean() -> &'static str {
    let is_male = false;
    if is_male {
        return "male";
    } else {
        return "not male";
    }
}

fn return_string() -> &'static str {
    return "Hello World";
}
