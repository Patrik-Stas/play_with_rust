use std::env;

//fn num_threads() -> Result<usize, Box<dyn std::error::Error>>
//fn num_threads() -> Result<usize, env::VarError>
//fn num_threads() -> Result<usize, std::num::ParseIntError>
//fn num_threads() -> Result<usize, std::error::Error>
fn num_threads() -> Result<usize, Box<std::error::Error>>
{
    let s = env::var("NUM_THREADS")?;
    let n: usize = s.parse()?;
    Ok(n + 1)
}

pub fn run()
{
    let foo = num_threads();
}
