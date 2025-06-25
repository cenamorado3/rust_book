//skipping ahead

pub mod concurrency{
    use std::fmt::Display;
    use std::iter::Iterator;
    use std::thread;

    pub trait Concurrent<T>{
        fn process<C>(&self, t: C, f: fn(T) -> T) 
        where 
        C : Iterator<Item = T>,
        T: Display + Send + 'static;

        fn g(&self);
    }

    // enum ConcurrentCollection{
    //     ConcurrentVector
    // }


    pub struct ConcurrentVector<T>{
        pub list: Vec<T>
    }
    impl<T> Concurrent<T> for ConcurrentVector<T>{
        fn process<C>(&self, t: C, f: fn(T) ->  T)
        where C : Iterator<Item = T>,
        T: Display + Send + 'static{
            for (_, mut element) in t.enumerate(){
                println!("before: {}", element);
                
                let handle = thread::spawn( move ||{
                    //because of the move happening in spawn, we don't need the closure f to be mutable anymore
                    //since the thread has taken ownership
                    println!("spawn thread for {}", element);
                    f(element)//return closure value
                });
                
                println!("joining threads");
                let r = handle.join().unwrap();
                element = r;


                //these values are not assocated with self...
                //which could be accetpable in some context so ill check in for now as checkpoint
                //i want the under lying list to be updated which is not currently updating
                println!("after: {}", element);
            }
        }

        fn g(&self){println!("asdf");}
    }
}