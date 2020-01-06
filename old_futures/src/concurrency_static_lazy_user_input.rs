struct SomeData { id: u32 }

struct ComplexFoo { some_data: SomeData }

static mut SELECTED_STRATEGY: &ComplexFoo = &ComplexFoo {some_data: SomeData{ id: 1 }};

lazy_static! {
    /// This is an example for using doc comment attributes
    static ref EXAMPLE: u8 = 42;
    static ref EXAMPLE2: u8 = {
        println!("Evaluating example2");
        use std::io::{stdin,stdout,Write};
        let mut s=String::new();
        print!("Please enter some text: ");
        let _=stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        if let Some('\n')=s.chars().next_back() {
            s.pop();
        }
        if let Some('\r')=s.chars().next_back() {
            s.pop();
        }
        println!("You typed: {}",s);
        123
    };
    static ref EXAMPLE666: u8 = {
        println!("This will not be called");
        123
    };
}
pub fn run()
{
    println!("Run prints example2: {:?}", *EXAMPLE2);
}