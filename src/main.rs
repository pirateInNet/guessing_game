use std::io;
use rand::Rng;
fn main() {
 
    let mut rng = rand::rng();
    let secret_num: u32 = rng.random_range(1..11);

    println!("guess brother! ");
    println!("from 1 to 10 ");


    let mut x = 0;
    loop {

        println!("inter:");
        let mut guess = String::new();
            io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number!");
                continue;
            }
        };

        if secret_num == guess {
            println!("you won!!!");
            println!("the secret number is {secret_num}");
            break;
        } else if secret_num < guess {
            println!("no, secret number is smaller than your guess");
        } else if secret_num > guess {
            println!("no, secret number is bigger than your guess");
        }

        x += 1;
        if x >= 3 {
        	println!("no more chances");
            break;
        }
    }

}
