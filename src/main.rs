use crate::modules::chapter_10::traits::*;
pub mod modules;

fn main() {
    let x = SomeStruct{
        x: 3.0,
        y: 2.4
    };
    println!("{}", x.some_method().to_string());
    println!("{}", x.to_string());

    let x = UndefinedBehavior{};
    println!("{}", x.some_method());
    println!("{}", x.to_string());
}
