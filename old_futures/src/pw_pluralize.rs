fn pluralize(s: &str) -> String {
    s.to_owned() + "s"
}

pub fn run()
{
    let s = String::from("book");
    let p = pluralize(&s);
    println!(
        "I have one {}, you have two {}",
        s,
        p
    );
}