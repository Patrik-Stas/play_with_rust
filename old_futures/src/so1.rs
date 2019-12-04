trait Bar<T> {}
//
//struct Foo<T> {
//    data: Vec<Box<Bar<T>>>,
//}
//
//impl<T> Foo<T> {
//    fn add<U: Bar<T> + 'static>(&mut self, x: U) {
//        self.data.push(Box::new(x));
//    }
//}
//
//impl<T> Bar<T> for &str {}
//
//pub fn run() {
//    let mut foo: Foo<&str> = Foo { data: vec![] };
//
//    // this will not work!
//    {
//        let s = "oh no".to_string();
//        foo.add(s.as_ref());
//    }
//    // this will woork
////    foo.add("oh no")
//}

struct Foo<'a, T> {
    data: Vec<Box<dyn Bar<T> + 'a>>,
}

impl<'a, T> Foo<'a, T> {
    fn add<U>(&mut self, x: U)
        where
            U: Bar<T> + 'a,
    {
        self.data.push(Box::new(x));
    }
}

impl<T> Bar<T> for &String {}

pub fn run() {
    let mut foo: Foo<&str> = Foo { data: vec![] };

    let s = "oh no".to_string();
//    foo.add(&s);
    // this will woork
//    foo.add("oh no")
}