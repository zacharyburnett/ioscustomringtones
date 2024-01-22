use crate::data::Ringtones;
use crate::list_tones::tones_in_directory;
use std::path::PathBuf;

pub fn ringtones_plist(media_directory: &PathBuf) -> PathBuf {
    media_directory.join("iTunes_Control/iTunes/Ringtones.plist")
}

pub fn write_ringtones_plist(
    tones_directory: &PathBuf,
    filename: &PathBuf,
    alerts_threshold: &u16,
    xml: bool,
    overwrite: bool,
) {
    if filename.exists() && !overwrite {
        panic!(
            "file exists at {:?}; use `--overwrite` to overwrite",
            filename
        );
    }

    let tones = tones_in_directory(tones_directory, alerts_threshold);

    println!(
        "found {} `.m4r` files in {:?}",
        tones.len(),
        tones_directory
    );

    let tones = Ringtones { ringtones: tones };
    let message = format!("unable to write {:?}", filename);
    if xml {
        plist::to_file_xml(filename, &tones).expect(&message);
    } else {
        plist::to_file_binary(filename, &tones).expect(&message);
    }
}

pub fn validate_ringtones_plist(
    tones_directory: &PathBuf,
    filename: &PathBuf,
    alerts_threshold: &u16,
) {
    if !filename.exists() {
        panic!("file not found at {:?}", filename);
    }

    let tones = tones_in_directory(&tones_directory, &alerts_threshold);

    let tones = Ringtones { ringtones: tones };

    let message = format!("could not read {:?}", filename.to_owned());
    let entries: Ringtones = plist::from_file(filename.to_owned()).expect(&message);

    for (name, tone) in tones.ringtones.iter() {
        let entry = entries.ringtones.get(name);
        match entry {
            Some(entry) => {
                let mut differences = vec![];
                if tone != entry {
                    fn diff(differences: &mut Vec<String>, key: &str, a: &str, b: &str) {
                        if a != b {
                            let key = format!("\"{}\"", key);
                            differences.push(format!("{:>21} {} != {}", key, a, b));
                        }
                    }
                    diff(&mut differences, "Name", &tone.name, &entry.name);
                    diff(
                        &mut differences,
                        "Total Time",
                        &format!("{}", tone.total_time),
                        &format!("{}", entry.total_time),
                    );
                    diff(
                        &mut differences,
                        "Media Kind",
                        &format!("{:?}", tone.media_kind),
                        &format!("{:?}", entry.media_kind),
                    );
                    diff(
                        &mut differences,
                        "Protected Content",
                        &format!("{}", tone.protected_content),
                        &format!("{}", entry.protected_content),
                    );
                    diff(
                        &mut differences,
                        "PID",
                        &format!("{}", &tone.pid),
                        &format!("{}", &entry.pid),
                    );
                    diff(
                        &mut differences,
                        "GUID",
                        &format!("{}", &tone.guid),
                        &format!("{}", &entry.guid),
                    );

                    if differences.len() > 0 {
                        println!("{}", name);
                        for difference in differences {
                            println!("{}", difference);
                        }
                    }
                }
            }
            None => {
                println!("{}\nnot present in {:?}", name, filename.to_owned());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::list_tones::tones_directory;

    #[test]
    fn test_write_ringtones_plist() {
        let media_directory = PathBuf::from(format!(
            "{:}/{:}",
            env!("CARGO_MANIFEST_DIR"),
            "data/iPhone_Media"
        ));

        let filename = media_directory
            .parent()
            .unwrap()
            .join("test_Ringtones.plist");
        write_ringtones_plist(
            &tones_directory(&media_directory),
            &filename,
            &10,
            false,
            true,
        );

        assert!(ringtones_plists_equal(
            &ringtones_plist(&media_directory),
            &filename
        ));
    }

    fn ringtones_plists_equal(filename_1: &PathBuf, filename_2: &PathBuf) -> bool {
        let message = format!("could not read {:?}", filename_1);
        let file_1: Ringtones = plist::from_file(filename_1.to_owned()).expect(&message);

        let message = format!("could not read {:?}", filename_2);
        let file_2: Ringtones = plist::from_file(filename_2.to_owned()).expect(&message);

        file_1 == file_2
    }
}
