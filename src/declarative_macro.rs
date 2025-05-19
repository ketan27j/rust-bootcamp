macro_rules! say_hello {
    () => {
        println!("Hello world");
    };
}

macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("Hello from {}", stringify!($func_name));
        }
    };
}

create_function!(hello);

fn main(){
    say_hello!();
    hello();
}