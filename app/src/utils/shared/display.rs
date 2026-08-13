use std::collections::HashMap;

pub fn hashmap<S: std::hash::BuildHasher>(map: HashMap<String, String, S>) -> String {
    map.into_iter()
        .map(|(k, v)| format!("{k} • {}", pretty_if_json(&v)))
        .collect::<Vec<String>>()
        .join("\n")
}

/// Pretty-prints `value` if it parses as a JSON object or array.
///
/// Annotations in particular routinely carry a whole JSON document as their
/// value (`kubectl.kubernetes.io/last-applied-configuration`, `ArgoCD`'s
/// tracking annotations, ...) - rendered as the single compact line K8s
/// stores it as, that's unreadable. Left untouched when it doesn't parse as
/// JSON, and also when it parses as some other JSON type (a bare
/// number/string/bool has no structure for pretty-printing to clarify).
pub fn pretty_if_json(value: &str) -> String {
    match serde_json::from_str::<serde_json::Value>(value) {
        Ok(parsed @ (serde_json::Value::Object(_) | serde_json::Value::Array(_))) => {
            serde_json::to_string_pretty(&parsed).unwrap_or_else(|_| value.to_string())
        }
        _ => value.to_string(),
    }
}
