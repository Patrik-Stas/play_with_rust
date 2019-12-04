
// does not allow for Copy - pairwise copy! If we add that, instead of moves, bitwise copies will be
// created. So even after "moving" a
#[derive(Debug,Clone,Copy)]
struct Position {
    x: i32,
    y: i32
}

//#[derive(Debug,Clone)]
//struct Position {
//    x: i32,
//    y: i32
//}



fn move_out(p: Position) {
    println!("Moved out position = {:?}", p);
}


fn if_copy_was_implemented() {
    let p = Position {x:100, y:1000};
    // when Copy is implemented, pairwise copy from P to X happens and both X and P can be used
    // When Copy is not implemented. "move" will occur and P cannot be used anymore
    // Note that actually in both cases bitwise copy might occur, even when "moving" (see pw_momve_address.rs for demonstration)
    // but the difference is that in case of moving, there will be only 1 "owner" of underlying data responsible for cleanup (some data on heap, sockets, files)
    // If for example socket or String implemented Copy, we would have multiple references point to same resource and double free could easily happen
    let x = p;
    // next line can be uncommented only if Copy is implemented
     println!("After assignment x=p, p={:?}", p)
}

pub fn run() {
    if_copy_was_implemented();
}