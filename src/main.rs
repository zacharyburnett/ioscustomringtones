extern crate plist;
#[macro_use]
extern crate serde_derive;
extern crate glob;

use clap::{Parser, Subcommand};
use core::panic;
use glob::glob;
use lofty;
use lofty::AudioFile;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// generate `Ringtones.plist` at `/var/root/Media/iTunes_Control/iTunes/Ringtones.plist`
    Write {
        /// path to mounted iOS `/var/root/Media/` directory
        media_directory: PathBuf,
        /// number of seconds under which to assume sound is an alert tone, as opposed to a ringtone
        #[arg(default_value_t = 10)]
        alerts_threshold: u16,
        /// write `.plist` in XML format
        #[arg(short, long, action)]
        xml: bool,
        /// write over an existing file
        #[arg(short, long, default_value_t = false)]
        overwrite: bool,
    },
    /// list `.m4r` files in `/var/root/Media/iTunes_Control/Ringtones/`
    List {
        /// path to mounted iOS `/var/root/Media/` directory
        media_directory: PathBuf,
        /// number of seconds under which to assume sound is an alert tone, as opposed to a ringtone
        #[arg(default_value_t = 10)]
        alerts_threshold: u16,
    },
    /// list discrepancies between `.m4r` files in `/var/root/Media/iTunes_Control/Ringtones/` and `/var/root/Media/iTunes_Control/iTunes/Ringtones.plist`
    Validate {
        /// path to mounted iOS `/var/root/Media/` directory
        media_directory: PathBuf,
        /// number of seconds under which to assume sound is an alert tone, as opposed to a ringtone
        #[arg(default_value_t = 10)]
        alerts_threshold: u16,
    },
}

#[derive(Deserialize, Serialize)]
struct Ringtones {
    #[serde(rename = "Ringtones")]
    ringtones: HashMap<String, Tone>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
struct Tone {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Total Time")]
    total_time: f64,
    #[serde(rename = "Media Kind")]
    media_kind: MediaKind,
    #[serde(rename = "Protected Content")]
    protected_content: bool,
    #[serde(rename = "PID")]
    pid: u64,
    #[serde(rename = "GUID")]
    guid: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
enum MediaKind {
    #[serde(rename = "tone")]
    Tone,
    #[serde(rename = "ringtone")]
    Ringtone,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Write {
            media_directory,
            alerts_threshold,
            xml,
            overwrite,
        } => {
            let tones_directory = media_directory.join("iTunes_Control/Ringtones");
            let tones = directory_tones(&tones_directory, &alerts_threshold);

            println!(
                "found {} `.m4r` files in {:?}",
                tones.len(),
                tones_directory.to_owned()
            );

            let filename = media_directory.join("iTunes_Control/iTunes/Ringtones.plist");
            if filename.exists() && !overwrite {
                panic!(
                    "file exists at {:?}; use `--overwrite` to overwrite",
                    filename
                );
            }

            let generated_list = Ringtones { ringtones: tones };

            let message = format!("unable to write {:?}", filename);
            if xml {
                plist::to_file_xml(filename, &generated_list).expect(&message);
            } else {
                plist::to_file_binary(filename, &generated_list).expect(&message);
            }
        }
        Command::List {
            media_directory,
            alerts_threshold,
        } => {
            let tones_directory = media_directory.join("iTunes_Control/Ringtones");
            let tones = directory_tones(&tones_directory, &alerts_threshold);

            println!(
                "found {} `.m4r` files in {:?}",
                tones.len(),
                tones_directory.to_owned()
            );

            for (_, tone) in tones {
                let media_kind = format!("{:?}", tone.media_kind);
                let total_time = format!("{:3.1}", tone.total_time);
                println!("{:>8} ({:>5}s) {}", media_kind, total_time, tone.name)
            }
        }
        Command::Validate {
            media_directory,
            alerts_threshold,
        } => {
            let tones_directory = media_directory.join("iTunes_Control/Ringtones");
            let tones = directory_tones(&tones_directory, &alerts_threshold);

            let filename = media_directory.join("iTunes_Control/iTunes/Ringtones.plist");
            if !filename.exists() {
                panic!(
                    "no file found at {:?}; use `--overwrite` to overwrite",
                    filename
                );
            }

            let generated_list = Ringtones { ringtones: tones };

            let message = format!("could not read {:?}", filename.to_owned());
            let existing_list: Ringtones = plist::from_file(filename.to_owned()).expect(&message);

            for (name, generated_tone) in generated_list.ringtones.iter() {
                let existing_tone = existing_list.ringtones.get(name);
                match existing_tone {
                    Some(existing_tone) => {
                        let mut differences = vec![];
                        if generated_tone != existing_tone {
                            fn diff(differences: &mut Vec<String>, key: &str, a: &str, b: &str) {
                                if a != b {
                                    let key = format!("\"{}\"", key);
                                    differences.push(format!("{:>21} {} != {}", key, a, b));
                                }
                            }
                            diff(
                                &mut differences,
                                "Name",
                                &generated_tone.name,
                                &existing_tone.name,
                            );
                            diff(
                                &mut differences,
                                "Total Time",
                                &format!("{}", generated_tone.total_time),
                                &format!("{}", existing_tone.total_time),
                            );
                            diff(
                                &mut differences,
                                "Media Kind",
                                &format!("{:?}", generated_tone.media_kind),
                                &format!("{:?}", existing_tone.media_kind),
                            );
                            diff(
                                &mut differences,
                                "Protected Content",
                                &format!("{}", generated_tone.protected_content),
                                &format!("{}", existing_tone.protected_content),
                            );
                            diff(
                                &mut differences,
                                "PID",
                                &format!("{}", &generated_tone.pid),
                                &format!("{}", &existing_tone.pid),
                            );
                            diff(
                                &mut differences,
                                "GUID",
                                &format!("{}", &generated_tone.guid),
                                &format!("{}", &existing_tone.guid),
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
    }
}

fn directory_tones(directory: &PathBuf, alerts_threshold: &u16) -> HashMap<String, Tone> {
    let mut tones = HashMap::<String, Tone>::new();

    for (index, entry) in glob(directory.join("*.m4r").to_str().unwrap())
        .expect("failed to read tone pattern")
        .enumerate()
    {
        match entry {
            Ok(path) => {
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

                tones.insert(
                    path.file_name().unwrap().to_str().unwrap().to_string(),
                    Tone {
                        name: path.file_stem().unwrap().to_str().unwrap().to_string(),
                        total_time: duration,
                        media_kind,
                        protected_content: false,
                        pid: 4918251813855823052 + index as u64,
                        guid: format!("B9753FD82AE718E{}", 2 + index),
                    },
                );
            }
            Err(_) => {}
        }
    }

    tones
}
