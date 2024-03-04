use std::path::PathBuf;

use crate::data::{MediaKind, Tone};
use glob::glob;
use lofty;
use lofty::AudioFile;
use sha256;
use std::collections::HashMap;

pub fn tones_in_directory(directory: &PathBuf, alerts_threshold: &u16) -> HashMap<String, Tone> {
    let mut tones = HashMap::<String, Tone>::new();

    let mut paths: Vec<PathBuf> = vec![];
    for entry in
        glob(directory.join("*.m4r").to_str().unwrap()).expect("failed to read tone pattern")
    {
        match entry {
            Ok(path) => {
                paths.push(path);
            }
            Err(_) => {}
        }
    }

    paths.sort_unstable();

    for path in paths {
        let message = format!("could not read {:?}", path.to_owned());
        let duration = lofty::read_from_path(path.to_owned())
            .expect(&message)
            .properties()
            .duration()
            .as_secs_f64();
        let media_kind = if duration < *alerts_threshold as f64 {
            MediaKind::Tone
        } else {
            MediaKind::Ringtone
        };

        let name = path.file_name().unwrap().to_str().unwrap().to_string();

        tones.insert(
            name.to_owned(),
            Tone {
                name: path.file_stem().unwrap().to_str().unwrap().to_string(),
                total_time: duration,
                media_kind,
                protected_content: false,
                pid: string_to_pid(&name),
                guid: string_to_guid(&name),
            },
        );
    }

    tones
}

pub fn tones_directory(media_directory: &PathBuf) -> PathBuf {
    media_directory.join("iTunes_Control/Ringtones")
}

pub fn print_tones_in_directory(tones_directory: &PathBuf, alerts_threshold: &u16) {
    let tones = tones_in_directory(tones_directory, alerts_threshold);

    println!(
        "found {} `.m4r` files in {:?}",
        tones.len(),
        tones_directory
    );

    for (_, tone) in tones {
        let media_kind = format!("{:?}", tone.media_kind);
        let total_time = format!("{:3.1}", tone.total_time);
        println!("{:>8} ({:>5}s) {}", media_kind, total_time, tone.name)
    }
}

/// pseudo-hash of string to 19-digit PID
fn string_to_pid(string: &str) -> u64 {
    let hash = sha256::digest(string);
    let mut digits = String::new();
    for character in hash.as_bytes() {
        digits.push_str(&character.to_string());
    }

    {
        let start_index = digits.to_string().char_indices().nth_back(18).unwrap().0;
        digits[start_index..].parse::<u64>().unwrap()
    }
}

/// pseudo-hash of string to 16-character GUID
fn string_to_guid(string: &str) -> String {
    let hash = sha256::digest(string);

    {
        let start_index = hash.to_string().char_indices().nth_back(15).unwrap().0;
        hash[start_index..].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_tones_10s() {
        let media_directory = PathBuf::from(format!(
            "{:}/{:}",
            env!("CARGO_MANIFEST_DIR"),
            "example/iPhone_Media"
        ));
        let tones = tones_in_directory(&tones_directory(&media_directory), &10);

        let filename_1 = "TP_Bottle_Pop.m4r";
        let filename_2 = "SC_Strategic_Launch.m4r";
        let filename_3 = "TP_HawkGrass.m4r";
        let filename_4 = "OuterWilds_End_Times.m4r";

        assert!(tones.contains_key(filename_1));
        assert!(tones.contains_key(filename_2));
        assert!(tones.contains_key(filename_3));
        assert!(tones.contains_key(filename_4));

        let tone_1 = tones.get(filename_1).unwrap();
        let tone_2 = tones.get(filename_2).unwrap();
        let tone_3 = tones.get(filename_3).unwrap();
        let tone_4 = tones.get(filename_4).unwrap();

        assert_eq!("TP_Bottle_Pop", tone_1.name);
        assert_eq!(tone_1.total_time, 0.244);
        assert_eq!(MediaKind::Tone, tone_1.media_kind);
        assert!(!tone_1.protected_content);

        assert_eq!("SC_Strategic_Launch", tone_2.name);
        assert_eq!(tone_2.total_time, 1.741);
        assert_eq!(MediaKind::Tone, tone_2.media_kind);
        assert!(!tone_2.protected_content);

        assert_eq!("TP_HawkGrass", tone_3.name);
        assert_eq!(tone_3.total_time, 11.19);
        assert_eq!(MediaKind::Ringtone, tone_3.media_kind);
        assert!(!tone_3.protected_content);

        assert_eq!("OuterWilds_End_Times", tone_4.name);
        assert_eq!(tone_4.total_time, 14.943);
        assert_eq!(MediaKind::Ringtone, tone_4.media_kind);
        assert!(!tone_4.protected_content);

        assert_ne!(tone_1.guid, tone_2.guid);
        assert_ne!(tone_1.guid, tone_3.guid);
        assert_ne!(tone_1.guid, tone_4.guid);
        assert_ne!(tone_2.guid, tone_1.guid);
        assert_ne!(tone_2.guid, tone_3.guid);
        assert_ne!(tone_2.guid, tone_4.guid);
        assert_ne!(tone_3.guid, tone_1.guid);
        assert_ne!(tone_3.guid, tone_2.guid);
        assert_ne!(tone_3.guid, tone_4.guid);
        assert_ne!(tone_4.guid, tone_1.guid);
        assert_ne!(tone_4.guid, tone_2.guid);
        assert_ne!(tone_4.guid, tone_3.guid);

        assert_ne!(tone_1.pid, tone_2.pid);
        assert_ne!(tone_1.pid, tone_3.pid);
        assert_ne!(tone_1.pid, tone_4.pid);
        assert_ne!(tone_2.pid, tone_1.pid);
        assert_ne!(tone_2.pid, tone_3.pid);
        assert_ne!(tone_2.pid, tone_4.pid);
        assert_ne!(tone_3.pid, tone_1.pid);
        assert_ne!(tone_3.pid, tone_2.pid);
        assert_ne!(tone_3.pid, tone_4.pid);
        assert_ne!(tone_4.pid, tone_1.pid);
        assert_ne!(tone_4.pid, tone_2.pid);
        assert_ne!(tone_4.pid, tone_3.pid);
    }

    #[test]
    fn test_list_tones_1s() {
        let media_directory = PathBuf::from(format!(
            "{:}/{:}",
            env!("CARGO_MANIFEST_DIR"),
            "example/iPhone_Media"
        ));
        let tones = tones_in_directory(&tones_directory(&media_directory), &1);

        let filename_1 = "TP_Bottle_Pop.m4r";
        let filename_2 = "SC_Strategic_Launch.m4r";
        let filename_3 = "TP_HawkGrass.m4r";
        let filename_4 = "OuterWilds_End_Times.m4r";

        let tone_1 = tones.get(filename_1).unwrap();
        let tone_2 = tones.get(filename_2).unwrap();
        let tone_3 = tones.get(filename_3).unwrap();
        let tone_4 = tones.get(filename_4).unwrap();

        assert_eq!(MediaKind::Tone, tone_1.media_kind);
        assert_eq!(MediaKind::Ringtone, tone_2.media_kind);
        assert_eq!(MediaKind::Ringtone, tone_3.media_kind);
        assert_eq!(MediaKind::Ringtone, tone_4.media_kind);
    }

    #[test]
    fn test_list_tones_12s() {
        let media_directory = PathBuf::from(format!(
            "{:}/{:}",
            env!("CARGO_MANIFEST_DIR"),
            "example/iPhone_Media"
        ));
        let tones = tones_in_directory(&tones_directory(&media_directory), &12);

        let filename_1 = "TP_Bottle_Pop.m4r";
        let filename_2 = "SC_Strategic_Launch.m4r";
        let filename_3 = "TP_HawkGrass.m4r";
        let filename_4 = "OuterWilds_End_Times.m4r";

        let tone_1 = tones.get(filename_1).unwrap();
        let tone_2 = tones.get(filename_2).unwrap();
        let tone_3 = tones.get(filename_3).unwrap();
        let tone_4 = tones.get(filename_4).unwrap();

        assert_eq!(MediaKind::Tone, tone_1.media_kind);
        assert_eq!(MediaKind::Tone, tone_2.media_kind);
        assert_eq!(MediaKind::Tone, tone_3.media_kind);
        assert_eq!(MediaKind::Ringtone, tone_4.media_kind);
    }

    #[test]
    fn test_string_to_pid() {
        let filename_1 = "TP_Bottle_Pop.m4r";
        let filename_2 = "SC_Strategic_Launch.m4r";
        let filename_3 = "TP_HawkGrass.m4r";
        let filename_4 = "OuterWilds_End_Times.m4r";

        assert_eq!(string_to_pid(filename_1), 5550521009748555449);
        assert_eq!(string_to_pid(filename_2), 5049102565010010251);
        assert_eq!(string_to_pid(filename_3), 4898495154494849100);
        assert_eq!(string_to_pid(filename_4), 9102535255494951101);
    }

    #[test]
    fn test_string_to_guid() {
        let filename_1 = "TP_Bottle_Pop.m4r";
        let filename_2 = "SC_Strategic_Launch.m4r";
        let filename_3 = "TP_HawkGrass.m4r";
        let filename_4 = "OuterWilds_End_Times.m4r";

        assert_eq!(string_to_guid(filename_1), "9ff460f724da0761");
        assert_eq!(string_to_guid(filename_2), "5231123621f82df3");
        assert_eq!(string_to_guid(filename_3), "03461b80b136101d");
        assert_eq!(string_to_guid(filename_4), "72f93bb1f547113e");
    }
}
