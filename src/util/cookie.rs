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
