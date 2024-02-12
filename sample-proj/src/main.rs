fn main() {
    println!("Hello, world!");
    let mut x = 5;
    println!("{x}");
    x = 10;
    println!("{x}");
    {
	println!("{x}");
    }
    let spaces = "    ";
    let spaces = spaces.len();
    
    println!("{spaces}");

    // Can Access values in a mpped manner, such as a = 500, e = "b"
    println!("Tuple time!");
    let tup = (500, 6.2, 1, 'A', "b");
    let (a, b, c, d, e) = tup;
    println!("The value of a is {b}");
    
    // Can also access values via X.Y
    let tupleTwo: (i32, f64, u8) = (-500, 6.4, 1);
    let five_hundred = tupleTwo.0;
    let six_point_four = tupleTwo.1;
    let one = tupleTwo.2;

    println!("{five_hundred}, {six_point_four}, {one}");

    println!("###Array time!###");

    let a = [1,2,3,4,5];
    // alternatively....
    let a: [i32; 5] = [1,2,3,4,5];

    //Or if you want 5 3s...
    let a = [3;5];

    // And Of course...
    let first = a[0];
    let second = a[1];

}
