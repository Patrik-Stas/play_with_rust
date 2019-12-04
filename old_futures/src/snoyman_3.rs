fn double(x: &mut u32) {
    *x *= 2;
}

fn exercise1() {
    let mut x = 5;
    double(&mut x);
    println!("{}", x);
}

fn iterators_1()
{
    for i in 1..3 {
        let nums = vec![1, 2, 3, 4, 5]; // we are reconstructin this on every iteration!
        for j in nums {
            println!("{},{}", i, j);
        }
    }
}

fn iterators2()
{
    let nums = vec![1, 2, 3, 4, 5];
    for i in 1..3 {
        for j in &nums { // borrows here the array to avoid recreating it
//            let _: u32 = *j;
//            let _: u32 = j;
//            let _: &u32 = j;

            println!("{},{}", i, j);
        }
    }
}

fn iterators_mut_ref() {
    let mut nums = vec![1, 2, 3, 4, 5];
    for i in 1..3 {
//        for j in &mut nums {
        for j in nums.iter_mut() {
            let _: &mut u32 = j;
            println!("{},{}", i, j);
            *j *= 2;
        }
    }
}

fn move_out(val: i32) {
    println!("value moved out = {}", val)
}

fn with_into_iter_to_get_ownership()
{
    let nums = vec![1, 2, 3, 4, 5];
        for j in nums.into_iter() { // IntoIterator trait
        move_out(j);
    }
}

fn with_enumerate()
{
    let nums = vec![6, 5, 4, 3, 2, 1, 2, 3, 4, 5];
    for (idx, j) in nums.into_iter().enumerate().fuse() { // IntoIterator trait
        println!("idx {}, val {}", idx, j);
    }
}


pub fn run() {
//    exercise1();
//    iterators2();
//    iterators_mut_ref();
//    with_into_iter_to_get_ownership();
    with_enumerate();
}