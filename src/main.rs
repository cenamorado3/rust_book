use crate::modules::chapter_9::dream_theather;
pub mod modules;

fn main() {
    let r = dream_theather::systematic_chaos("404.txt");
    //the error has propogated/"bubbled" we have to deal with it eventually
    match r {
        Ok(txt) => println!("{}", txt),
        Err(e) => panic!("{}", e.to_string())
    };
    //there are some tricks outlined in the book, could vs should.
}
