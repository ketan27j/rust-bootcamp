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

macro_rules! generate_functions {
    ($($func_name:ident),*) => {
        $(
            fn $func_name() {
                println!("Hello from {}", stringify!($func_name));
            }
        )*
    };
}

create_function!(hello);
generate_functions!(foo, bar, baz);
fn main(){
    say_hello!();
    hello();
    foo();
    bar();
    baz();
}