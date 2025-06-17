/*
ownership
---
ownership pertains to memory management

if ownership rules are violated the program will not compile

---

ownership rules:
each value has an owner
only one owner at a time
when owner leaves scope, values are dropped/memory is freed

---
let x = "some string" is immutable
let x = String::from("some string") is mutable, the :: is refered to as namespacing


re-definding a variable is called a "move" in rust ie

let x = String::from("x");
let x2 = x;

println!("{x}");

would throw an error as x has been moved to x2

---

scope and assignment follows similair convection to C# surely with some caveat, when a variable exits a scope is no longer used it is flagged for GC
in languages like C#, in rust memory is immediately freed/"dropped"

---

deep copy to work around moving

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");

---

stack only copy
    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");


unlike strings this works due to the rust ecosystem due to the implementation of the "copy trait" on scalar types

---

ownership and functions

with the previous notes and deep clones and copy traits, it's interesting to see how ownership rules are different from other languages and the importance
of returning a value


    let x: String = String::from("x");
    let i: i8 = 0;
    take_ownership(x); 
    //x no longer exist as it was dropped immediately after exiting the take_ownership function
    
    copy(i);
    //since i is an integery implemnting the "copy trait", unlike a string, i still exist immediately after the the invocation of copy

    fn take_ownership(s: String){ //consider -> String
        println!("{s}");
    }

    fn copy(i: i8){
        println!("{i}");
    }

---

referece and "borrowing"

rather than taking ownership and returning a valuable, which is a terrible idea, a reference can be used instead
    fn take_ownership(s: String){ //consider -> String
        println!("{s}");
    }

    fn borrow(s: &String){//rather than taking ownership use a reference instead, which is called borrowing in rust
        println!("{s}");

        //to quote the book
        //A reference is like a pointer in that it’s an address we can follow to access the data stored at that address; 
        //that data is owned by some other variable. 
        //Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.

        //this helps clear the air as someone more interested in C than any other language memory management
        //especially when & still means "reference" or "address of"
        //and * is still a dereferance of a pointer
    }


    fn copy(i: i8){
        println!("{i}");
    }

---

a critical thing to note is references are immutable by default, therefore, the value can not be changed,
since we have borrowed it and thus have not taken ownership. this can easily be fixed with a mutable reference, mut &borrow

    fn borrow(s: mut &String){//rather than taking ownership use a reference instead, which is called borrowing in rust
        println!("{s}");

the one caveat is that mutable references MUST NOT have any other references to that value

---

like other memoery unsafe languages, "dangling" can still happen, where in C this is usually a dangling pointer when not freed
in rust it is a dangling reference 

    fn dangle() -> &String {
        let s = String::from("hello");

        &s //this is due to a future concept called lifetimes
    }

    fn no_dangle() -> String {
        let s = String::from("hello");

        s
    }

---

&The Rules of References

Let’s recap what we’ve discussed about references:

    At any given time, you can have either one mutable reference or any number of immutable references.
    References must always be valid.

---

similiar to python, rust allows slicing with different syntax
with python
let slice = arr[start:stop:step]
with rust
let slice = &arr[start..stop] t

unlike python there is no step feature for slices and must be positive, negative values can not be used for reversing


for all the more complex stuff see 
https://doc.rust-lang.org/book/ch08-02-strings.html#storing-utf-8-encoded-text-with-strings 
*/
fn main(){}