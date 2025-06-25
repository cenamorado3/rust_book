use crate::modules::chapter_16::concurrency::*;
pub mod modules;

fn main() {

    println!("=============");
    println!("==== int ====");
    println!("=============");

    let x = ConcurrentVector::<i32>{
        list: vec![1,2,3]
    };



    x.process(&mut x.list.iter().copied(), |e|{
        e + 1
    });

    println!("=============");
    println!("== string ==");
    println!("=============");
    let x = ConcurrentVector::<String>{
        list: vec!["a".to_string(), "b".to_string(), "c".to_string()]
    };

    x.process(&mut x.list.iter().cloned(), | e|{
       format!("{} {}", e, "cat")
    });



}
