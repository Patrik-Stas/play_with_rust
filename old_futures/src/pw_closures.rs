use std::collections::HashMap;

struct MyStruct {
    text: &'static str,
    number: u32,
}
impl MyStruct {
    fn new (text: &'static str, number: u32) -> MyStruct {
        MyStruct {
            text: text,
            number: number,
        }
    }
    // We have to specify that 'self' is an argument.
    fn get_number (&self) -> u32 {
        self.number
    }
    // We can specify different kinds of ownership and mutability of self.
    fn inc_number (&mut self) {
        self.number += 1;
    }
    // There are three different types of 'self'
    fn destructor (self) {
        println!("Destructing {}", self.text);
    }
}

fn is_fn <A, R>(_x: fn(A) -> R) {}
fn is_Fn <A, R, F: Fn(A) -> R> (_x: &F) {}
fn is_Fn2 <A, B, R, F: Fn(A, B) -> R> (_x: &F) {}
fn is_Fn3 <A, B, C, R, F: Fn(A, B, C) -> R> (_x: &F) {}
fn is_FnMut <A, R, F: FnMut(A) -> R> (_x: &F) {}
fn is_FnOnce <A, R, F: FnOnce(A) -> R> (_x: &F) {}


fn test_fn()
{
    let obj1 = MyStruct::new("Hello", 15);
    let obj2 = MyStruct::new("More Text", 10);

    fn func1 (x: &MyStruct) -> u32 {
        x.get_number() + 3
    }
    assert_eq!(func1(&obj1), 18);
    assert_eq!(func1(&obj2), 13);
    // compiles successfully.
    is_fn(func1);
    is_Fn(&func1);
    is_FnMut(&func1);
    is_FnOnce(&func1);
}

fn test_fn_closure()
{
    let obj1 = MyStruct::new("Hello", 15);
    let obj2 = MyStruct::new("More Text", 10);
    let closure1 = |x: &MyStruct| x.get_number() + 3;
    assert_eq!(closure1(&obj1), 18);
    assert_eq!(closure1(&obj2), 13);
    // compiles successfully.
    is_fn(closure1);
    is_Fn(&closure1);
    is_FnMut(&closure1);
    is_FnOnce(&closure1);
}

fn test_Fn()
{
    let mut obj1 = MyStruct::new("Hello", 15);
    let obj2 = MyStruct::new("More Text", 10);
// obj1 is borrowed by the closure immutably.
    let closure2 = |x: &MyStruct| x.get_number() + obj1.get_number();
    assert_eq!(closure2(&obj2), 25);
// We can borrow obj1 again immutably...
    assert_eq!(obj1.get_number(), 15);

// But we can't borrow it mutably.
// obj1.inc_number();               // ERROR, unless we enclose closure2 goes out of its scope before this is called

// Does not compile:
// is_fn(closure2);
// Compiles successfully:
    is_Fn(&closure2);
    is_FnMut(&closure2);
    is_FnOnce(&closure2);
}

fn test_FnMut()
{
    let mut obj1 = MyStruct::new("Hello", 15);
    let obj2 = MyStruct::new("More Text", 10);
// obj1 is borrowed by the closure mutably.
    let mut closure3 = |x: &MyStruct| {
        obj1.inc_number();
        x.get_number() + obj1.get_number()
    };
    assert_eq!(closure3(&obj2), 26);
    assert_eq!(closure3(&obj2), 27);
    assert_eq!(closure3(&obj2), 28);
// We can't borrow obj1 mutably or immutably
//    assert_eq!(obj1.get_number(), 18);   // ERROR
// obj1.inc_number();                   // ERROR

// is_fn(closure3);
// is_Fn(&closure3);
// Compiles successfully:
    is_FnMut(&closure3);
    is_FnOnce(&closure3);

}

pub fn test_FnOnce()
{
    let obj1 = MyStruct::new("Hello", 15);
    let obj2 = MyStruct::new("More Text", 10);
    // obj1 is owned by the closure
    let mut closure4 = move |x: &MyStruct| {
        obj1.get_number();
        x.get_number();
//        obj1 // this will make it fnOnce
    };
//    obj1.get_number();
    is_Fn(&closure4);
//    is_FnMut(&closure4);

// Compiles successfully:
//    is_FnOnce(&closure4);

//    assert_eq!(closure4(&obj2), 10);
// We can't call closure4 twice...
// assert_eq!(closure4(&obj2), 10);             //ERROR
// We can't borrow obj1 mutably or immutably
// assert_eq!(obj1.get_number(), 15);           // ERROR
// obj1.inc_number();                           // ERROR
}


pub fn test_multipleArgumnets()
{
    let obj1 = MyStruct::new("Hello", 15);
    let obj2 = MyStruct::new("More Text", 10);
    // obj1 is owned by the closure
    let closure4 =  |x: &MyStruct, y: &MyStruct| {
        obj1.get_number();
        x.get_number();
        y.get_number();
        obj1.get_number() + x.get_number() + y.get_number()
//        obj1 // this will make it fnOnce
    };
//    is_Fn(&closure4);
    is_Fn2(&closure4)
//    is_Fn3(&closure4)
}

pub fn run()
{
//    test_fn();
//    test_fn_closure();
//    test_Fn();
//    test_FnMut();
//    test_FnOnce();
    test_multipleArgumnets();
}