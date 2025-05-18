fn main() {
    let mut a: u32 = 10;
    let b: u32 = 2;
    let sum: u32 = sum(&a, &b);
    println!("{}",sum);
    a = 20;
    println!("{}{}",a,b);
    let boo:&str  = return_boolean();
    println!("{}",boo);
    let str:&str  = &return_string();
    println!("{}",str);
    let arr:[i32; 5]  = return_array();
    println!("{:?}",arr);
    let ans: Vec<i32>= return_vector();
    println!("{:?}", ans);
    let fname = String::from("ketan juikar");
    println!("{}",get_first_name(fname))
}

fn return_vector() -> Vec<i32> {
    let a: Vec<i32> = vec![1, 2, 3];
    return a;
}

fn return_array() -> [i32; 5] {
    return [1, 2, 3, 4, 5];
}

fn sum(a: &u32, b: &u32) -> u32 {
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

pub fn get_first_name(str:String)-> String{
    let mut first_name=String::from("");
    for c in str.chars(){
        if c==' '{
            break
        }
        first_name.push(c);
    }
    return first_name;
}