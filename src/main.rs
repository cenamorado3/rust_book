pub mod modules;

use crate::modules::chapter_17::asyncs::*;
use std::{collections::HashMap};
fn main() {
    let mut x = MyClient::new("www.cataas.com:443".to_string());
    let mut http_header = HttpHeader{
        method: HttpMethods::GET.as_str(),
        http_version: "1.1",
        absolute_uri: "cataas.com",
        uri: "cat?json=true",
        //port: "443",
        headers: HashMap::new()
    };

    http_header.headers.insert("Accept", "image/jpg");
    let body = x.get(http_header);

   /*
        let body = x.async_get(http_header);

        this commit mostly just get some notes and thoughts out there
        a major pain point is the inability to run async from main
        
         scouring through the souce code a few crates including trpl and tokio
        reveals that it at least appears to rely on mspc::channel which is network based

        at a higher level that should be ok and in context, there is some networking involved 
        at a lower level, that is not what I want, at all. To quote Yoda "You must unlearn what you have learned"

        the usage of asyncs, awaits and Task and C# does not truly carry over 1 to 1 because again, C# does support an async run time
        the closest would be using:


        thread::spawn((...move) || {}.... | || {...} when i was toying with the idea what i disliked about it at the time with 
        the inherit need for a return in my use case, at higher level i would not need a return as the time-consuming process would likely 
        take place beyond the boundary

        "Difficult to see, always in motion is the future"
    
    
     */
    println!("{body}");
}
