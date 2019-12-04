use std::collections::HashMap;

pub fn patrik_plays()
{
    let mut routes: HashMap<String, i32> = HashMap::new();
    routes.insert("a".into(), 1);
    routes.insert("b".into(), 2);
    routes.insert("c".into(), 3);
    routes.insert("d".into(), 4);
    routes.insert("e".into(), 5);

    let a = routes.get("a".into()).unwrap_or(&123);
    println!("value = {:?} (resolved from \"a\" key)", a);

    let def_aq = routes.get("aq".into()).unwrap_or(&123);
    println!("value = {:?} (used default value on unresolved key)", def_aq);

    let aq_custom = routes.get("aq".into()).unwrap_or_else(||&123);
    println!("value = {:?} (custom result calculation)", aq_custom);

    let collected = routes.iter().map(|keyvalue| keyvalue.1).collect::<Vec<_>>();;
    println!("iter collected = {:?}", collected);
}