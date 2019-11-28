use std::fs::File;
use std::fs::OpenOptions;
use std::{io, result};
use failure::{Backtrace, Fail};
use crate::rim_3_6_custom_errors::DocumentServiceError::RateLimitExceeded;

const MAX_DOCS_CREATED_PER_MINUTE: u8 = 30;

fn num_documents_created_in_past_minute() -> u8 {
    2
}

#[derive(Debug, Fail)]
pub enum DocumentServiceError {
    #[fail(display = "You have exceeded the allowed number of documents per minute.")]
    RateLimitExceeded(Backtrace),
    #[fail(display = "I/O Error: {}", _0)]
    Io(io::Error, Backtrace),
}


impl From<io::Error> for DocumentServiceError
{
    fn from(other: io::Error) -> Self {
        DocumentServiceError::Io(other, Backtrace::new())
    }
}

pub fn create_document(filename: &str) -> Result<File>
{
    if num_documents_created_in_past_minute() > MAX_DOCS_CREATED_PER_MINUTE {
//        return Err("Create file limit exceeded.".into())
        return Err(DocumentServiceError::RateLimitExceeded(Backtrace::new()))
    }

    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(filename)?;

    Ok(file)
}

pub type Result<T> = result::Result<T, DocumentServiceError>;

pub fn update_document(filename: &str) -> Result<File>
{
    unimplemented!()
}

pub fn delete_document(filename: &str) -> Result<()>
{
    unimplemented!()
}

pub fn read_document(filename: &str) -> Result<File>
{
    unimplemented!()
}

pub fn create_project(project_name: &str) -> Result<()> {
    create_document(&format!("{}-draft1", project_name))?;
    create_document(&format!("{}-draft2", project_name))?;
    create_document(&format!("{}-revision1", project_name))?;
    create_document(&format!("{}-revision2", project_name))?;
    Ok(())
}

pub fn run()
{
    println!("START: failures");
    match create_project("my-project") {
        Ok(()) => println!("Project created successfully"),
        Err(e) => {
            println!("Project creation failed {}", e);
            if let Some(backtrace) = e.backtrace() {
                if !backtrace.to_string().trim().is_empty() {
                    println!("Backtrace: {:?}", backtrace);
                }
            }
        }
    }
    println!("FINISH: failures");
}