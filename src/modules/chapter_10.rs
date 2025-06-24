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



/*
Lifetyime syntax
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime


---
 fn fmt(&self, f: &mut Formatter<'_>)-> Result<(), Error>{

 so that thing i did not understand has a mutable reference set with a lifetime explictly set which is never used?

---

 'static is a lifetime that last the length of the program

---

Lifetime Elision
rust history -> newer versions, including rustc 1.87.0 (17067e9ac 2025-05-09) can infer these lifetimes so it doesnt have to be dealt with
as frequently


---

i understand the theory of traits, hehe, (ToTs), thanks to my use of generics in C# and to some degree the lifetimes since they are generics

*/#[derive(Debug)]
    pub struct LifetimeExample{
    }
    //lots going on here
    impl LifetimeExample{
        pub fn longest_with_an_announcement<'a, T>(
            &self,
            //below argument x references/borrows lifetime 'a from function
            x: &'a str, //str not String, get lots of errors if you just change it to String, str is shorthand for &str?
            //y shares lifetime a, so lifetime of function?
            y: &'a str,
            ann: T,//the third arg
        ) -> &'a str //returns a str that also last the lifetime of 'a, ie the length of the function
        where
            T: Display,//other ways to do this for sure, this is the only reason line 115 works, otherwise ("{}", ann) syntax
        {
            println!("Announcement! {ann}");
            if x.len() > y.len() { x } else { y }//returns str x or y, whicher is longest
        }

        /*
        still don't fully grasp the concept of lifetimes, my understanding is to enforce a desired ownership model/ensure a borrowed variable
        lives some duration, i think the bold statement here is, its a parallel to design patterns but on memory rather than objects

        by declaring a lifetime you can effectively prevent compilation and given the low level nature of rust, i can see why that could
        be useful/desired, particularly with traits and by virtue anything that implements that trait

        ive defintely used generics to enforce type safety and i can see a lifetime enforcing memory safety.

        complicated yes, worth it, depends, regardless i think its probably a good sign of a well thought out system or a good
        rustaceans
        */
    }
}