fn main() {
    println!("Hello, world!");
    another_function (5, 'h');
    wack_statement();
    let x = five();
    
    println!("The Value of x is {x}");
}

fn another_function(value: i32, unit_label: char){
    println!("Hello from inside another function!");
    println!("We are passing in value and unit label: {value}, {unit_label}");
}
fn wack_statement(){
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is {y}");

}

fn five() -> i32 {
5
}
