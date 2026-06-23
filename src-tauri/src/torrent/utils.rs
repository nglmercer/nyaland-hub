pub fn extract_hash_from_magnet(magnet: &str) -> Option<String> {
    let prefix = "xt=urn:btih:";
    let start = magnet.find(prefix)? + prefix.len();
    let rest = &magnet[start..];
    let end = rest.find('&').unwrap_or(rest.len());
    let hash = &rest[..end];
    if hash.len() == 40 {
        Some(hash.to_lowercase())
    } else {
        None
    }
}

pub fn extract_trackers_from_magnet(magnet: &str) -> Vec<String> {
    let mut trackers = Vec::new();
    for part in magnet.split('&') {
        if let Some(tr) = part.strip_prefix("tr=") {
            if let Ok(decoded) = urlencoding::decode(tr) {
                trackers.push(decoded.to_string());
            }
        }
    }
    trackers
}

pub fn truncate_magnet(magnet: &str) -> String {
    if let Some(name_start) = magnet.find("dn=") {
        let encoded = &magnet[name_start + 3..];
        let end = encoded.find('&').unwrap_or(encoded.len());
        let name = &encoded[..end];
        urlencoding::decode(name).unwrap_or_default().to_string()
    } else if magnet.len() > 60 {
        format!("{}...", &magnet[..60])
    } else {
        magnet.to_string()
    }
}

pub fn hash_fallback(data: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for &byte in data {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}
