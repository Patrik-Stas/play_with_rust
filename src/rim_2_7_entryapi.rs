use std::collections::HashMap;

pub fn run()
{
    let text = "hello world world this world is hello text hehe what is this";

    let mut freqs = HashMap::new();

//    for word in text.split_ascii_whitespace() {
//        match freqs.get_mut(word) {
//            Some(value) => *value += 1,
//            None => {
//                freqs.insert(word, 1);
                  // this would not work in older Rust versions, because before NLL
                  // because it didn't distinquish that the "value" was only used in some arms of the match
                  // .. But of course in None arm, value is not actually used.
//            },
//        }
//    }
    for word in text.split_ascii_whitespace() {
        *freqs.entry(word).or_insert(0) += 1
    }

    println!("Word frequencies: {:#?}", freqs);
}