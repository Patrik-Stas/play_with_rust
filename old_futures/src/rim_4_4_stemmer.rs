struct Stemmer {
    suffix: String
}


impl Stemmer {
    pub fn new(suffix: String) -> Stemmer {
        Stemmer { suffix }
    }

    pub fn simple_stem(word: &str) -> &str {
        println!("simple stemming word = {:?}", word);
        word
    }

    pub fn stem<'a>(&self, word: &'a str) -> &'a str {
//        println!("stemming word = {:?}", word);
//        println!("word's last two letters {:?}", word[word.len()-2..]);
//        println!("word's last two letters {:?}", &word[word.len()-2..]);
//        println!("stemming word, length = {}", word.len());
//        println!("stemming word, ends with {:?} {:?}", &self.suffix, word.ends_with(&self.suffix));
        if (word.ends_with(&self.suffix)) {
            let index = word
                .rfind(&self.suffix)
                .expect("found because ends_with returned true");
            &word[0..index]
        } else {
            word
        }
    }
}

pub fn run()
{
    let word = String::from("Credited");
    let stemmer = Stemmer::new("ed".to_owned());
    let stemmed_simple = Stemmer::simple_stem(&word);
    let stemmed = stemmer.stem(&word);
    println!("stemmed_simple result = {:?}", stemmed_simple);
    println!("stemmed result = {:?}", stemmed);
}
