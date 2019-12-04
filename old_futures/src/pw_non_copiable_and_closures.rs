//#[derive(Debug,Clone,Copy)]
//struct PositionOnlyCloneable {
//    x: i32,
//    y: i32
//}

#[derive(Debug,Clone)]
struct Position {
    x: i32,
    y: i32
}



fn move_away(p: Position) {
    println!("Moved away position = {:?}", p);
}

fn borrow(p: &Position) { println!("Borrowed position = {:?}", p); }

fn closure_test() {
    let p1 = Position {x:12, y:43};
    {
        let closure1 = || {
//        let p1c1 = p1.clone();
            println!("In closure1, p1 before move_out = {:?}", p1.clone());
            move_away(p1.clone());
        };
        closure1();
    }

    let closure2 =  || {
//        let p1c2 = p1.clone();
//        println!("In closure2, p1c = {:p} {:?}", &p1c2, p1c2);
//        borrow(&p1)
        move_away(p1)
    };

//    println!("before calling closures ... x = {}, y = {}", x, y);

    closure2();
//    println!("after calling closures ... x = {}, y = {}", x, y);
}

// https://stackoverflow.com/questions/54287719/how-to-copy-instead-of-borrow-an-i64-into-a-closure-in-rust
fn value_cant_escape_fnmut_closure()
{
// this won't compile, the other version own i, other values cant take them away
//    let mut i = 0;
//    let mut closure = || {
//        i = 2;
//        || {
//            println!("i = {}", i);
//        }
//    };
//    closure()();
}

pub fn run() {
    closure_test();
//    value_cant_escape_fnmut_closure()
}