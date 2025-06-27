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
    println!("{body}");
    //dbg!(x);
}
