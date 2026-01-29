pub fn parse_scalar(input: &str) -> serde_yaml::Value {
    if let Ok(val) = serde_yaml::from_str::<serde_yaml::Value>(input) {
        match val {
            serde_yaml::Value::Bool(_)
            | serde_yaml::Value::Number(_)
            | serde_yaml::Value::Null
            | serde_yaml::Value::String(_) => return val,
            _ => {}
        }
    }

    serde_yaml::Value::String(input.to_string())
}
