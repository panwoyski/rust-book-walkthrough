fn main() {
    let mut s1 = String::from("xD");
    println!("{}", s1);
    // let s2 = takes_ownership(&s1);
    // println!("{}", s2)
    borrow_immutable(&mut s1);
    println!("{}", s1)
}

// fn takes_ownership(s: &String) -> String {
//     println!("{}", s);
//     s
// }

fn borrow_immutable(s: &mut String) {
    s.push_str("DDDDDDDDDDDD");
}
