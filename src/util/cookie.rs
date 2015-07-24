use std::collections::HashMap;

pub fn parse_cookie(cookie: &str) -> HashMap<&str, &str> {
    let kvs: Vec<Vec<&str>> = cookie.split('&')
        .map(|kv| kv.split('=').collect())
        .collect();
    kvs.iter().map(|vec| (vec[0], vec[1])).collect()
}
