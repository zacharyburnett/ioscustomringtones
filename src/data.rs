use std::collections::HashMap;

#[derive(Deserialize, Serialize, PartialEq)]
pub struct Ringtones {
    #[serde(rename = "Ringtones")]
    pub ringtones: HashMap<String, Tone>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Tone {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Total Time")]
    pub total_time: f64,
    #[serde(rename = "Media Kind")]
    pub media_kind: MediaKind,
    #[serde(rename = "Protected Content")]
    pub protected_content: bool,
    #[serde(rename = "PID")]
    pub pid: u64,
    #[serde(rename = "GUID")]
    pub guid: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub enum MediaKind {
    #[serde(rename = "tone")]
    Tone,
    #[serde(rename = "ringtone")]
    Ringtone,
}
