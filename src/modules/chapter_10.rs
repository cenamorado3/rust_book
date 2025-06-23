//generics can be complicated if you have not been exposed to them, so no notes from me, experience is the best teacher
//when coupled with traits a lot of the usual generic usage comes to life ie
/*
    fn generic_fun<T: trait + trait2>(t: &T)
or
    fn generic_fun<T>(t: &T)
    where
    T: trait + trait2
    trait  

in C# this seems a bit easier to digest as some object would have to inherit trait and trait2, which would be the generic type given
with Rust's "compositional" approach, we need to define the traits and any type which implements these traits can be used
*/


pub mod traits{
use std::fmt::{Display, Formatter, Error};
    //interface like
    pub trait AreaTrait{
        //default impl - note to pub
        fn some_method(&self) -> f32{
            //panic!("Not implemented!");
            unimplemented!();// ::<>
        }


    }


    pub struct SomeStruct{
        pub x: f32,
        pub y: f32
    }

    impl AreaTrait for SomeStruct{
        //note no pub
        fn some_method(&self) -> f32{
            return &self.x * &self.y;//old habits die hard
        }
    }

    //rather than reinventing the wheel, i think i can just impl for Display
    // pub trait TypeName{
    //     fn print_type(t: &(impl AreaTrait + TypeName + ToString)){
    //         println!("{}", &t.to_string());
    //     }
    //}
    impl Display for SomeStruct{
        //i have no idea what this means and have gone to deep and must save myself
        //https://doc.rust-lang.org/std/fmt/trait.Display.html
        fn fmt(&self, f: &mut Formatter<'_>)-> Result<(), Error>{
            return write!(f, "Display trait implemntned to_string for type: SomeStruct", );
        }
    }


    pub struct UndefinedBehavior{}
    impl AreaTrait for UndefinedBehavior{
        //unlike interfaces, implementation is not enforced for at compile time 

    }

    impl Display for UndefinedBehavior{
        //read the docs!
        fn fmt(&self, f: &mut Formatter<'_>)-> Result<(), Error>{
            return write!(f, "Display trait implemented for type: UndefinedBehavior", );
        }
    }

}