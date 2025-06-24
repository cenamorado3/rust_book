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
    //println!("{}", x.some_method());
    println!("{}", x.to_string());


    let x = LifetimeExample{};
    println!("{}", x.longest_with_an_announcement("asdf", "longer than asdf", "blah"));//everything on this line gets dropped...except x
    dbg!(x);//now everything is dropped, x was consumed
    //x.longest_with_an_announcement("asdf", "longer than asdf", "blah");//try this
}
