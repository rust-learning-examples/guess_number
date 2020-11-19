// extern crate rand;
use rand::{thread_rng, Rng};
use std::io;
use std::cmp::Ordering;
fn main() {
    let real_value:u32 = thread_rng().gen_range(0, 100);
    println!("请输入一个数字");
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).ok().expect("failed to read line");
        // let guess: u32 = guess.trim().parse().expect("not a number");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("近支持数字类型");
                continue;
            },
        };
        println!("guess: {}", guess);
    
        match guess.cmp(&real_value) {
            Ordering::Less => println!("too samll!"),
            Ordering::Greater => println!("too big!"),
            Ordering::Equal => {
                println!("you win!");
                break;
            }
        }    
    }
}
