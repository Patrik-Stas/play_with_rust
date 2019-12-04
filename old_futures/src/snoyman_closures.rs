fn say_message_2 (msg: &str)  {
    println!("{}", msg);
}

fn call_with_hi<F>(f: F)
    where F: Fn(&str)
{
    f("Hi!");
}

fn greeting()
{
    let name = String::from("Alice");
    let say_something = |msg: &str| println!("{}, {}", msg, name);
    call_with_hi(say_something);
    call_with_hi(say_something);
    call_with_hi(say_message_2);
}


fn counters()
{
    let mut count = 0;
    {
        let visit = || {
            count += 1;
            println!("You are visitor #{}", count);
        };

        call_five_times(visit);
        call_five_times(|| println!("Hello World!"));
    }
    println!("We still own the count variable! count = '{}'.", count);
    count += 10;
    println!("And after we added 10 it's '{}'.", count);
}

fn with_closure_move()
{
    let name1 = String::from("Alice"); // using string for demonstration to avoid copying (Doesn't implement Copyable)

    let welcome = || {
        let mut name2 = name1; // moving in name1 and making it mutable in context of closure!
        name2 += " and Bob";
        println!("Welcome, {}", name2);
    };

    // test1 ---------
//    call_five_times(welcome) // this will no work: ^^ this closure implements `FnOnce`, not `FnMut`
    // that's because the closure is moving in value, which means it can only called be once (you can't move in a value again once it was already moved in)
    // ------------------

    // test2 ---
//    welcome();
//    println!("name1= {}", &name1); // after calling welcome, we cant do this. We can't borrow name1, because it was moved into the closure by calling it.
    // ------------------

    // test3 ---
    call_once(welcome) // alternative to test2
    // ------------------
}

fn call_once<F>(mut f: F)
    where
        F: FnOnce(),
{
    f();
}

fn call_five_times<F>(mut f: F)
    where
        F: FnMut(),
{
    for _ in 1..6 {
        f();
    }
}

pub fn run() {
//    greeting();
//    counters();
    with_closure_move();
}

