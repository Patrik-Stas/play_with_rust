#![allow(dead_code)]
use std::mem;


pub fn run()
{
    use std::io::{stdin,stdout,Write};
    let mut s= String::new();
    print!("Please enter some text: ");
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    let r = "foobar";
    println!("You typed: {}, our string is {}",s, r);
    let sref = &s;
    let equal1: bool = r == s;
    let equal2: bool = r == sref;
    println!("Equal r = s    ... {:?}", equal1);
    println!("Equal r = sref ... {:?}", equal2);
}

