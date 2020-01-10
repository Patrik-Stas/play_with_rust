pub fn run() {

    let val: &Option<String> = &None;

    match *val {
        Some(ref s) => println!("the string is {}", s),
        None => println!("no string")
    }

    // after Match Ergonomics in 2016 https://rust-lang.github.io/rfcs/2005-match-ergonomics.html
    // following is possible
    match val {
        Some( s) => println!("the string is {}", s),
        None => println!("no string")
    }


}
