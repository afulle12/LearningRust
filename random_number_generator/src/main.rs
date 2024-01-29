use rand::prelude::*;

fn main(){
    let output: u8 = rand::thread_rng().gen();
    println!("The random number generated was: {} ", output);
}
