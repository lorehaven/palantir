use std::collections::HashMap;

pub fn hashmap(map: HashMap<String, String>) -> String {
    map.into_iter()
        .map(|(k, v)| format!("{k} • {v}"))
        .collect::<Vec<String>>()
        .join("\n")
}