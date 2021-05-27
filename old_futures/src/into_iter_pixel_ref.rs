// source of this example:
// https://stackoverflow.com/questions/30218886/how-to-implement-iterator-and-intoiterator-for-a-simple-struct

struct Pixel {
    r: i8,
    g: i8,
    b: i8,
}

impl<'a> IntoIterator for &'a Pixel {
    type Item = i8;
    type IntoIter = PixelIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        PixelIterator {
            pixel: self,
            index: 0,
        }
    }
}

struct PixelIterator<'a> {
    pixel: &'a Pixel,
    index: usize,
}

impl<'a> Iterator for PixelIterator<'a> {
    type Item = i8;
    fn next(&mut self) -> Option<i8> {
        let result = match self.index {
            0 => self.pixel.r,
            1 => self.pixel.g,
            2 => self.pixel.b,
            _ => return None,
        };
        self.index += 1;
        Some(result)
    }
}

pub fn run() {
    let p = Pixel {
        r: 54,
        g: 23,
        b: 74,
    };
    println!("Iterating pixel using reference based iterator");
    // for component in p.into_iter() {
    //     println!("{}", component);
    // }
    for component in &p {
        println!("{}", component);
    }
}