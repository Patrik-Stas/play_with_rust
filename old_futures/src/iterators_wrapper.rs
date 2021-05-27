// The structure that holds the wrapped iterator and the current state.
struct CircularEnumerate<I> {
    iter: I,
    items: u32,
    cur: u32,
}

// Implement the iterator.
impl<I> Iterator for CircularEnumerate<I> where I: Iterator {
    type Item = (u32, I::Item);

    fn next(&mut self) -> Option<(u32, I::Item)> {
        match self.iter.next() {
            // If the wrapped iterator is done, we're done.
            None => None,

            // If the wrapped iterator is not done, tack on
            // the circular enumeration.
            Some(v) => {
                let idx = self.cur;
                // Increment the current value, wrapping it
                // if necessary.
                self.cur = (self.cur + 1) % self.items;
                Some((idx, v))
            }
        }
    }
}

// For now, we need a way to create a new CircularEnumerator.
impl<I> CircularEnumerate<I> {
    fn new(iter: I, items: u32) -> CircularEnumerate<I> {
        CircularEnumerate { iter: iter, items: items, cur: 0 }
    }
}

fn run() {
    for (idx, i) in CircularEnumerate::new("abcdefghijkl".chars(), 3) {
        println!("{}: {}", idx, i);
    }
}