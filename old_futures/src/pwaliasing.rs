type CoolNumber = i32;

fn take_i32(x:i32) {
    println!("received i32: {:?}", x);
}


fn take_cool_number(x:CoolNumber) {
    println!("received cool number {:?}", x);
}

pub fn play_with_aliasing()
{
    let x:i32 = 123;
    let y:CoolNumber = 876;
    take_i32(x);
    take_i32(y);
    let z:usize = 12;
//    the error will report i32, not CoolNumber:
//    "expected i32, found usize"
//    take_cool_number( z);
}