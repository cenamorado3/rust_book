use crate::modules::echo::Echo;
use crate::modules::chapter_2::GuessingGame;

pub mod modules;

fn main() {
    let e = Echo{
        message: String::from("echo")
    };
    e.echo();


    GuessingGame::guess();
}
