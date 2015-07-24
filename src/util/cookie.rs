use std::collections::HashMap;

pub fn parse_cookie(cookie: &str) -> HashMap<&str, &str> {
    let kvs: Vec<Vec<&str>> = cookie.split('&')
        .map(|kv| kv.split('=').collect())
        .collect();
    kvs.iter().map(|vec| (vec[0], vec[1])).collect()
}

pub fn encode_cookie(map: &HashMap<&str, &str>) -> String {
    let mut result = String::new();
    for (&k, &v) in map.iter() {
        result.push_str(k);
        result.push_str("=");
        result.push_str(v);
        result.push_str("&");
    }
    result.pop();
    result
}

fn encode_profile(map: &HashMap<&str, &str>) -> String {
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
    map.insert("uid", "10");
    map.insert("role", "user");
    map.insert("email", email);
    Ok(encode_profile(&map))
}
