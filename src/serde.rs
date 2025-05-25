use serde::{de::value::Error, Deserialize, Serialize};
use serde_json::{self,Value};

#[derive(Serialize,Deserialize,Debug)]
struct Person{
    name: String,
    age: u32,
}
fn main() {
    let person = Person{
        name: String::from("Ketan"),
        age :40
    };
    let json_str = serde_json::to_string(&person);
    let str1: &String;
    match &json_str {
        Ok(str) => { 
            str1 = str;
            println!("Serialize person {}", str)
        },
        Err(e)=> print!("Error occurred {}",e)
    }

    if let Ok(json_string) = json_str {
        let deserialized_json: Result<Person, serde_json::Error> = serde_json::from_str(&json_string);
        match deserialized_json {
            Ok(person) => println!("Deserialize person {:?}", person),
            Err(e) => print!("Error occured {}", e)
        }
    }
}