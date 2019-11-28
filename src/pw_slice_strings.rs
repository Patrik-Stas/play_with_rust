pub(crate) fn run() {
    let s = String::from("MyString");
    let string_literal = "This is literal";

    either_string_or_literal(&s);
    either_string_or_literal(string_literal);
}

fn either_string_or_literal(s: &str) {
    println!("This is string slice {:} - might be literal or borrowed String", s);
}

// &String become


