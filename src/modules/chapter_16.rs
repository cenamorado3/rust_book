//skipping ahead

pub mod concurrency{
    use std::fmt::Display;
    use std::iter::Iterator;
    //use std::thread;
    //spitballing a bit so im actually working out my brain and working toward something
    pub trait Concurrent{
        //thinking like taking an action in C#, where a delegate can be defined to do some work on an element conncurently
        //the fun part will be thinking about the scope of memory, going to try if it can be inferred first
        fn process<T, E>(&self, t: T, f: fn(E) -> E) 
        where T : Iterator<Item = E>,
        E : Display + Copy;

        fn g(&self);
    }

    // enum ConcurrentCollection{
    //     ConcurrentVector
    // }
    pub struct ConcurrentVector<T>{
        pub list: Vec<T>
    }
    impl<T> Concurrent for ConcurrentVector<T>{
        fn process<L, E>(&self, t: L, f: fn( E) -> E )
        where L : Iterator<Item = E>,
        E : Display + Copy{
            for element in t{
                //i guess rust generics arent supported so i cant return of type E, only exactly E?
                //or im in to deep again
                println!("-->{}", f( element));
                // thread::spawn(||{
                    //eventually id like to execute each on its own thread just because 
                // });
            }
        }

        fn g(&self){println!("asdf");}
    }
}

//this is absolutely overcomplicated. i dont think its wrong for the sake knowledge