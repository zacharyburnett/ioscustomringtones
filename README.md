[![build](https://github.com/zacharyburnett/ioscustomringtones/actions/workflows/build.yml/badge.svg)](https://github.com/zacharyburnett/ioscustomringtones/actions/workflows/build.yml)
![PyPI - Python Version](https://img.shields.io/pypi/pyversions/ioscustomringtones)
![PyPI - Version](https://img.shields.io/pypi/v/ioscustomringtones)

# manage custom iPhone alert and ring tones 

These instructions explain how to connect your iPhone to your computer, upload arbitrary audio files to it, and have those audio files show up in your Settings as options for text, alert, and ring tones. This does **not** require a jailbroken device.

#### Instructions

1. convert your desired audio files to `.m4a`, then rename their file extensions to `.m4r`:
    ```shell
    ffmpeg -i some_tone.wav some_tone.m4a
    mv some_tone.m4a some_tone.m4r
    ```

> [!IMPORTANT]
> The audio files need to be in `.m4a` format, and then have their file extension renamed from `.m4a` to `.m4r`. Be aware that your file browser likely hides file extensions by default; you may need to change that setting in order to rename the files.

> [!WARNING]
> If the file extension of an audio file is not already `.m4a`, you must convert the underlying format with `ffmpeg` or similar software. **Simply renaming the extension to `.m4a` will not work.** Remaming works for `.m4a` -> `.m4r` because those two are actually the same file format with different names.

2. connect your iPhone to your computer via USB, unlock it, and accept the prompt asking to `Trust` the connected device (your computer)

3. mount the `/var/root/Media/` directory of your iPhone to an empty folder on your computer: 
    ```shell
    mkdir ~/iPhone_Media/
    ifuse ~/iPhone_Media/
    ```
> [!IMPORTANT]
> These instructions use `ifuse`, which can be installed on Linux and macOS; 
> on Windows you can try [iMazing](https://imazing.com/download) or similar software.

4. copy your `.m4r` files to `/Media/iTunes_Control/Ringtones/` on the device:
    ```shell
    cp ~/Music/Ringtones/*.m4r ~/iPhone_Media/iTunes_Control/Ringtones/
    ```

5. download an executable from [the Releases page](https://github.com/zacharyburnett/ioscustomringtones/releases)

6. run the executable with `write` and wherever you mounted `/var/root/Media/`:
    ```shell
    ioscustomringtones.exe write ~/iPhone_Media/
    ```

7. unmount `/var/root/Media/`:
    ```shell
    fusermount -u ~/iPhone_Media/
    ```

8. disconnect and restart your iPhone

> [!NOTE]
> By default, any files under 10 seconds in length will be classified 
> as `tone` (alert / text tones), and above 10 seconds as `ringtone`. 
> You can change this threshold with the `--alerts-threshold` option, i.e.:
> ```shell
> write_ios_ringtones_plist ~/iPhone_Media/ --alerts-threshold 20
> ```

#### Commands

##### `ioscustomringtones --help`

```shell
manage custom iOS alert and ring tones on an iOS device (requires `/var/root/Media/` to be mounted locally, i.e. with `ifuse` or iMazing)

Usage: ioscustomringtones <COMMAND>

Commands:
  write     generate `Ringtones.plist` at `/var/root/Media/iTunes_Control/iTunes/Ringtones.plist`
  list      list `.m4r` files in `/var/root/Media/iTunes_Control/Ringtones/`
  validate  list discrepancies between `.m4r` files in `/var/root/Media/iTunes_Control/Ringtones/` and `/var/root/Media/iTunes_Control/iTunes/Ringtones.plist`
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

##### `ioscustomringtones validate --help`
```shell
list discrepancies between `.m4r` files in `/var/root/Media/iTunes_Control/Ringtones/` and `/var/root/Media/iTunes_Control/iTunes/Ringtones.plist`

Usage: ioscustomringtones validate <MEDIA_DIRECTORY> [ALERTS_THRESHOLD]

Arguments:
  <MEDIA_DIRECTORY>   path to mounted iOS `/var/root/Media/` directory
  [ALERTS_THRESHOLD]  number of seconds under which to assume sound is an alert tone, as opposed to a ringtone [default: 10]

Options:
  -h, --help  Print help
```

##### `ioscustomringtones list --help`
```shell
list `.m4r` files in `/var/root/Media/iTunes_Control/Ringtones/`

Usage: ioscustomringtones list <MEDIA_DIRECTORY> [ALERTS_THRESHOLD]

Arguments:
  <MEDIA_DIRECTORY>   path to mounted iOS `/var/root/Media/` directory
  [ALERTS_THRESHOLD]  number of seconds under which to assume sound is an alert tone, as opposed to a ringtone [default: 10]

Options:
  -h, --help  Print help
```

##### `ioscustomringtones write --help`
```shell
generate `Ringtones.plist` at `/var/root/Media/iTunes_Control/iTunes/Ringtones.plist`

Usage: ioscustomringtones write [OPTIONS] <MEDIA_DIRECTORY> [ALERTS_THRESHOLD]

Arguments:
  <MEDIA_DIRECTORY>   path to mounted iOS `/var/root/Media/` directory
  [ALERTS_THRESHOLD]  number of seconds under which to assume sound is an alert tone, as opposed to a ringtone [default: 10]

Options:
  -b, --binary     write in binary format (otherwise write XML)
  -o, --overwrite  write over an existing file
  -h, --help       Print help
```
