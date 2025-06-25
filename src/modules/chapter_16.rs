//skipping ahead

pub mod concurrency{
    use std::fmt::Display;
    use std::iter::Iterator;
    use std::thread;

    pub trait Concurrent<T>{
        fn process<C>(&self, t: C, f: fn(&mut T) -> T) 
        where 
        //where for<'a> F: Fn(&'a (u8, u16)) -> &'a u8,
        C : Iterator<Item = T>,
        T: Display;

        fn g(&self);
    }

    // enum ConcurrentCollection{
    //     ConcurrentVector
    // }


    pub struct ConcurrentVector<T>{
        pub list: Vec<T>
    }
    impl<T> Concurrent<T> for ConcurrentVector<T>{
        fn process<C>(&self, t: C, f: fn(&mut T) ->  T)
        where C : Iterator<Item = T>,
        T: Display{
            for mut element in t{
                println!("before: {}", element);
                
                element = f(&mut element);//inplace =  should spawn thread? 
                thread::spawn(||{
                    //now make it thread safe
                });

                println!("after: {}", element);

            }
        }

        fn g(&self){println!("asdf");}
    }
}