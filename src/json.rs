use serde_json;

pub fn to_json(data: String) -> serde_json::Value {
    // Some JSON input data as a &str. Maybe this comes from the user.
    // Parse the string of data into serde_json::Value.
    let v = match serde_json::from_str(&data) {
        Ok(res) => res,
        Err(e) => {
            print!("{}", e);
            serde_json::Value::Null
        },
    };
    v
}