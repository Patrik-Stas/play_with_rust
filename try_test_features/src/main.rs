pub struct Person {
    pub name: String,
    pub age: u32,
}

pub fn get_person_age(p: Person) -> u32 {
    return p.age
}

pub fn substract(a: u32, b: u32) -> u32 {
    a - b
}

pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

fn main() {
    println!("Hello, world!");
    println!("1+2={}", add(1, 2));
    let p = Person { name: "john".into(), age:23 };
}

#[cfg(test)]
mod tests {
    use crate::{add, substract};
    #[cfg(feature = "PPP")]
    use crate::{Person};

    #[test]
    fn test_no_feature() {
        assert_eq!(1, 1);
    }

    #[cfg(feature = "AAA")]
    #[test]
    fn test_feature_AAA() {
        assert_eq!(1, 1);
    }

    #[test]
    #[cfg(feature = "BBB")]
    fn test_feature_BBB() {
        assert_eq!(1, 1);
    }

    #[test]
    #[cfg(feature = "AAA")]
    #[cfg(feature = "BBB")]
    fn test_feature_AAA_BBB() {
        assert_eq!(1, 1);
    }

    #[test]
    #[cfg(feature = "PPP")]
    fn test_using_PPP_feature() {
        let p = Person { name: "john".into(), age:23 };
        assert_eq!(1, 1);
    }

    #[test]
    #[cfg(feature = "PPP_extra")]
    fn test_PPP_extra_feature() {
        let p = Person { name: "john".into(), age:23 };
        assert_eq!(1, 1);
    }
}
