use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn guess_game(num_tries: u32) -> u32 {
    const MIN: u32 = 1; 
    const MAX: u32 = 100;

    let secret = rand::thread_rng().gen_range(MIN..=MAX);

    let mut count = 0;
    loop {
        let mut guess = String::new();
        println!("Guess the number!");
        io::stdin()
            .read_line(&mut guess)
            .expect("Unable to read line");
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)  => {
                println!("\t--Not a valid number!--");
                continue;
            }
        };
        
        match guess.cmp(&secret) {
            Ordering::Less => println!("\tToo small!"),
            Ordering::Greater => println!("\tToo big!"),
            Ordering::Equal => {
                println!("Correct!");
                break;
            }
        }
        if guess != secret {
            println!("\tYou guessed {guess} but the number is {secret}");
        }
        count += 1;
        if count == num_tries {
            println!("You have exhausted all your tries. Good luck next time!");
            break;
        }
    }
    secret
}

/* Inefficient Recursion */
fn fib(n: u32) -> u64 {
    if n < 1 { return 0 }
    if n == 1 || n == 2 { return 1 } 
    return fib(n-1) + fib(n-2)
}

/* Bottom-Up Approach */
fn fib1(n: u32) -> u64 {
    if n < 1 { return 0 }
    if n == 1 || n == 2 { return 1 } 

    let mut x1 = 1;
    let mut x2 = 1;
    let mut x3 = x1 + x2;
    for _ in 3..n {  // 3..=n => n is inclusive
        x1 = x2;
        x2 = x3;
        x3 = x1 + x2;
    }
    x3
}

fn main() {
    let a: [u32; 5] = [1,2,3,4,5];
    let b: (u32, u32) = (1, 2);
    println!("a[1] = {}, b[1] = {}", a[1], b.1);

    let secret = guess_game(5);
    println!("A bonus for you, fib({}) = {}", secret, fib1(secret))
}
