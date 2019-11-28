//
//extern crate reqwest;
//use std::collections::HashMap;
//
//fn do_reqwest() -> Result<(), Box<std::error::Error>> {
//    let resp: HashMap<String, String> = reqwest::get("https://httpbin.org/ip")?
//        .json()?;
//    println!("{:#?}", resp);
//    Ok(())
//}
//
//
//pub fn play_with_reqwest()
//{
////    let client = reqwest::Client::new();
////    let res = client.post("http://httpbin.org/post")
////        .body("the exact body that is sent")
////        .send()?;
//    do_reqwest()
//
//}