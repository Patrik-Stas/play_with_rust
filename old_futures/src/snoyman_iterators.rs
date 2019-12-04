struct Empty;

impl Iterator for Empty {
    type Item = u32;
    //    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
    fn next(&mut self) -> Option<Self::Item> { // "as Iterator" not necessary here
        None
    }
}

struct TheAnswer;

impl Iterator for TheAnswer {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        Some(42)
    }
}

struct ToTen(u32);

impl ToTen {
    fn new(start: u32) -> ToTen {
        ToTen(start)
    }
}

impl Iterator for ToTen {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            n if n < 10 => {
                self.0 += 1;
                Some(n)
            }
            _ => None
        }
    }
}

struct Doubler<I> {
    iter: I,
}

//impl<I : Iterator<Item=u32>> Iterator for Doubler<I> {
impl<I> Iterator for Doubler<I>
    where
        I: Iterator,
        I::Item: std::ops::Mul<Output=I::Item> + From<u8>,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            None => None,
            Some(x) => Some(x * From::from(2u8)),
        }
    }
}

fn iter_sum<I>(i:I) -> I::Item
    where
        I: Iterator,
        I::Item : std::ops::Add<Output=I::Item> + From<u8>
{
    i.fold(From::from(0u8), std::ops::Add::add)
}

fn fold_results() {
    let folded: u32= (1..112).fold(0, |acc, x| acc + x);
    println!("{:?}", folded);
    let summed_nums = std::ops::Add::add(1,2);
    println!("1+2 = {:?}", summed_nums);
    let folded2: u32= (1..112).fold(0, std::ops::Add::add);
    println!("{:?}", folded2);
    let folded3: u32= iter_sum(1..112);
    println!("{:?}", folded3);
}

fn collect_results() {
    let my_vec: Vec<u32> = (1..11).collect();
    println!("{:?}", my_vec);
}

fn try_doubler_idiomatic() {
//    for i in (1..11).map(|x| x * 2) {
//        println!("{}", i);
//    }
    for i in (1..11).skip(3).map(|x| x + 1).filter(|x| x % 2 == 0) {
        println!("{}", i);
    }
}

fn try_doubler() {
//    let orig_iter = 1..11; //
    let orig_iter = 1..11u64; //
    let doubled_iter = Doubler {
        iter: orig_iter,
    };
    for i in doubled_iter {
        println!("{}", i);
    }
}

fn do_one_to_ten() {
    for i in ToTen::new(4) {
        println!("i = {}", i);
    };
}


fn endless_iterator() {
    for i in TheAnswer.take(10) {
        println!("The answer to life, the universe, and everything is {}", i);
    }
    println!("All done!");
}

fn empty_iterator() {
    for i in Empty {
        panic!("Wait, this shouldn't happen!");
    }
    println!("All done!");
}

fn loop_iterator() {
    let mut range = 0..10;

    loop {
        match range.next() {
            Some(x) => {
                println!("{}", x);
            }
            None => {
                println!("Got none");
                break;
            }
        }
    }
}

pub fn run() {
//    loop_iterator();
//    empty_iterator();
//    endless_iterator();
//    do_one_to_ten();
//    try_doubler();
//    try_doubler_idiomatic();
//    collect_results();
    fold_results()
}