use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use crate::rim_3_6_custom_errors::DocumentServiceError::RateLimitExceeded;

const MAX_DOCS_CREATED_PER_MINUTE: u8 = 30;

pub mod errors {
    error_chain! {
        errors {
            RateLimitExceeded {
                display("You have exceeded the allowed number of documents per minute.")
            }
        }
        foreign_links {
            Io(::std::io::Error);
        }
    }
}

use errors::*;

fn num_documents_created_in_past_minute() -> u8 {
    2
}

pub fn create_document(filename: &str) -> Result<File>
{
    if num_documents_created_in_past_minute() > MAX_DOCS_CREATED_PER_MINUTE {
        bail!(ErrorKind::RateLimitExceeded)
    }

    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(filename)
        .chain_err(|| format!("could not open {}", filename))?;

    Ok(file)
}

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
    println!("START: error-chain");
    match create_project("my-project") {
        Ok(()) => println!("Project created successfully"),
        Err(e) => {
            println!("Project creation failed {}", e);
            for e in e.iter().skip(1) {
                println!("Caused by: {}", e);
            }
            if let Some(backtrace) = e.backtrace() {
                println!("Backtrace: {:?}", backtrace);
            }
        }
    }
    println!("FINISH: error-chain");
}
