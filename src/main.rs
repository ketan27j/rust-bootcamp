fn main() {
    // let ans: u32 = sum(1,2);
    let ans: &str = boolean();
    println!("{}", ans);
}

fn sum(a: u32, b: u32) -> u32 {
    return a + b
}

fn boolean()->&'static str {
    let is_male=false;
    if is_male {
        return "male";
    } else {
        return "not male";
    }
}