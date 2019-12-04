pub fn take_by_value(s: String)
{
    println!("{:?}", s)
}

pub fn take_by_ref(s: &str)
{
    println!("{:?}", s)
}


pub fn run()
{
    let mut scores = vec![1, 2, 3];
    let score = &scores[0];
    scores.push(4);
//    println!("score {:?}", score) // will error here
}