//#[derive(Debug,Clone,Copy)]
//struct PositionOnlyCloneable {
//    x: i32,
//    y: i32
//}


#[derive(Debug, Clone, Copy)]
struct PositionCopyable {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone)]
struct PositionCloneable {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
struct PositionNonCloneable {
    pub x: i32,
    pub y: i32,
}

fn cloneSomeCloneablePosition(m: &Option<PositionCloneable>) -> Option<PositionCloneable> {
    let cloned = m.clone();
    println!("Cloned Some position === {:?}", cloned);
    cloned
}

fn cloneSomeNonCloneablePosition(m: Option<PositionNonCloneable>) {
//    let cloned = m.clone();
//    println!("Cloned Some position === {:?}", cloned);
}

fn test_with_cloneable() {
    let mut p1 = Some(PositionCloneable { x: 12, y: 43 });
    let mut p2 = None;

    let p1clone = cloneSomeCloneablePosition(&p1);
    let p2clone = cloneSomeCloneablePosition(&p2);

    if let Some(p) = p1.as_mut() {
        p.x = 123123;
    }

    println!("Original modified position = {:?}", p1);
    println!("p1clone position = {:?}", p1clone);
    println!("p2clone position = {:?}", p2clone);
}

fn test_with_non_cloneable() {
    let mut p1 = Some(PositionNonCloneable { x: 12, y: 43 });
//    let mut p2 = None;

//    let mut n1 = PositionNonCloneable { x: 999, y: 222 };
//    let heh = n1.clone();
    //--------------------------- method `clone` not found for this


    // following line will error at compile time:
//    let p1clone = p1.clone();
    // note: the method `clone` exists but the following trait bounds were not satisfied:
    // comment: If you look at Clone implementation for Option<T>, you'll find:
    // impl<T: Clone> Option<&T> {
    // hence a type T can be Option, but such instance can use Option<T>.clone() only as far as
    // the T itself implements clone! And if so, when Option is cloned, it creates new options with
    // cloned inner content as well.

//    cloneSomeNonCloneablePosition(p1);
    // no method named `clone` found for type `std::option::Option<pw_clone_options::PositionNonCloneable>` in the current scope
//    cloneSomeNonCloneablePosition(p2);
//
//    if let Some(p) = p1.as_mut() {
//        p.x = 123123;
//    }
//
//    println!("Original modified position = {:?}", p1);
//    println!("p1clone position = {:?}", p1clone);
//    println!("p2clone position = {:?}", p2clone);
}


pub fn run() {
    test_with_cloneable();
    test_with_non_cloneable();
}