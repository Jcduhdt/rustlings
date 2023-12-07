// macros2.rs
//
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a
// hint.

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

// main函数使用宏的时候，必须先编译宏。
fn main() {
    my_macro!();
}

