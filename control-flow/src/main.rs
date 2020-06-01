use rand::Rng;

fn main() {
    if true {
        println!("your normal if statement")
    }
    let rnd = rand::thread_rng().gen_range(1, 3);

    if rnd == 1 {
        println!("Congrats, you hit the 50% probability");
    }
    else {
        println!("Bummer! you didnt hit the 50% probability");
    }

    // since if is an expression, you can use it terniary-like. Very cool.
    let response = if rnd == 2 {"rnd is two!"} else {"rnd isnt two :("};
    println!("{}", response);


    let mut counter = 0;

    // NOTE: loops can have fucking RETURN TYPES because they´re an expression
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    while false {
        println!("i mean theres also while loops, usual shit")
    }

    // loop through an array:
    let important_numbers = [1,2,3,4,5];
    for number in important_numbers.iter() {
        println!("{}", number);
    }

    // (1..4) is definetly cool stuff
    for num in (1..4).rev() {
        println!("{}", num);
    }

    let _arr = 1..4;
}
