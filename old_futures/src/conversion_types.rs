use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::{io, fmt, result};

#[derive(Debug)]
pub struct Comment(String);
#[derive(Debug)]
pub struct Article(String);
#[derive(Debug)]
pub struct Information(String);

// try comment this out to see where is From conversion needed
// impl From<io::Error> for DocumentServiceError
// {
//     fn from(other: io::Error) -> Self {
//         DocumentServiceError::Io(other)
//     }
// }

impl From<Information> for Article
{
    fn from(information: Information) -> Self {
        Article(information.0)
    }
}

impl From<Comment> for Information
{
    fn from(comment: Comment) -> Self {
        Information(comment.0)
    }
}

fn print_information(information: Information) {
    println!("Information = {:?}", information);
}


fn print_article(article: Article) {
    println!("Article = {:?}", article);
}

pub fn run()
{
    let comment = Comment("<3 what a cute doggo".into());
    let comment2 = Comment("very cute".into());
    let comment3 = Comment("veryyy cute".into());
    let comment4 = Comment("hellooo".into());
    println!("Comment = {:?}", comment);
    let article = Article(String::from("This is article about dogs."));
    println!("Article = {:?}", article);

    print_article(Article::from(Information::from(comment))); // here ware using the 'from' methods we've implemented

    // print_article(comment2.into()); // transitive conversion is not figured out, this will look for impl From<Comment> for Article
    print_article(Information::from(comment2).into()); // this does work

    // "implementing From automatically provides one with an implementation of Into thanks to the blanket implementation in the standard library."
    // https://doc.rust-lang.org/std/convert/trait.From.html
    let information: Information = comment3.into();
    print_information(information);

    print_information(comment4.into());
}
