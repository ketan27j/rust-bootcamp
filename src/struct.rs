#[derive(Clone)]
struct User {
    sign_in_count: u64,
    active: bool,
    username: String,
}

fn main() {
    let user1 = User {
        active: true,
        sign_in_count: 1,
        username: String::from("someusername123"),
    };
    print_name(user1.clone());
    print!("{}{}", user1.active, user1.sign_in_count);
}

// fn print_name(user1: User) {
//     print!("{}", user1.active);
// }

impl User {
    fn print_name(&self) {
        print!("{}", self.username);
    }
}