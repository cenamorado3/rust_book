/*
variables and mutability
let 
an immutable variable ie can not be modified, a readonly variable

vs

let mut
a mutable var

const are always immutable

---

rust allows re-declaring a variable, this is called shadowing, essentially an override

---

data types

scalars - int, float. bool, char


int types =
{
    i8 // 8bit int
    ...
    i128,
    u8, //8 bit unsigned int
    ...
    u128, 

    isize, "arch" sized
    usized //arch = architecture dependant

}


syntax for int literals
decimals = 3_14
hex = 0xff
octals = 0o77
binary = 0b1111_0000
bytes = b'A' (u8 only)

i32 is the default
isize/usize primarily used when indexing collections?


//
exections/errors are called "panic(s)"
where most languages error or crash with integer overflows rust will "panic"


---

floating points 
let x = 2.0
ley y: f32 = 3.14 //IEEE-754


---

numeric operations
+, -, *, /, %

---

bools

let t = true;
let f: bool = false;

---

char

let c = 'c';
let z:char = 'z
let emoji = '{emoji};

---

compund types
in rust these are tuples and arrays


tuples
let tup: (i:32, f64, u8) = (500, 6.4, 1);

to get a value
let (x, y, z) = tup;

println!("{y}");

or

println!("{x.1}");

arrays
let arr = [0,1,2]
let arr[i32; 2] = [1,2]
let arr[3;5]; // = [3,3,3,3,3]


to access arrays
the standard arr[i] works

---

functions
fn func()

fn func(x: i8)


with return
fn func() -> i8

---
comments are similiar to C#

---

control flow
stand if, else if, else

ternary

let x = if x {} else {}

loops

let x = loop {
    if con{
    break;
    }

}

loops can be labeled

'loop_name': loop{
    loop{
        if 'loop_name'{
        break;
        }
    }
}

standard while

*/
fn main(){}