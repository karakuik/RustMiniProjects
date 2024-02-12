fn main() {
    let mut number = 3;
    
    if number < 5 {
         println!("Condition is true");
    }
    else{
        println!("The condition was false");
    }
    number = 0;
    loop {
        println!("Number: {number}");
        number += 1;
        
        if number > 10 {break;}
    }
    counter();
    liftoff();
    forLoop();
}

fn counter() {

    let mut counter = 0;
    
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }

    };
    println!("The Result is {result}!");
}

fn liftoff() {

    let mut number = 3;
    while number !=0 {
        println!("{}!", number);
        number -=1;

     }
   
    println!("LIFTOFF!");
 
}

fn forLoop(){

     let a = [10, 20, 30, 40, 50];
     let mut index = 0;
     while index < 5 {
        println!("The value is: {}", a[index]);
        index += 1;
    }

    println!("For loop style:");

    for element in a {
         println!("the value is: {element}");
    }

    //Alternative Counting style for for loop
    println!("For loop counting");
    
    for number in (1..4).rev() {
        println!("{number}!"};
    }

}
