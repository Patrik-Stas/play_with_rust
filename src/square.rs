struct Foo {
    bar: i32
}

struct Rectangle<T> {
    x: T,
    y: T,
    width: T,
    height: T,
}

impl<T: PartialEq> Rectangle<T> {
    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

pub fn play_with_generic_struct() {
    let mut r = Rectangle {
        x: 0,
        y: 0,
        width: 47,
        height: 47,
    };

    // no method named `is_square` found for type `square::Rectangle<square::Foo>` in the current scope
    // that's because Foo doesn't fit the generic constraint "T: PartialEq"
//    let mut r = Rectangle {
//        x: Foo {bar: 1},
//        y: Foo {bar: 2},
//        width: Foo {bar: 3},
//        height: Foo {bar: 4},
//    };

    assert!(r.is_square());

    r.height = 10;
//    r.height = Foo {bar: 10};
    assert!(!r.is_square());
}