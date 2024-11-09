// This is a simple macro named `say_hello`.
//create macro with the argument 
macro_rules! say_hello {
    // `()` indicates that the macro takes no argument.
    //add argument to macro
    ($name: expr) => {
        println!("Hello, {}!", $name)
    };
}

fn main() {
    // This call will expand into `println!("Hello!")`
    say_hello!("rust")
}
//rustc lecture01.rs
