/*
Project Management


Key Notes:
Packages -> build, test, share crates
Crates -> tree of modules which make a library
Modules ->"use" statements, control organization, scope and privacy of paths
Paths -> how an object is named (struct, fn, module, etc) 

---

crates are the smallest piece of code the rust compi;er considers at a time
crates can contain modules

two types of crates
binary and library crate
ie executable and shareable code

https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html#modules-cheat-sheet




there is a fair bit to digest here and it feels like one of those doing it is the best approach

the quick notes id share from doing is, every folder which contains modules needs a matching "folder_name.rs" in the src ie root/src directory

in this "folder_name.rs" file you define the modules as "pub mod" in order to use them in other scopes

each "pub mod" brought in this way has a linking "mod_name.rs" file which may or may not expose structs, enums, impls etc by including or omitting
the pub keyword



there are alternatives to this approach and as mentioned earlier, there are two types of crates, 
this is a binary crate so there may be some difference with a library crate


*/

fn main(){}