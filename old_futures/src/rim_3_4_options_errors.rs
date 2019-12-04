pub fn take_by_value(s: String)
{
    println!("{:?}", s)
}

pub fn take_by_ref(s: &str)
{
    println!("{:?}", s)
}

pub fn result_ret(i: u32) -> Result<u32, &'static str>
{
    if (i == 42) {
        Err("Bug simulation")
    }
    else if (i < 100) {
        Ok(i + 100)
    } else {
        Err("Can't be higher than 100")
    }
}

pub fn option_ret(i: i32) -> Option<i32>
{
    if (i < 100) {
        Some(i + 100)
    } else {
        None
    }
}

pub fn option_ret_negative_wrapper(i: i32) -> Option<i32>
{
    let res = option_ret(i)?;
    Some(-res)
}


pub fn run()
{
    assert_eq!(result_ret(50), Ok(150));
    assert_eq!(result_ret(150), Err("Can't be higher than 100"));
    assert_eq!(option_ret(50), Some(150));
    assert_eq!(option_ret_negative_wrapper(50), Some(-150));
    assert_eq!(option_ret_negative_wrapper(150), None);
}

#[test]
fn result_works_for_42() -> Result<(), Box<dyn std::error::Error>>{
    let number = result_ret(42)?;
    Ok(())
}