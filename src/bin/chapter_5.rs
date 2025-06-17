
//structs
#[derive(Debug)]//see lines 58
struct Example {
    attribute1: bool,
    attribute2: String,
    attribute3: u8,
}

// warnings bother me 
//rust does not have classes
// impl Example{
//     ///Methods can take ownership of self, borrow self immutably, or borrow self mutably, just as they can any other parameter.
//     fn ex(&self) -> i8{
//         return 0;//or just 0 to omit return keywork, no semicolon
//     }
//     fn mut_ex(&mut self) -> i8{
//         return 0;
//     }
//     fn own_ex(self) -> i8{
//         /*
//         Having a method that takes ownership of the instance by using just self as the first parameter is rare; 
//         this technique is usually used when the method transforms self into something else and you want to prevent the caller 
//         from using the original instance after the transformation.
//         */
//         return 0;
//     }
    
// }

//tuple struct
// #[derive(Debug)]
// struct TupleStruct(u8, u8, u8);

//unit like
//struct Always;//this will be important once we get to trait
fn main(){
    
    //immutable
    let struct_example = Example{
        attribute1: false,
        attribute2: String::from("asdf"), //a reference type can be used in place of an owned type...which required a lifetime, which ill learn about later
        attribute3: 0,
    };

    //dot notation for accessing values
    struct_example.attribute1;


    //mutable
    let mut struct_mut_example = Example{
        attribute1: false,
        attribute2: String::from("asdf"),
        attribute3: 0,
    };
    struct_mut_example.attribute1 = true;

    let _builder = struct_builder(String::from("builder"));


    //rather than updating each field in a struct such as

    let example2 = Example{
        attribute1: struct_example.attribute1,
        attribute2: struct_example.attribute2,
        attribute3: 2,
    };
    //update syntax can be used

// it is important to keep in mind Rust's "shadowing" and copy trait(s)

    //update struct syntax
    let update_syntax = Example{
        attribute3: 2,
        ..example2//must come last
        //it is important to keep in mind Rust's "shadowing" and copy trait(s) as the original "struct_exmaple" can not be used
        //due to the values being moved

    };
    println!("{0}", update_syntax.attribute3);
    println!("{update_syntax:?}");
    println!("{update_syntax:#?}");


    //let c = TupleStruct(0,1,2);
    //println!("{c:#?}");
    //let c = dbg!(c);//dbg! macro takes and returns ownership, here we shadow it for educational purposes, "C" would other be dropped
    //dbg!(&c);//by passing a reference instead, ownership is not taken and C persist
}

//when function's parameters/arguments match a struct's field name(s) "field init" syntax can be used
fn struct_builder(attribute2: String) -> Example{
    return Example{
        attribute1: true,
        attribute2, //from argument,
        attribute3: 0,
    };
}

