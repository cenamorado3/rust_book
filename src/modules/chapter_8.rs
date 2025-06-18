/*
Major collections, all of which are stored on heap

    A vector allows you to store a variable number of values next to each other. -> list like
    
    A string is a collection of characters. We’ve mentioned the String type previously,
    but in this chapter we’ll talk about it in depth. -> *char[] or Vec<char>
    
    A hash map allows you to associate a value with a specific key.
    It’s a particular implementation of the more general data structure called a map. -> dictionary like


    https://doc.rust-lang.org/std/collections/index.html
*/

pub mod vectors{
    pub fn vector(){
        //standard syntax
        //let v: Vec<i32> = Vec::new();
        //with vec! macro which infers the type
        let v = vec![1, 2, 3];
        //standard index appraoch
       // let third: &i32 = &v[2];
        //with get
        let third: Option<&i32> = v.get(2);
        //use in pattern match
        match third {
            Some(third) => println!("The third element is {third}"),
            None => println!("There is no third element."),
        }

        //ownership rules are important when modfiying a collections 
        //https://doc.rust-lang.org/nomicon/intro.html
    }

    //keep mod scope in mind
    // pub enum SpreadsheetCell {
    //     Int(i32),
    //     Float(f64), 
    //     Text(String),
    // }   

    // fn _multi_type_vec(){
    //     let _row = vec![
    //         SpreadsheetCell::Int(3),
    //         SpreadsheetCell::Text(String::from("blue")),
    //         SpreadsheetCell::Float(10.12),
    //     ];

    //     //let mut row = Vec<SpreadsheetCell> = Vec::new();//this wont work until we get into traits
    // }
    fn _strings(){

    //    let mut s = String::from("lo"); //idempotent with let mut s = "lo".to_string();
    //     s.push('l');//pushes chars only as it is a vector
    //     s.push_str("push a range of chars");

    //     println!("{s}");

        // let s1 = String::from("a");
        // let s2 = String::from(" - b");
        // let s3 = s1 + &s2; // s1 has been moved here and can no longer be used
        // println!("{s3}");

        //would again recomment this link for complicated string stuff, because of encoding for non-english characters
        //https://doc.rust-lang.org/book/ch08-02-strings.html

        //the must knows are, you can not concat multiple strings and format! should be used instread,
        //this is due to ownership rules -> deref coersion  

    }

    pub fn hash(){
        use std::collections::HashMap;

        //init
        let mut scores = HashMap::new();

        //adding
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);//ownership rules important here as well, non-copyable traits/types are consumed

        //accessing
        let team_name = String::from("Blue");
        let score = scores.get(&team_name).copied().unwrap_or(0);//note ref

        //iter
        for (key, value) in &scores {
            println!("{key}: {value}");
        }

        //update
        scores.insert(String::from("Blue"), 25);
        //add missing key
        scores.entry(String::from("Yellow")).or_insert(50);//entry vs insert



        println!("{score}");
        
        //updating based on previous values is a bit more complicated
        let mut map = HashMap::new();
        for word in "hello world wonderful world".split_whitespace() {
            let count = map.entry(word).or_insert(0);//or_insert returns a mutable reference
            *count += 1;//value must be deferenced 
            //reference is dropped after current iter, important as ownership rule would be violated by count
            println!("{map:?}");
        }
    }
}





