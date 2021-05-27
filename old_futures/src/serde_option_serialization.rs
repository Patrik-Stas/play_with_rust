use serde_json;

#[derive(Serialize)]
struct Data {
    #[serde(skip_serializing_if = "Option::is_none")]
    b: Option<String>,
    c: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    foo: Option<String>
}

fn option_none_serialized_as_null() {
    let data = Data { b: None, c:None};
    let serialized = serde_json::to_string(&data).unwrap();
    println!("serialized={}", serialized);
}

fn can_deserialize_with_extra_fields() {
    let config = r#"{"foo": "fooval"}"#;
    let config: Config = serde_json::from_str(config).unwrap();
    println!("Deserialized config = {:?}", config);

    let config2 = r#"{"foo": "fooval", "bar": "extraval"}"#;
    let config2: Config = serde_json::from_str(config2`).unwrap();
    println!("Deserialized config = {:?}", config2);
}

pub fn run()
{
    can_deserialize_with_extra_fields()
}