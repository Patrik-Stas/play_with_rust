struct Empty;

struct PatrikList {
    main: Vec<u32>
}

struct DivisableIterator {
    // data: Box<[u32]>, // how to declare slice
    data: Vector<u32>, // how to declare slice
    position: usize,
    divisable_by: u32,
}

impl Iterator for DivisableIterator {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        while self.position < self.data.len() {
            let val = self.data[self.position];
            if val % self.divisable_by == 0 {
                self.position = self.position + 1;
                return Some(val);
            }
            self.position = self.position + 1;
        }
        None
    }
}

impl PatrikList {
    fn getDivisableIterator(&self, divisable_by: u32) -> DivisableIterator {
        DivisableIterator { divisable_by, data: &self.main, position: 0 }
    }
}

impl IntoIterator for PatrikList {
    type Item = u32;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.main.into_iter()
    }
}

pub fn run() {
    let m = PatrikList { main: vec!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20) };
    for item in m.getDivisableIterator(4) {
        println!("Item = {}", item);
    }
}