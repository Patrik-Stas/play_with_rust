use std::fmt::{Display, Formatter};
use std::mem;

pub fn take_by_value_copyable<T: Clone>(s: T) -> T
{
    println!("{:p} Copyable Take by value", &s);
    s
}

pub fn take_by_ref_copyable<T: Clone>(s: &T)
{
    println!("{:p} Copyable Take by ref", s)
}

pub fn take_by_value_vector<T>(s: Vec<T>) -> Vec<T>
{
    println!("{:p} Vector Take by value", &s);
    println!("{:p} &Vector[0] Take by value", &(s[0]));
    s
}

pub fn take_by_ref_vector<T>(s: &Vec<T>)
{
    println!("{:p} Vector Take by ref", s);
    println!("{:p} &Vector[0] Take by ref", &(s[0]));
}
pub fn run()
{
    println!("{} Size of i32", mem::size_of::<i32>());

    let x = 1;
    println!("{:p} X starts", &x);
    let y = x;
    println!("{:p} Y (X moved there)", &y);
    take_by_ref_copyable(&y);
    println!("{:p} Y after was taken by ref", &y);
    let z = take_by_value_copyable(y);
    println!("{:p} Z (Y moved there)", &z);

    println!("------ scores -------");

    let mut scores_x = vec![1, 2, 3];
    println!("{:p} X starts", &scores_x);
    println!("{:p} &X[0] starts", &(scores_x[0]));
    let scores_y = scores_x;
    println!("{:p} Y (X moved there)", &scores_y);
    println!("{:p} &Y[0] starts", &(scores_y[0]));
    take_by_ref_vector(&scores_y);
    println!("{:p} Y after was taken by ref", &scores_y);
    let scores_z = take_by_value_vector(scores_y);
    println!("{:p} Z (Y moved there)", &scores_z);
    println!("{:p} &Z[0] starts", &(scores_z[0]));
}