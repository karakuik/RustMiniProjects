use std::io;

fn main() {
    println!("Please enter in a number to fibonacci");

    let mut input = String::new();

    io::stdin()
       .read_line(&mut input)
       .expect("Failed to Read Line");

    let number: u64 = match input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
           println!("Invalid Input, please enter in a valid integer!");
           return;
         }
    };

    println!("{}", fibonacci_calculator(number)); 
}


fn fibonacci_calculator(number: u64) -> u64 {

let mut current = 1;
let mut prev = 0;

    if number == 1 {
         0
    }
    else if number == 2 {
         1
    }
   else {

       for _ in 3..number {
           println!("{} ", current);
           let next = prev + current;
           prev = current;
           current = next;
       }
    current
    }

}
