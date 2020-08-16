use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::{io, fmt, result};
use crate::rim_3_6_custom_errors::DocumentServiceError::RateLimitExceeded;

const MAX_DOCS_CREATED_PER_MINUTE: u8 = 30;

fn num_documents_created_in_past_minute() -> u8 {
    2
}

#[derive(Debug)]
pub enum DocumentServiceError {
    RateLimitExceeded,
    Io(io::Error),
}

impl Error for DocumentServiceError {
    fn description(&self) -> &str {
        use DocumentServiceError::*;
        match *self {
            RateLimitExceeded => "rate limit exceeded",
            Io(_) => "I/O error",
        }
    }
}

impl fmt::Display for DocumentServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use DocumentServiceError::*;
        match *self {
            RateLimitExceeded => write!(
                f,
                "You have exceeded the allowed number of documents per minute."
            ),
            Io(ref io) => write!(f, "I/O error: {}", io),
        }
    }
}

// try comment this out to see where is From conversion needed
impl From<io::Error> for DocumentServiceError
{
    fn from(other: io::Error) -> Self {
        DocumentServiceError::Io(other)
    }
}

// pub fn create_document(filename: &str) -> Result<File, Box<dyn Error>> // before we implemented custom error
pub fn create_document(filename: &str) -> Result<File, DocumentServiceError> // before we created alias on Result using our error type
// pub fn create_document(filename: &str) -> Result<File>
{
    if num_documents_created_in_past_minute() > MAX_DOCS_CREATED_PER_MINUTE {
       // return Err("Create file limit exceeded.").into()
       // return Err("Create file limit exceeded.".into())
        return Err(DocumentServiceError::RateLimitExceeded)
        // return Err(DocumentServiceError::RateLimitExceeded)
    }

    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(filename)?; // the question mark also handles error mapping, without ? we would have to use result.map_err() as shown below
    Ok(file)


    // let newfile_res = OpenOptions::new()
    //     .write(true)
    //     .create_new(true)
    //     .open(filename);
    // newfile_res
    //     .map_err(|err| DocumentServiceError::from(err))
}

// pub type Result<T> = result::Result<T, DocumentServiceError>;
// Alias so we don't have to specify our error type on each of our method

// pub fn update_document(filename: &str) -> Result<File>
// {
//     unimplemented!()
// }
//
// pub fn delete_document(filename: &str) -> Result<()>
// {
//     unimplemented!()
// }
//
// pub fn read_document(filename: &str) -> Result<File>
// {
//     unimplemented!()
// }

pub fn run()
{
    let filename = "foobar2";
    let res = create_document(&filename);
    match res {
        Ok(file) => {
            println!("Created file");
        }
        Err(err) => {
            println!("Problem creating file: {:?}", err)
        }
    }
}
