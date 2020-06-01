fn main() {
    println!("Double of 5: {}", double(5));

    let half_of_three = {
        // because this line has no semicolon, it is a expression(returns a value).
        // if i would add a semicolon at the end of it, it would turn into a statement
        // which cant return a value
        3 / 2
    };

    println!("Half of three: {}", half_of_three)
}

fn double(n: i32) -> i32 {
    n * 2
}