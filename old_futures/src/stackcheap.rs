fn stack_reference()
{
    let mut x = 5;
    {
        let mut y = &mut x;
        *y += 1;
        println!("stackreference: y is: {}", y);
    }

    println!("stackreference: x is: {}", x);
}

fn copyTraitValues() {
    let mut x = 5;
    let mut y = x;

    y += 1;

    println!("copyTraitValues: x is: {}", x);
    println!("copyTraitValues: y is: {}", y);
}

pub fn run() {
    copyTraitValues();
    stack_reference();
}