use std::collections::HashMap;

pub fn parse_cookie(cookie: &str) -> HashMap<String, String> {
    let kvs: Vec<Vec<&str>> = cookie.split('&')
        .map(|kv| kv.split('=').collect())
        .collect();
    kvs.iter().map(|vec| (vec[0].to_owned(), vec[1].to_owned())).collect()
}

pub fn encode_cookie(map: &HashMap<String, String>) -> String {
    let mut result = String::new();
    for (k, v) in map.iter() {
        result.push_str(&k);
        result.push_str("=");
        result.push_str(&v);
        result.push_str("&");
    }
    result.pop();
    result
}

pub fn encode_profile(map: &HashMap<String, String>) -> String {
    let mut result = String::new();
    result.push_str("email=");
    result.push_str(map.get("email").unwrap());
    result.push_str("&uid=");
    result.push_str(map.get("uid").unwrap());
    result.push_str("&role=");
    result.push_str(map.get("role").unwrap());
    result
}

pub fn profile_for(email: &str) -> Result<String, &str> {
    if email.contains("&") || email.contains("=") {
        return Err("Email contains illegal characters.")
    }

    let mut map = HashMap::new();
    map.insert("uid".to_string(), "10".to_string());
    map.insert("role".to_string(), "user".to_string());
    map.insert("email".to_string(), email.to_owned());
    Ok(encode_profile(&map))
}
