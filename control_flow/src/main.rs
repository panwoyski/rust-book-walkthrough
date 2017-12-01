fn main() {
    // if_example();
    // loop_example();
    range_example();
}

fn if_example() {
    let x = 5;

    let y = if x < 3 {
        println!("Less than three");
        1
    } else {
        println!("Not less than three");
        4
    };

    println!("{}", y)
}

fn loop_example() {
    let a = [1,2,3,4,5];

    for element in a.iter() {
        println!("val: {}", element);
    }
}

fn range_example() {
    for  number in (1..4).rev() {
        println!("val: {}", number);
    }
}
