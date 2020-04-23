use std::io::stdin;

fn main() {
    'outer: loop {
        let number: i32 = 10;
        println!("Pick a Number");
        
        loop {
            let mut line = String::new();
    
            let input = stdin().read_line(&mut line);

            let guess: Option<i32> = input.ok().map_or(None, |_| line.trim().parse().ok());

            match guess {
                None => println!("Enter a Number"),
                Some(n) if n == number => {
                    println!("You Guessed it");
                    break 'outer;
                }
                Some(n) if n < number => println!("Too Low"),
                Some(n) if n > number => println!("Too High"),
                Some(_) => println!("Error")
            }
        }
    }
}