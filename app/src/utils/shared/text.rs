use base64::Engine;

pub fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    chars
        .next()
        .map(|first| first.to_uppercase().collect::<String>() + chars.as_str())
        .unwrap_or_default()
}

pub fn decode_jwt_token(token: &str) -> String {
    let jwt_token = token.split('.').collect::<Vec<&str>>()[1];
    let bytes_url = base64::engine::GeneralPurpose::new(
        &base64::alphabet::URL_SAFE,
        base64::engine::general_purpose::NO_PAD,
    )
    .decode(jwt_token)
    .unwrap();
    String::from_utf8(bytes_url).unwrap()
}
