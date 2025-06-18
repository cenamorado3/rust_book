// enum Example{
//     E1,
//     E2,
// }

fn main(){
    //creating enum instances
    //let e1 = Example::E1;
    //let e2 = Example::E2;

    let e1 = RustEnum::ExampleWithString(String::from("example"));
    let e2 = RustEnum::ExampleWithInt(2);
    e1.print_value();
    e2.print_value();


    println!("{}", optional(Some(1)));
    println!("{}", optional(None));
    //println!("{}", optional::<String>(Some(String::from("sdf"))));

}



//this isn't typically possible within an enum in other languages
enum RustEnum{
    ExampleWithString(String),
    ExampleWithInt(u8),
}
impl RustEnum{
    fn print_value(&self){
        match &self{
            RustEnum::ExampleWithString (msg) => println!("{}", msg),
            RustEnum::ExampleWithInt (i) => println!("{}", i)
        }
        //enums can also have methods in rust
        //no return ketword, no semicolon, implicit return
    }
}


//keeping this simple for now since I can't easily figure out to how to make the compiler happy with a generic option, 
//fn option<T>(o: option<T>) -> String {type check...match...}




fn optional(o: Option<u8>) -> String{
    match o{
        Some(i) => String::from(format!("{i} as some value")),
        _ => String::from("Has no value"),
    }
}

