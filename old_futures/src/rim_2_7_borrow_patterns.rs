pub fn run()
{
    let mut list = vec![1, 2, 3];

//    *list.first_mut().expect("list was empty") += 1; // if we dont' create a actual variable, mutable borrow only "lasts" on this line

    {
        let list_first = list.first();
        let list_last = list.last();

        println!(
            "First elemet = {:?}, last element = {:?}",
            list_first,
            list_last
        );
    }
    let list_first_mut = list.first_mut().expect("list was empty");
    *list_first_mut += 1;
//    {
//        let list_first_mut = list.first_mut().expect("list was empty");
//        *list_first_mut += 1;
//    }

    let list_first = list.first();
    let list_last = list.last();

    println!(
        "First elemet = {:?}, last element = {:?}",
        list_first,
        list_last
    );
//    *list.first_mut().expect("list was empty") += 1;
}