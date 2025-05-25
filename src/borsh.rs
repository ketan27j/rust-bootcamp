use borsh::{BorshSerialize, BorshDeserialize};

#[derive(BorshSerialize,BorshDeserialize,Debug,Clone)] 
struct User {
    username: String,
    password: String,
}
fn main(){
    let u = User {
        username : String::from("Ketan"),
        password : String::from("pass")
    };
    let mut v: Vec<u8> = Vec::new();
    let ans = u.serialize(&mut v);
    match ans {
        Ok(_) => println!("{:?}",v),
        Err(e) => println!("Error {}", e)
    };

    let user = User::try_from_slice(&v);
    match user {
        Ok(user1) => println!("{:?}",user1),
        Err(e) => println!("{}",e),
    };
    
}