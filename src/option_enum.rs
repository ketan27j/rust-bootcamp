fn find_first_a(s: String) -> Option<i32> {
    for(index,char) in s.chars().enumerate(){
        if char == 'a' {
            return Some(index as i32);
    }}
    return None;
}

fn main(){
    let my_string = String::from("ketan");
    match find_first_a(my_string){
        Some(index)=>println!("letter a found  {} index", index),
        None => println!("Error: a not found"),
    }
}