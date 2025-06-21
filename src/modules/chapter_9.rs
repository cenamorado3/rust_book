/*
Error handling

---

two major types, recoverable and unrecoverable and does not have exceptions

recoverable errors -> Result<T, E>
unrecoverable errors -> panic!();

by default Rust will walk up the stack and clean up memory when it panics, which is called "unwinding" the cargo.toml can be updated with:

=================
profile.release]
panic = 'abort'
================
which instead immediately exits the program leaving memory clean up to the OS


*/

pub mod dream_theather{
    use std::io::{Read, Error};
    use std::fs::{File};

    pub fn panic_attack(){

        let file_result = File::open("lrllrlrrlrllrlrrlrllrlrr.txt");
        match file_result {
            Ok(file) => file,
            Err(_) => panic!("https://www.youtube.com/watch?v=oa7oOdYPOSk"),//unrecoverable
        };
    }


    //lots of ctrl c+v code that can be used
    //https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html


    pub fn systematic_chaos(s: &str) -> Result<String, Error>{
        let mut text = String::new();
        File::open(s)?.read_to_string(&mut text)?;//note the '?' an early return, similar to "lifting" in C# but different
        //if an error occurs, it is returned

        Ok(text)//implicit return of text
    }
}
