use crate::modules::chapter_16::concurrency::*;
pub mod modules;

fn main() {
    let x = ConcurrentVector{
        list: vec![1,2,3]
    };



    //AAAAAHHHHHHHH
    x.process(&mut x.list.iter(), | e|{
        e 
    });


    let x = ConcurrentVector{
        list: vec!["a", "b", "c"]
    };

    x.process(&mut x.list.iter(), |e|{
       println!("{} {}", e, String::from("cat"));
       //format!("{}{}", e, "cat").as_str()
       e

       //leaving this for now, im not sure if this is the right path, its all i got, thanks compiler
       //https://doc.rust-lang.org/nomicon/hrtb.html

       //if i return some operation compiler complains about &E vs E or &&E
       //it seems to either want a hrtb or something small and silly for me to correctly resolve the types
       //clearly i dont have a solid grasp on this quite yet, wasnt exepcting to
    });
}
