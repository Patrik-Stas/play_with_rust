pub(crate) fn run() {
    let a = [1,2,3];
    let v = vec![4,5,6];

    take_only_array_of_3(&a);
    // take_only_array_of_3(&v[..]);
    take_slice(&a[..]);
    take_slice(&v[..]);
}

fn take_only_array_of_3(param: &[i32; 3]) {
    println!("This si array {:?}", param);
}

fn take_slice(param: &[i32]) {
    println!("This is slice {:?}", param);
}


