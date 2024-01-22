mod data;
mod list_tones;
mod ringtones_plist;

extern crate plist;
#[macro_use]
extern crate serde_derive;
extern crate glob;

use crate::list_tones::print_tones_in_directory;
use crate::ringtones_plist::{ringtones_plist, validate_ringtones_plist, write_ringtones_plist};
use clap::{Parser, Subcommand};
use list_tones::tones_directory;
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

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Write {
            media_directory,
            alerts_threshold,
            xml,
            overwrite,
        } => {
            write_ringtones_plist(
                &tones_directory(&media_directory),
                &ringtones_plist(&media_directory),
                &alerts_threshold,
                xml,
                overwrite,
            );
        }
        Command::List {
            media_directory,
            alerts_threshold,
        } => print_tones_in_directory(&tones_directory(&media_directory), &alerts_threshold),
        Command::Validate {
            media_directory,
            alerts_threshold,
        } => validate_ringtones_plist(
            &tones_directory(&media_directory),
            &ringtones_plist(&media_directory),
            &alerts_threshold,
        ),
    }
}
