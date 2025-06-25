use crate::modules::chapter_16::concurrency::*;
pub mod modules;

fn main() {

    println!("=============");
    println!("==== int ====");
    println!("=============");

    let x = ConcurrentVector::<i32>{//adding explit types helped?
        list: vec![1,2,3]
    };



    x.process(&mut x.list.iter().copied(), |&mut e|{//copied
        e + 1//delegated operation
    });

    println!("=============");
    println!("== string ==");
    println!("=============");
    let x = ConcurrentVector::<String>{//adding explit types helped?
        //build vec! with String not str/&str (still a bit confused on the diff)
        list: vec!["a".to_string(), "b".to_string(), "c".to_string()]
    };

    x.process(&mut x.list.iter().cloned(), | e|{//cloned
       format!("{} {}", e, "cat")
    });
}
