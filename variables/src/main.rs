fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);


    // tuples
    let tup: (i32, f64, u8) = (500, 10.2, 1);
    println!("second value of the tuple: {}", tup.1);

    // destruct a tuple
    let (_,_, mut my_u8) = tup;

    println!("destructed through patternmatching - u8 value: {}", my_u8);
    my_u8 = 3;
    println!("after mutation: {}", my_u8);
    println!("tup.3 after mutation of my_u8: {}", tup.2);

}
