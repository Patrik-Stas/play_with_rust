use std::collections::HashMap;

#[derive(Debug)]
struct Person {
    name: String,
    age: i32
}

trait Ageable {
    fn age(&mut self) -> i32;
}

impl Ageable for Person {
    fn age(&mut self) -> i32 {
        self.age += 1;
        self.age
    }
}

#[derive(Debug)]
struct EmployeeRegister {
    janitor: Option<Person>,
    manager: Person,
}

impl EmployeeRegister {

    pub fn age_janitor(&mut self) {
        // ISSUE: We had following code.
        // let mut janitor = self.janitor.unwrap();
        //                         ^^^^^^^^^^^^ cannot move out of borrowed content
        // As explained here: https://stackoverflow.com/questions/32338659/cannot-move-out-of-borrowed-content-when-unwrapping
        // unwrap moves value out. so self would no longer be owner of its self.janitor field,
        // instead, the let mut janitor would become owner of that data. However, we only have reference
        // to &mut self, so self is only borrowed. So we can't just start to own a subset (janitor)
        // of data which is borrowed (self)

        // The solution is to get janitor from the option by reference. We can do that using
        // as_ref() to get immutable reference, or as_mut() to get mutable reference.
        let janitor = self.janitor.as_mut().unwrap();
        janitor.age();
        ()
    }

    pub fn age_manager(&mut self) {
        self.manager.age();
        ()
    }

    pub fn set_janitor(&mut self, name: String, age: i32) {
        self.janitor = Option::from(Person { name, age: 41 })
    }


    pub fn print_janitor(&mut self) {
        println!("Janitor: {:?}", self.janitor);
    }

}

pub fn run()
{
    let mut register = EmployeeRegister { janitor: None, manager: Person { name: "Andrew".into(), age: 52} };
    register.print_janitor();
    register.set_janitor(String::from("Josh"), 32);
    register.print_janitor();
}