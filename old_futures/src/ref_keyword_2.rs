#[derive(Clone, Copy)]
struct Point { x: i32, y: i32 }

pub fn run() {
    let y = 11111;
    let z = 22222;
    let mut w = 22222;
    let mut ymut = 33333;
    let mut zmut = 44444;
    let mut mutable_pointer = &y;
    let ref mut pointer_to_mutable = ymut; // this
    let pointer_to_mutable_2 = &mut zmut; // and this means exactly the same

//    *mutable_pointer = 1231; // cant change VALUE, we can only change where the thing points, not the values at those addresses

    println!("mutable_pointer:: Value= {} {:p}", mutable_pointer, mutable_pointer);
    mutable_pointer = &z;
    println!("mutable_pointer:: Value= {} {:p}", mutable_pointer, mutable_pointer);

//    pointer_to_mutable = &z; // cant change POINTER, we can only change value of this specific address we initialized "pointer_to_mutable" with

    println!("pointer_to_mutable:: Value= {} {:p}", pointer_to_mutable, pointer_to_mutable);
    *pointer_to_mutable = z;
    println!("pointer_to_mutable:: Value= {} {:p}", pointer_to_mutable, pointer_to_mutable);


    println!("pointer_to_mutable:: Value= {} {:p}", pointer_to_mutable_2, pointer_to_mutable_2);
    *pointer_to_mutable_2 = z;
    println!("pointer_to_mutable:: Value= {} {:p}", pointer_to_mutable_2, pointer_to_mutable_2);
}
