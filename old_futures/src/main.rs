#![allow(dead_code)]
#![allow(unused_variable)]
#[macro_use]
extern crate quick_error;
#[macro_use]
extern crate error_chain;

extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate chrono;

#[macro_use]
extern crate futures;

#[macro_use]
extern crate lazy_static;

mod sh;
mod circle;
mod square;
mod pwclosures;
mod pwaliasing;
mod pwcopy;
mod pwhyper;
mod pwreqwest;
mod pwdestructuring;
mod pwpatrik;
mod borrowselfissue;
mod pw_closures;
mod pw_borrow_move;
mod pw_pluralize;
mod pw_slice_args;
mod pw_slice_strings;
mod rim_2_7_borrow_patterns;
mod rim_2_7_entryapi;
//mod rim_2_7_structs;
mod rim_2_7_structs_refactored;
mod rim_2_7_tempvar;
mod rim_2_8_socket;
mod rim_3_4_options_errors;
mod rim_3_5_multiple_errors;
mod rim_3_6_custom_errors;
mod rim_3_7_quick_error;
mod rim_3_7_error_chain;
mod rim_3_7_failures;
mod stackcheap;
mod pw_move_address;
mod rim_4_4_stemmer;
mod pw_copy_trait;
mod pw_non_copiable_and_closures;
mod pw_clone_options;
mod futures_either;
//mod futures_in_ifs;
mod lifetime_futures;
mod futures_in_parallel;
mod devto_future_custom_implementation;
mod devto_future_custom_implementation2;
mod static_mut_so_question_refreturn;
mod static_mut_so_question_simplified;
mod static_mut_so_question_simplified_lazy_static;
mod devto_streams;
mod futures_to_streams;
mod snoyman_iterators;
mod snoyman_3;
mod snoyman_closures;
mod snoyman_streams_exercise6;
mod snoyman_streams_exercise7;
mod concurrency_pw_threads;
mod ExpensiveFuture;
mod tokio_example_resolveconnect;
mod str_compare;
mod static_mut;
mod static_mut_so_question;
mod static_lazy_with_mutex;
mod static_mut_unsafe;
mod concurrency_thread_local;
mod concurrency_thread_local_with_rc;
mod concurrency_thread_local_with_arc;
mod concurrency_cell;
mod concurrency_cell_usecase;
mod concurrency_refcell;
mod concurrency_thread_local_with_rwlock;
mod concurrency_racing_1;
mod concurrency_racing_2;
mod concurrency_send_sync_issue;
mod concurrency_cell_mine;
mod concurrency_factory_creatin_send_sync;
mod concurrency_arc;
mod concurrency_rw_lock_example_configs;
mod concurrency_arc_mutex;
mod global_mutable;
mod static_lazy_hashmap;
mod concurrency_rw_lock;
mod ref_keyword_matching;
mod global_mutable_integer;
mod ref_keyword;
mod ref_keyword_2;
mod so1;
mod static_lazy_vector;
mod channels;
use std::mem;

const MEANING_OF_LIFE:u8 = 42; // no fixed address

static mut Z:i32 = 12312; // if you need to have address on your global variable, but you'll have to take care of its safety if it's mutable one

fn operators() {

    let mut a = 2+3*4;
    println!("{}", a);
    a = a+1;
    a -= 2;

    println!("remainder of {} / {} is {}", a, 3, (a%3));
    let a_cubed = i32::pow(a, 3);
    println!("{} cubed is {}", a, a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3); // integral power
    let b_to_pi= f64::powf(b, std::f64::consts::PI); // more expensive floating power
    println!("{};  cubed={}, {}^pi={}", b, b_cubed, b, b_to_pi);

    // bitwise is only available for integers
    // ... | OR     & AND     ^ XOR     ! NOR
    let c = 1 | 2; // 01 OR 10 = 11 = (dec)3
    println!("c = {}", c);

    let two_to_10 = 1 << 10; // we have also opposite shift >>
    println!("2^10 = {}", two_to_10);

    let _pi_less_4 = std::f64::consts::PI < 4.0;
}

fn datatypes() {
    let a:u8 = 123; // 8 bits
    println!("a = {}", a);

    let mut b:i8 = 0;
    println!("b = {}", b);
    b=111;
    println!("b = {}", b);

    let mut c = 132132131; // 32 bit signed i32
    println!("c = {}   size = {}", c, mem::size_of_val(&c));
    c=-1;
    println!("after modification c = {}   size = {}", c, mem::size_of_val(&c));

    let z:isize = 123; // isize size of word in memory - 32 on 32bit systems, 64 on 64bit systems
    let size_of_z = mem::size_of_val(&z);
    println!("z = {} takes up {} bytes => {}bit OS", c, size_of_z, size_of_z * 8);

    let d = 'x';
    println!("after modification d = {}   size = {}", d, mem::size_of_val(&d)); // 4 bytes, char datatype is encodes unicode

    let e = 2.5; // double precision, 8bytes/64bits, f64
    println!("after modification e = {}   size = {}", e, mem::size_of_val(&e)); // 4 bytes, char datatype is encodes unicode

    let ee:f32 = 2.5;
    println!("after modification ee = {}   size = {}", ee, mem::size_of_val(&ee));

    let g = false;
    println!("after modification g = {}   size = {}", g, mem::size_of_val(&g));
}

fn scope_and_shadowing() {
    let a = 123;
    let a = 777;
    {
        let b = 456;
        println!("b is {}", b);
        let a = 1000;
        println!("a inside scope is {}", a);

    }
    println!("a is {}", a);
}

fn global_variables() {
    println!("meaning of life {}", MEANING_OF_LIFE);
    unsafe {
        println!("static constant = {}", Z);
    }
}

fn if_statement() {

    let temp = -18;
    if temp > 30
    {
        println!("Really hot outside.")
    }
    else if temp < 10
    {
        println!("Really cold.")
    }
    else
    {
        println!("Temperature is ok.")
    }
    // if statement is actually expression
    let day = if temp > 20 {"sunny"} else {"cloudy"};
    println!("today is {}", day );

    println!("Lastly, it is {}",
        if temp > 20 {"hot"} else if temp < 10 {"cold"} else {"OK"}
    );

    println!("it is ... {} ",
        if temp > 20 {
            if temp > 30 {"very hot"} else {"hot"}
        } else if temp < 10 {"cold"} else {"ok"}
    )
}

fn while_loop() {
    let mut x = 1;
    while x < 1000
    {
        x *= 2;
        if x == 64 {continue;}
        println!("x = {}", x);
    }
    let mut y = 1;
    loop // == while true
    {
        y *= 2;
        println!("y = {}", y);
        if y == 1<<10 { break; }
    }
}

fn for_loop() {
    for x in 1..11 // unlike in match statement range, 11 is exclusive
    {
        if x == 3 {continue;}
        if x == 8 {break;}
        println!("x = {}", x)
    }

    for (pos, y) in (30..41).enumerate()
    {
        println!("pos={}, y={}", pos, y)
    }
}

fn match_statement() {
//    let country_code = 9990;
    let country_code = 999;
    let country = match country_code
    {
        44 => "UK",
        46 => "Russia",
        1...999 => "unknown", /// unlike in for cycle range, 999 is inclusive here
        _  => "INVALID"
    };

    println!("the country with code {} is {}", country_code, country)
}

#[derive(Debug)]
struct Point
{
    x: f64,
    y: f64
}

#[derive(Debug)]
struct Line
{
    start: Point,
    end: Point
}


fn structures()
{
    let p = Point { x: 3.0, y : 4.5 };
    println!("point is at ({}, {})", p.x, p.y);

    let p2 = Point { x: -5.0, y : 10.0 };
    let myline = Line { start: p, end: p2 };
    println!("My line is {:?}", myline)

}

enum Color
{
    Red,
    Green,
    Blue,
    RgbColor(u8,u8,u8),
    CmykColor{cyan: u8, magenta:u8, yellow:u8, black:u8}, // struct
}


fn enums()
{
//    let c:Color = Color::RgbColor(0,0,0);
//    let c:Color = Color::RgbColor(0,0,1);
//    let c:Color = Color::CmykColor {cyan:0, magenta: 10, yellow: 23, black:0};
    let c:Color = Color::CmykColor {cyan:0, magenta: 10, yellow: 23, black:255};
    match c
        {
            Color::Red => println!("r"),
            Color::Green => println!("g"),
            Color::Blue => println!("b"),
//            Color::RgbColor(0,0,0) | Color::CmykColor{cyan:_, magenta:_, yellow:_, black:255}=> println!("Black"),
            Color::RgbColor(0,0,0) | Color::CmykColor{black:255, ..}=> println!("Black"), // .. refers to all other struct values I don't care about
            Color::RgbColor(r,g,b) => println!("rgb {},{},{}",r,g,b),
            _ => ()
        }
}

union IntOrFloat
{
    i: i32,
    f: f32
}

fn process_value(iof: IntOrFloat)
{
    unsafe
        {
            match iof {
                IntOrFloat{i:42} => {println!("meaning of life")}
                IntOrFloat{f } => {println!("f32 = {}", f); }
            }
        }
}

fn unions()
{
    let mut iof = IntOrFloat { i : 123 };
    unsafe { iof.i = 42 }
    let value = unsafe { iof.i };
    process_value(iof);
    process_value(IntOrFloat{f:1.23});
}

fn option()
{
    // Option<T>
    let x = 3.0;
    let y = 0.00;
    let y = 2.22;

    // Some(z) or None
    let result: Option<f64> =
        if y != 0.0 {Some(x/y)} else {None};

    println!("{:?}", result);

    match result
        {
            Some(z) => println!("{}/{} = {}", x,y,z),
            None => {println!("Cannot divide {} by {}", x, y)}
        }

    // if destructuring fails, the if body will not execute
    if let Some(z) = result { println!("z = {}", z) }
}

fn array()
{
    // keep in mind: you cant resize an array

    let mut a:[i32;5] = [1,2,3,4,5];
    println!("a has '{}' elements, first is '{}'", a.len(), a[0]);
    a[0]=321;
    println!("a[0] = {}", a[0]);
    println!("a = {:?}", a);

    if a != [1,2,3,4,5]
    {
        println!("Original array has been modified.")
    }
    if a == [321,2,3,4,5]
    {
        println!("Got match here.")
    }

    let b = [1u16; 10]; // i want 10 elements, each should be 1
    let c = [1u32; 10]; // i want 10 elements, each should be 1
    let d = [1; 10]; // i want 10 elements, each should be 1
    let e = [1u128; 10]; // i want 10 elements, each should be 1
    for i in 0..b.len()
    {
        println!("Array element: {}", i)
    }

    println!("b took {} bytes", mem::size_of_val(&b));
    println!("c took {} bytes", mem::size_of_val(&c));
    println!("d took {} bytes", mem::size_of_val(&d));
    println!("e took {} bytes", mem::size_of_val(&e));

    let mtx:[[f32;3]; 2] =
    [
        [1.0, 0.0, 0.0],
        [0.0, 2.0, 3.0]
    ];
    println!("2d matrix = {:?}", mtx);

    for i in 0..mtx.len()
        {
            for j in 0..mtx[i].len()
                {
                    if i == j // print on diagonal
                    {
                        println!("mtx[{}][{}] = {}", i, j, mtx[i][j])
                    }
                }
        }
}

fn vector()
{
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);
    println!("vec = {:?}", a);
    a.push(3);
    println!("vec = {:?}", a);
    println!("vec[0] = {:?}", a[0]);

    let idx:usize = 1;
    println!("vec[{}] = {:?}", idx, a[idx]);

    match a.get(3)
        {
            Some(x) => println!("a[3]={}", x),
            None => println!("no such element")
        }

    for x in &a { println!("{}", x)}
    a.push(123);
    println!("{:?}", a);
    let last_elem = a.pop();
    println!("last element is {:?}, a = {:?}", last_elem, a);

    while let Some(x) = a.pop()
        {
            println!("{}", x)
        }
}

fn use_slice(slice: &mut[i32]) // &[i32] denotes slice of an array of i32
{
    println!("first elem = {}, slice len = {}", slice[0], slice.len());
    println!("full slice = {:?}", slice);
    slice[0]+=100
}


fn slice()
{
    let mut data = [11,22,33,44,55,66];
    use_slice(&mut data[2..5]);
    use_slice(&mut data);
    println!("final array = {:?}", data);
}

fn mySliceTest()
{
    let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let iterTemp = a.iter()
        .map(|&x| x * 10)
        .rev()
        .filter(|&x| x > 30);
    let mut doubled: Vec<i32> = iterTemp.collect();
    let doubled_chunks: Vec<_> = doubled.chunks(3).collect();
    println!("doubled = {:?}", doubled);
    println!("doubled chunks = {:?}", doubled_chunks);


    // slicing a Vec
    let mut vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let tmp: Vec<_> = vec.chunks(2).collect();
    println!("{:?}", tmp);

    let (left, right) = vec.split_at_mut(4 as usize);
//    let int_slice = &mut vec[0..6];
//    let int_slice2 = &mut vec[7..9];
    println!("slice 1 = {:?}", left);
    println!("slice 2 = {:?}", right);
    let str_slice: &[&str] = &["one", "two", "three"];
}

fn strings()
{
    let s:&'static str = "hello theree";
    // &str - string slice
    // static says that it will be included in the program, glued in code we compile
    // you also can't reference individual chars like println({"{}", s[0])
    for c in s.chars().rev()
        {
            println!("{}", c)
        }

    if let Some(first) = s.chars().nth(0)
    {
        println!("first letter is {}", first)
    }

    //heap
    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8)
        {
            letters.push(a as char);
            letters.push_str(",");
            a += 1
        }
    println!("{}", letters);

}

fn sum_and_product(x:i32, y:i32) -> (i32, i32)
{
    (x+y, x*y)
}

fn tuple()
{
    let x = 3;
    let y = 4;
    let sp = sum_and_product(x,y);
    let (a,b) = sp;

    println!("sp = {:?}", sp);
    println!("{0}+{1}={2} .... {0}*{1}={3}", x, y, sp.0, sp.1);
    println!("{0}+{1}={2} .... {0}*{1}={3}", x, y, a, b);

    let sp2 = sum_and_product(10,30);
    let combined = (sp, sp2);
    println!("combined={:?}", combined);
    println!("last elem = {}", (combined.1).1);

    let ((c,d),(e,f)) = combined;
    println!("combined destructured: {}, {}, {}, {}", c, d ,e, f);

    let foo = (true, 42.0, -1i8);
    println!("foo = {:?}", foo);

    let meaning = (42,);
    println!("tuple of single element {:#?}", meaning);
}

fn how_many(x:i32) -> &'static str
{
    match x
        {
            0 => "no",
            1 | 2 => "one or two",
            12 => "dozen",
            z @ 9...11 => "lots of",
            _ if (x % 2 == 0) => "some",
            _ => "few"
        }
}

fn pattern_matching()
{
    for x in 0..13
        {
            println!("{}: I have {} oranges", x, how_many(x))
        }

    let point = (0,4);

    match point
        {
            (0,0) => println!("origin"),
            (0,y) => println!("on x axis, y={}", y),
            (x,0) => println!("on y axis, x={}", x),
//            (ref mut x,0) => println!("on y axis, x={}", x), // in this case we could even modify he value of x
            (_,y) => println!("(?,{})", y) // _ means that value can be whatever and I don't care what it is
        }
}


#[derive(Debug)]
struct GenericPoint<T>
{
    y: T,
    x: T
}

#[derive(Debug)]
struct GenLine<T>
{
    start: GenericPoint<T>,
    end: GenericPoint<T>
}

fn generics()
{
    let a = GenericPoint{x:0.0, y:2.1};
//    let b = GenericPoint{x:0, y:2.0}; different types, won't work
    let c : GenericPoint<i32> = GenericPoint{x:0, y:2}; // we can be explicit if we want to
    let d : GenericPoint<f64> = GenericPoint{x:0.0, y:2.232323};

    let line = GenLine{start:a, end:d};

    println!("Generic line = {:#?}", line)
}

fn print_value(x: i32)
{
    println!("value = {}", x);
}

fn increment(x: &mut i32)
{
    *x += 1
}

fn product(x:i32, y:i32) -> i32
{
    x * y
}

fn functions()
{
    let mut z = 1;
    increment(&mut z);
    print_value(z);

    let a = 3;
    let b = 5;
    let p = product(a, b);
    println!("{} * {} = {}", a, b, p);

}

impl Line
{
    fn len(&self) -> f64
    {
        let dx = self.start.x - self.end.x;
        let dy = self.start.y - self.end.y;
        (dx*dx + dy*dy).sqrt()
    }
}

fn methods()
{
    let p1 = Point{ x: 0.0, y: 4.2 };
    let p2 = Point{ x: 10.0, y: 6.2 };
    let p3 = Point{ x: 50.0, y: 100.2 };
    let myline = Line{ start: p1, end: p2};

    println!("line length = {} ", myline.len())
}


fn say_hello()
{
    println!("Hello.")
}

fn closures()
{
    say_hello();
    let sh = say_hello;
    sh();

    let plus_one = |x:i32| -> i32 { x + 1 };
    let a = 5;
    println!("{} + 1 = {}", a, plus_one(a));

    let mut two = 2;
    let plus_two = |x|
        {
            let mut z = x; // just to demonstrate that closure can contain any valid rust constructs
            z += two;
            z
        };

    println!("{} + 2 = {}", a, plus_two(a));
    let borrow_two = &mut two;

    let plus_three = |x:&mut i32| *x += 32;
    let mut f = 12;
    plus_three(&mut f);
    println!("f = {}", f);


    let mut g = 12;
    let plus_four = |mut x:i32| x += 32; // the position syntax of "mut" is different than in case of i32 reference
    plus_four(g);
    println!("g = {}", g); // is still 12, because we in plus_four we only increased a mutable local variable, not reference
}

fn is_even(x: i32) -> bool
{
    x % 2 == 0
}

fn high_order_functions() // hof - functions which takes other functions as parameters
{
    let limit = 500;
    let mut sum = 0;
    for i in 0..
        {
            let isq = i*i;
            if isq > limit { break; }
            else if is_even(isq) { sum += isq }
        }
    println!("sum is = {}", sum);

    let sum2 = (0..)
        .map(|x| x*x)
        .take_while(|&x| x <= limit)
        .filter(|x| is_even(*x))
        .fold(0, |sum, x| sum + x);
    println!("sum2 = {}", sum2)
}

trait Animal
{
    fn create(name: &'static str) -> Self;

    fn name(&self) -> &'static str;

    fn talk(&self)
    {
        println!("{} cannot talk", self.name())
    }
}

struct  Human
{
    name: &'static str
}


struct  Cat
{
    name: &'static str,
    sound: &'static str
}

impl Animal for Human
{
    fn create(name: &'static str) -> Human {
        Human{name}
    }

    fn name(&self) -> &'static str
    {
        self.name
    }

    fn talk(&self)
    {
        println!("{} says hello", self.name)
    }
}

impl Animal for Cat
{
    fn create(name: &'static str) -> Cat {
        Cat{name, sound:"meowee"}
    }

    fn name(&self) -> &'static str
    {
        self.name
    }

    fn talk(&self)
    {
        println!("A catto {} says {} {}", self.name, self.sound, self.sound)
    }
}

trait Summable<T>
{
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32>
{
    fn sum(&self) -> i32
    {
        let mut result:i32 = 0;
        for x in self { result += *x; }
        return result
    }
}

fn traits()
{
//    let h= Human::create("Patrik");
    let h:Human = Animal::create("Patrik");
    let c = Cat{name:"Mindy", sound:"meow"};
    let c2:Cat = Animal::create("Blacky"); // based on c2 type, Rust can figure out what Animal::Create implementation to pick
    h.talk();
    c.talk();
    c2.talk();

    let a = vec![1,2,3];
    println!("vector sum is = {}", a.sum())
}

struct NamedPerson
{
    name: String
}

impl NamedPerson
{
//    fn new(name: &str) -> NamedPerson
//    {
//        NamedPerson {name: name.to_string()}
//    }

//    fn new<S: Into<String>>(name: S) -> NamedPerson
//    {
//        NamedPerson { name: name.into() }
//    }


    fn new<S>(name: S) -> NamedPerson
        where S: Into<String>
    {
        NamedPerson { name: name.into() }
    }
}

// Experiment: Seems like this cannot be done due to 'rust orphan rules' which does not allow to
// implement traits from different crate on structures from also different crate. At least
// one of those pieces would have to be defined in my crate
//impl std::convert::From<i32> for std::string::String
//{
//    fn from(x: i32) -> std::string::String
//    {
//        x.to_string()
//    }
//}

fn into()
{
    let john = NamedPerson::new("John");
    let janeName = "Jane".to_string();
//    let jane = NamedPerson::new(janeName.as_ref());
    let jane = NamedPerson::new(janeName);
//    let numberPerson = NamedPerson::new(45);
}


struct Creature
{
    name: String
}

impl Creature {
    fn new(name: &str) -> Creature
    {
        println!("{} enters the game", name);
        Creature {name: name.into()}
    }
}

impl Drop for Creature
{
    fn drop(&mut self) {
        println!("{} is dead.", self.name);
    }
}
//fn drop()
//{
//    let Josh = Goblin::new("Josh")
//}

fn droptest()
{
    let clever: Creature;
    {
        let goblin = Creature::new("Jeff");
        println!("Game proceeds.");
        clever = goblin;
        println!("Goblin's name is {}", clever.name); // have to use clever here, cause the Creature was moved into "clever"!
    }
    println!("Inner scope finished");
    println!("Clevers's name is {}", clever.name)
}


use std::ops::{Add, AddAssign};

#[derive(Debug, PartialEq)]
struct Complex<T>
{
    re: T,
    im: T
}

impl<T> Complex<T>
{
    fn new(re: T, im: T) -> Complex<T>
    {
        Complex::<T>{ re, im }
    }
}

impl<T> Add for Complex<T>
    where T: Add<Output = T>
{
    type Output = Complex<T>;

    // a + b
    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im
        }
    }
}

impl<T> AddAssign for Complex<T>
    where T: AddAssign<T>
{
    fn add_assign(&mut self, rhs: Self) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

// We don't have to implement this, we can use #[derive()] to generate partial equality trait for our struct!
//impl<T> PartialEq for Complex<T>
//    where T: PartialEq<T>
//{
//    fn eq(&self, rhs: &Self) -> bool {
//        self.re == rhs.re && self.im == rhs.im
//    }
//}

fn operator_overloading_1()
{
    let mut a = Complex::new(1.0, 2.0);
    let mut b = Complex::new(3.2, 4.1);
    let mut d = Complex::new(1.0, 2.0);
    println!("a={:?}", a);
    println!("b={:?}", b);
//    let mut c = a+b;
//    println!("c=a+b={:?}", c);
//    a+=b;
//    println!("a=a+b={:?}", a);
    println!("a==b = {:?}", a == b);
    println!("a==d = {:?}", a == d);
}


impl Add for Point
{
    type Output = Point;
    fn add(self, other:Point) -> Point
    {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}

fn operator_overloading()
{
    let p1 = Point{ x: 0.0, y: 4.2 };
    let p2 = Point{ x: 10.0, y: 6.2 };
    let p3 = p1 + p2;
    println!("P1 + P2 = {:#?}", p3)
}

trait Printable
{
    fn format(&self) ->  String;
}

impl Printable for i32
{
    fn format(&self) -> String
    {
        format!("i32: {}", *self)
    }
}

impl Printable for String
{
    fn format(&self) -> String
    {
        format!("string: {}", *self)
    }
}
//
//fn print(z:&Printable)
//{
//
//}


fn print_it<T: Printable>(z: T)
{
    println!("{}", z.format());
}

fn static_dispatch()
{
    let a = 123;
    let b = "hello".to_string();

// when you compile this code, compiler will generate concrete versions for of print_it for all types you've used to call it with. This is called "Monomorphization"
    print_it(a); // z.format() here is static dispatch. The format implementation is looked up in compile time.
// Monomorphization will for the String call above generate this:
//fn print_it(z: String)
//{
//    println!("{}", z.format());
//}
    print_it(b);
// and wil lgenerate this for call with "b"  argument
//fn print_it(z: i32)
//{
//    println!("{}", z.format());
//
}

// type erasure happens when argument is passed. All we know this is Printable object
fn print_it_2(z: &Printable)
{
    println!("{}", z.format());
    // z.format() represents dynamic dispatch, this is more expensive because in runtime must be looked up which Printable implementation to use
}

fn dynamic_dispatch()
{
    let a = 123;
    let b = "hello".to_string();


    print_it_2(&a);
    print_it_2(&b);
}


struct Circle { radius: f64 }
struct Square { side: f64 }

trait Shape {
    fn area(&self) -> f64;
}

impl Shape for Square
{
    fn area(&self) -> f64
    {
        self.side*self.side
    }
}

impl Shape for Circle
{
    fn area(&self) -> f64
    {
        self.radius*self.radius*std::f64::consts::PI
    }
}

// dynamic dispatch is more expensive in runtime than static one. So why do we need it?
// when we don't know types at compile time. For example:
fn why_we_need_dynamic_dispatch()
{
    let shapes:[&Shape; 5] = [&Circle{radius:12.0}, &Square{side:12.0}, &Circle{radius:3.0}, &Square{side:2.0}, &Circle{radius:22.13}];
    for (i, shape) in shapes.iter().enumerate() // enumarate will add me the index of iteration, the "i" variable
        {
            println!("Shape #{} has area {}", i, shape.area())
        }
}


fn ownership_example_1()
{
    let v = vec![1,2,3]; // v is on stack, but the data is on heap
    let v2 = v; // only copies pointer, effectively we would get 2 pointers to the same data. That could introduce race conditions!
//    println!("{:?}", v); // we can't use v anymore, because ownership was passed to v2. We will get error here:  value borrowed here after move
}

fn ownership_example_2()
{
    let v = vec![1,2,3];
    let foo = |v:Vec<i32>| ();
    foo(v);
//    println!("{:?}", v); // we can't use v anymore, because ownership was passed to closure foo.
}

fn ownership_example_3()
{
    let v= 123;
    let w = v; // in this case, v is primitive so value will be copied. No ownership concept applies here
    println!("{:?}", v);  // and we can still use the original v variable after assignment
}

fn ownership_example_4()
{
    let v= Box::new(123);
    let w = v; // if we are reassigning Boxed value, hence the actual data is on heap, ownership applies again
//    println!("{:?}", v);  // won't work
}


fn ownership_example_5()
{
    let v = vec![1,2,3]; // v is on stack, but the data is on heap
    let print_vector = |z:Vec<i32>|
        {
            println!("Vector = {:#?}", z);
        };
//    let vv = print_vector(v);
    print_vector(v); // the ownership of v moves into the print_vector
//    println!("v[0] = {}", v[0]) // therefore this fails at compile time
}



fn ownership_example_5_solved()
{
    let v = vec![1,2,3];
    let print_vector = |z:Vec<i32>| -> Vec<i32>
        {
            println!("Vector = {:#?}", z);
            z
        };
    let vv = print_vector(v); // return object referencing heap vector again
    println!("v[0] = {}", vv[0]) // we can safely use now
    // but this is impractical. Due to this, concept of borrowing exists.
}

fn borrowing()
{
    let print_vector = |z:&Vec<i32>| // changed signature to use reference&, as opposed to ownership_example_5
        {
            println!("Vector = {:#?}", z);
        };
    let v = vec![1,2,3];
    print_vector(&v);
    println!("v[0] = {}", v[0]) // we can still safely use now, because print_vector finished
}



fn borrowing_2()
{
    let mut a = 40;
    let b:&mut i32 = &mut a;
    *b += 2;
    a += 2; // this was not expected to compile, unless the occurence of b variable goes out of scope
    println!("{}", a);
}

struct Person
{
    name: String
}

struct Company<'z>
{
    name: String,
    ceo: &'z Person
}

impl Person
{
    fn get_ref_name(&self) -> &String
//    fn get_ref_name<'a>(&'a self) -> &'a String <---- this is what rust actually does from our function declaration automatically
    {
        &self.name
    }
}

fn lifetimes()
{
    let boss = Person { name: String::from("Elon Musk") };
    let tesla = Company { name: String::from("Tesla"), ceo: &boss};
}


fn lifetimes_not_living_long_enough()
{
    let mut z: &String;
    {
        let p = Person { name: String::from("Peter") };
        z = p.get_ref_name()
    }
//    println!("z = {}", z) // we'll get error refering to p.get_ref_name: borrowed value does not live long enough
    // that's because Person from which we're getting the String reference, therefore neither the String will exist after it goes out of its scope
}


struct RcPerson
{
    name: Rc<String>
}

impl RcPerson
{
    fn new(name: Rc<String>) -> RcPerson
    {
        RcPerson { name: name }
    }
    fn greet(&self)
    {
        println!("hello my name is {}", self.name)
    }
}

use std::rc::Rc; // Rc does not implement SendThread trait, hence it can't be passed to new/different threads

fn rc_demo()
{
    let name = Rc::new("John".to_string()); // using RC (Reference Counting) we can take break from concept of borrowing
    println!("Name = {} has {} strong pointers", name, Rc::strong_count(&name));
    {
        let person = RcPerson::new(name.clone());
        println!("Name = {} has {} strong pointers", name, Rc::strong_count(&name));
        person.greet();
    }
    println!("Name = {}", name);
    println!("Name = {} has {} strong pointers", name, Rc::strong_count(&name));
    // the problem is RC is that it only works in single thread
}

use std::sync::Arc; // Arc stands for Atomic Reference Counting. Arc is thread safe and implements SendThread trait

struct ArcPerson
{
    name: Arc<String>
}

impl ArcPerson
{
    fn new(name: Arc<String>) -> ArcPerson
    {
        ArcPerson { name: name }
    }
    fn greet(&self)
    {
        println!("hello my name is {}", self.name)
    }
}

use std::thread;

fn arc_demo()
{
    let name = Arc::new("John".to_string());
    let person = ArcPerson::new(name.clone());
    let t = thread::spawn(move || {
        person.greet();
    });
    println!("name = {}", name);
    t.join().unwrap()
}

fn take_a_look(foo_ref: &String)
{
    println!("Taking look at string {}", foo_ref)
}

fn my_ref_test()
{
    let mut foo: String = "abcd".to_string();
    println!("foo is {}", foo);
//    take_a_look(&foo);
    let foo_ref: &mut String = &mut foo;
    println!("reference to foo created");
    foo = "ijkl".to_string();
//    println!("foo is {}", foo);
//    println!("foo_ref is {}", foo_ref);
}


extern crate future_by_example;
extern crate tokio_core;

use futures::future::FutureResult;
use futures::Future;

fn future_example_2()
{
    use futures::prelude::*;
    use futures::future;

    let future = future::ok::<u32, u32>(1);
    let new_future = future.map(|x| x + 3);
    assert_eq!(new_future.wait(), Ok(4));
}


#[derive(Debug, PartialEq)]
pub enum ExamplePatrikFutureError {
    Oops,
}

type PatrikTestFuture = FutureResult<usize, ExamplePatrikFutureError>;

fn future_example()
{
    use std::time::{Duration, SystemTime};

    use future_by_example::{new_example_future};

    let timeBeforeWait = SystemTime::now();
    println!("Future by example. Before wait {:?}", timeBeforeWait);
    let future = new_example_future();
    let mapped_future = future.map(|i| i * 3);
    assert_eq!(mapped_future.wait(), Ok(6));
    let timeAfterWait = SystemTime::now();
    println!("Future by example. After wait {:?}", timeAfterWait);
}

fn creating_future()
{
    use futures::Future;
    use futures::future::ok;

    // Here I specify the type of the error as (); otherwise the compiler can't infer it
    let future = ok::<String, ()>(String::from("hello"));
//    let future = futures::future::ok::<_, ()>(123); // the _ in generics is just convenience sting so we don't have to write type, compiler can infer what it's supposed to be
    let result = future.wait().unwrap();
    assert_eq!(String::from("hello"), result);
    println!("custom future result = {}", result);
}

fn future_transformations()
{
    use futures::future::ok;
    use futures::Future;

    let future = ok(5).join(ok(7)).map(|(x, y)| x + y).map(|z| z / 2);
    let result: Result<i32, ()> = future.wait();
    assert_eq!( result, Ok(6) );
    assert_eq!( result.unwrap(), 6 );
    println!("Result is {:?}", result);
}

fn custom_future()
{
    use futures::Future;
    use futures::future::ok;

    fn make_twelve() -> Box<Future<Item=u64, Error=()>> {

        ok(5).join(ok(7)).map(|(x, y)| x + y).boxed()
    }

    let twelve_future = make_twelve();
    let six_future = twelve_future.map(|z| z / 2).wait();
    assert_eq!(six_future, Ok(6));
    println!("made 6 from future {:?}", six_future);
}

fn custom_future_from_closure()
{
    use futures::Future;

    let make_twelve = || {
        use futures::future::ok;

        // We don't need to put our `Future` inside of a `Box` here.
        ok(5).join(ok(7)).map(|(x, y)| x + y)
    };

    let expected: Result<u64, ()> = Ok(6);
    let twelve = make_twelve();
    assert_eq!(twelve.map(|z| z / 2).wait(), expected)
}

fn future_bad_good_sequentially()
{
    use future_by_example::{new_example_future, new_example_future_err, ExampleFutureError};

    let good= new_example_future();
    let bad = new_example_future_err();
    let both = good.and_then(|result| {
        println!("Finished good with value {:?}", result);
        bad
    });
    let expected = Err(ExampleFutureError::Oops);
    let foo = assert_eq!(both.wait(), expected);
}

fn futures_parallel()
{
    use futures::Future;
    use futures::future::ok;
    use future_by_example::new_example_future;
    use future_by_example::new_example_future_err;

    let future1 = new_example_future();
    let future2 = futures::future::ok(10);

    let joined = future1.join(future2);
    let result = joined.wait().unwrap();
    assert_eq!(result, (2, 10));
    println!("joined futures results. result={:?}", result)
}

fn play_with_borrow()
{
    let mut x = 5;

    let clo = || {
        x
    };
    // folowing is okay because we actually are getting copy of i32. i32 implements copy trait. So y and x are pointin to different memory
    let mut y = clo();
    y += 1;
    println!("x = {}", x);
    println!("y = {}", y);

    // here we are making sure the v is not moved into the inner scope!
    let mut v = vec![1,2,3];
    {
        let vclo = || {
            &v
        };
        let w = vclo();
        println!("w = {:?}", w);
    }
    println!("v = {:?}", v);


    // does not live long enough example
//    let a: &i32;
//    {
//        let b = 5;
//        a = &b;
//    }
//
//    println!("{}", a);

    let q:&i32;
    let s: i32 = 1000;
    let r: i32 = 2000;
    q = &s; // q being immutable doesn't mean it have to be initialized straight away.
//    q = &r; // but can no longer be changed after first assignment, as it is immutable
}


fn play_with_borrow2()
{
    let y: &i32;
    let x = 5;
    y = &x;

    println!("{}", y);


    // dangling pointer
//    let r;              // Introduce reference: `r`.
//    {
//        let i = 1;      // Introduce scoped value: `i`.
//        r = &i;         // Store reference of `i` in `r`.
//    }                   // `i` goes out of scope and is dropped.
//
//    println!("{}", r);  // `r` still refers to `i`.
}

fn x_or_y<'a, 'b>(x: &'a i32, y: &'b i32) -> &'b i32 {
    &32
}

fn play_with_lifetime()
{
//    let x= &10;
//    let y= &20;
//    let r = x_or_y(x, y);
//    println!("Result is {:}", *r)
    let x= &10;
    let r:&i32;
    {
        let y = &20;
        r = x_or_y(x, y);
    }
    println!("Result is {:}", *r)
}


fn guessing_game()
{
    extern crate rand;
    use rand::prelude::*;
    use std::cmp::Ordering;
    use std::io::{stdin};

    let mut rng = thread_rng();
    let x: i32 = rng.gen();
    let x = x % 6;
    let x = if (x>=0) {x} else {-x};

    loop {
        println!("Guess number. Type 'q' to quit the game.");
        let mut input = String::new();
        input = match stdin().read_line(&mut input) {
            Ok(data) => input.trim().to_string(),
            Err(_) => panic!("Couldn't process input")
        };
        if input == "q" {
            println!("Qutting the game. The answer was {}", x);
            break;
        }
        let guess: i32 = match input.parse() {
            Ok(value) => value,
            Err(_) => {
                println!("You need to enter a number");
                continue
            }
        };
        println!("You guessed number: {}", guess);

        match x.cmp(&guess) {
            Ordering::Greater => println!("Go higher!"),
            Ordering::Less => println!("Go lower!"),
            Ordering::Equal => {
                println!("Bingo! Congrats, random number was {}", x);
                break
            }
        }

    }
}


fn main() {
//    global_variables();
//    datatypes();
//    operators();
//    scope_and_shadowing();
//    sh::stack_and_heap()
//    if_statement();
//    while_loop();
//    for_loop();
//    match_statement();
//    structures();
//    enums(); // colors
//    unions();
//    option();
//    array();
//    vector();
//    slice();
//    mySliceTest()
//    strings();
//    tuple();
//    pattern_matching()
//    generics()

//    ------- section 5
//    functions();
//    methods();
//    closures();
//    high_order_functions();

//    ------- section 6
//    traits();
//    into()
//      droptest()

//    operator_overloading_1();
//    operator_overloading();
//      my_ref_test()
      //static_dispatch(); // dispatching is basically how compiler figures out which function to call
//    dynamic_dispatch();
//    why_we_need_dynamic_dispatch()

//    ------- section 7, ownership
//    ownership_example_1();
//    ownership_example_2();
//    ownership_example_3();
//    ownership_example_4();
//    ownership_example_5();
//    ownership_example_5_solved();

//    borrowing();
//    borrowing_2();
//    lifetimes();
//    lifetimes_not_living_long_enough();
//    rc_demo();
//    arc_demo();

//    future_example();
//    future_bad_good_sequentially();
//    futures_parallel();
//    creating_future();
//    future_transformations();
//    custom_future()
//    custom_future_from_closure()
//    play_with_borrow()
//    play_with_borrow2()
//    play_with_lifetime()
//    guessing_game()
//    circle::play_with_traits()
//    square::play_with_generic_struct()
//    pwclosures::play_with_closures();
//    pwcopy::play_with_copy();
//    pwhyper::play_with_hyper()
//    pwreqwest::play_with_reqwest()
//    pwaliasing::play_with_aliasing()
//    pwdestructuring::play_with_destructuring();
//    pwpatrik::patrik_plays();
//    borrowselfissue::run()
//    futures_in_ifs::run();
//    so1::run();
//    channels::run();
//    str_compare::run();
//    pw_closures::run();
//    pw_borrow_move::run();
//    pw_pluralize::run();
//    pw_slice_args::run();
//    pw_slice_strings::run();
//    rim_2_7_borrow_patterns::run();
//    rim_2_7_entryapi::run()
//    rim_2_7_structs::run();
//    rim_2_7_structs_refactored::run();
//    rim_2_7_tempvar::run();
//    rim_2_8_socket::run();
//    rim_3_5_multiple_errors::run();
//    rim_3_6_custom_errors::run();
//    rim_3_7_quick_error::run();
//    rim_3_7_error_chain::run();
//    rim_3_7_failures::run();
//    stackcheap::run();
//    pw_move_address::run();
//    rim_4_4_stemmer::run();
//    pw_copy_trait::run();
//    pw_clone_options::run();
//    futures_either::run();
//    futures_in_parallel::run();
//    future_custom_implementation::run();
//    devto_future_custom_implementation2::run();
//    devto_streams::run();
//    futures_to_streams::run();
//    snoyman_iterators::run();
//    snoyman_3::run();
//    snoyman_closures::run();
//    snoyman_streams_exercise6::run();
//    snoyman_streams_exercise7::run();
//    static_mut::run();
//    static_mut_so_question::run();
//    static_mut_so_question_refreturn::run();
//    static_mut_so_question_simplified::run();
//    static_mut_so_question_simplified::run();
//    static_mut_unsafe::run();
//    static_lazy_hashmap::run();
//    ref_keyword::run();
//    ref_keyword_2::run();
//    ref_keyword_matching::run();
//    static_lazy_vector::run();
    static_mut_so_question_simplified_lazy_static::run();
//    concurrency_arc::run();
//    concurrency_arc_mutex::run();
//    concurrency_rw_lock::run();
//    concurrency_rw_lock::run();
//    concurrency_cell::run();
//    concurrency_refcell::run();
//    concurrency_cell_usecase::run();
//    concurrency_cell_mine::run();
//    static_lazy_with_mutex::run();
//    concurrency_pw_threads::run();
//    concurrency_thread_local::run();
//    concurrency_thread_local_with_rc::run();
//    concurrency_thread_local_with_arc::run();
//    concurrency_thread_local_with_rwlock::run();
//    global_mutable::run();
//    global_mutable_integer::run();
//    concurrency_racing_1::run();
//    concurrency_racing_2::run();
//    concurrency_send_sync_issue::run();
//    concurrency_factory_creatin_send_sync::run();
//    tokio_example_resolveconnect::run();
}
