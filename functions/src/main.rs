fn main() {
    let y = {
        let x = 42;
        x*2
    };
    another_fun(y, 43);

    println!("{}", five())
}

fn another_fun(x: u32, y: u32) {
    println!("{} {}", x, y);
}

fn five() -> u32 {
    5
}
