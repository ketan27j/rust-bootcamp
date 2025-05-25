struct User<'a> {
    username : &'a str,
    password : &'a str
}

fn main(){
    let str1 = String::from("Ketan");
    let str2 = String::from("Ketan");

    let u = User {username: &str1, password: &str2};
    println!("{} {}",u.username,u.password);

    let v = new_user(&str1); 
    println!("{} {}",v.username,v.password);
}

// Functions returning a struct
fn new_user<'a>(name: &'a str) -> User<'a> {
    User { username: name, password: name }
}
