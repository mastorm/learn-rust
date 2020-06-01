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

    // arrays are fixed size and end up on the stack
    // vectors are allowed to grow in size
    // use arrays when the size of the collection never changes
    let _my_arr = [1, 2, 3, 4];
    // if infering doesnt work, use this to specify size and type
    let _not_inferred: [u32; 5] = [1, 2, 3, 4, 5];
    // default values for arrays are also a thing
    let _default_vals = [3, 5]; // _default_vals = [3,3,3,3,3]
}
