use std::collections::HashMap;

pub fn hashmap<S: std::hash::BuildHasher>(map: HashMap<String, String, S>) -> String {
    map.into_iter()
        .map(|(k, v)| format!("{k} • {v}"))
        .collect::<Vec<String>>()
        .join("\n")
}
