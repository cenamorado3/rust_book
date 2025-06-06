use std::io;//using/import statement
use std::cmp::Ordering;
use rand::{rng, Rng};
//stdlib = "prelude" => https://doc.rust-lang.org/std/prelude/index.html
fn main() {
    //humor me
    println!("Guess the number");//Console.WriteLine("")/print()/console.log...etc

    println!("Take a guess");//^

    //the book is using a deprecated function
    //let secret_number = rand::thread_rng().gen_range(1..=100);
    let rng  = rng().random_range(1..10).to_string(); //cargo add rand

    let mut guess = String::new();//create a mutable variable
    //if memory serves imutable variables are created my omitting mut
    let _x = 0; //WARNING: COMPILER WILL YELL AT YOU IF YOU DONT PREFIX UNUSED VARIABLES WITH AN UNDERSCORE....NEAT
    //correct, https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#variables-and-mutability 

    io::stdin()
    .read_line(&mut guess)// */& similiar to C reference 
    .expect("Failed to read");//seems like an assertion, will lazy load since it isnt immediately conducive to learning


      match guess.cmp(&rng) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
      }
      //i dont like this approach because its a string so...lets be a programmer

      println!("--------");
      println!("Guess a number between 1 and 10 within 3 tries.");//Console.WriteLine("")/print()/console.log...etc
      let rnd: i32 = rand::rng().random_range(0..10);
      //not a crate or module?
      //i dont understand why it works on line 13


      //not my parentheses!
      //it doesnt need to be mutable...and i do want it to be imutable in this case...

      //i chnged my mind, i want it to be mutable

      //what happened is that it failed to parse even though it was a valid "int"
      //its because you have to trim it

    let mut attempts:i32 = 3;
    //GET RID OF THE PARENTHESES
      while attempts > 0
      {
        let mut g = String::new();
        io::stdin()
      .read_line(&mut g)// */& similiar to C reference 
      .expect("Failed to read");//seems like an assertion, will lazy load since it isnt immediately conducive to learning

        let int_guess: i32 = match g.trim().parse::<i32>(){
          Ok(num) => num,
          Err(_) => 10
        };
        if &rnd == &int_guess{
            println!("That's right.");
            break;
        }
        if &rnd > &int_guess{//consider dereferencing the borrow, the what?
            println!("To small");
        }
        if &rnd <= &int_guess{
            println!("To big");
        }

        //the trait `PartialEq<i32>` is not implemented for `&i32`
        //help: consider removing the borrow




        if attempts > 0 {
          attempts -= 1;
          println!("{0} tries left", attempts); //standerize interpolation
        }
        if attempts <= 0{
          println!("You lose. The nunmber was {0}", rnd); //standerize interpolation
        }
        

    }
//nobody panic


//if you guess right in my code its right, but not in the books code?
//this is why you dont turbofish

//these types are weird
}
