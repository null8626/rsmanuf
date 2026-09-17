use regex::bytes::{Regex, RegexBuilder};
use std::{collections::BTreeMap, sync::LazyLock};

mod helpers;
#[cfg(feature = "online")]
pub mod online;

static CONTENT: LazyLock<BTreeMap<(u64, u8), String>> =
    LazyLock::new(|| helpers::parse_content(include_str!("manuf.txt")));
static MAC_REGEX: LazyLock<Regex> =
    LazyLock::new(|| RegexBuilder::new(r"^([0-9A-Fa-f]{2}[:-]){5}([0-9A-Fa-f]{2})$")
        .unicode(false)
        .build()
        .unwrap()
    );

pub fn lookup(mac: impl Into<String>) -> Result<String, String> {
    let new_mac = mac.into().to_ascii_uppercase().replace('-', ":");

    if MAC_REGEX.find(new_mac.as_bytes()).is_none() {
        return Err(String::from("Invalid MAC address"));
    }
    let mac_val = helpers::mac_to_u64(&new_mac).ok_or("Invalid MAC format")?;

    for &cidr in &[36, 28, 24] {
        let masked = helpers::mask_mac(mac_val, cidr);
        if let Some(m) = CONTENT.get(&(masked, cidr)) {
            return Ok(m.clone());
        }
    }

    Ok(String::from("unknown"))
}

#[deprecated(since = "2025.2.11", note = "please use `rsmanuf::lookup()` instead")]
#[derive(Debug, Clone)]
pub struct Index {}

#[allow(deprecated)]
impl Index {
    #[allow(clippy::new_without_default)]
    #[deprecated(since = "2025.2.11", note = "please use `rsmanuf::lookup()` instead")]
    pub fn new() -> Self {
        Index {}
    }

    #[deprecated(since = "2025.2.11", note = "please use `rsmanuf::lookup()` instead")]
    pub fn search(&self, mac: impl Into<String>) -> Result<String, String> {
        lookup(mac)
    }
}
