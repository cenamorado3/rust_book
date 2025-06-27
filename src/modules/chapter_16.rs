//skipping ahead

pub mod concurrency{
    use std::fmt::Display;
    use std::iter::Iterator;
    use std::thread;

    pub trait Concurrent<T>{
        fn process<C>(&self, t: C, f: fn(T) -> T) -> Vec<T>
        where 
        C : Iterator<Item = T>,
        T: Display + Send + 'static;
    }

    // enum ConcurrentCollection{
    //     ConcurrentVector
    // }


    pub struct ConcurrentVector<T>{
        pub list: Vec<T>
    }
    impl<T> Concurrent<T> for ConcurrentVector<T>{
        fn process<C>(&self, t: C, f: fn(T) ->  T) -> Vec<T>
        where C : Iterator<Item = T>,
        T: Display + Send + 'static{
            let mut updates = vec![];
            for (_, element) in t.enumerate(){
                println!("before: {}", element);
                
                let handle = thread::spawn( move ||{
                    println!("spawn thread for {}", element);
                    f(element)
                });
                
                updates.push(handle.join().unwrap());

            }

            updates
        }
    }
}

//decided i learned what i wanted so i wont spend more time on this
//          "Right or wrong, I know I just need closure"