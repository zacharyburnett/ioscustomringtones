[![build](https://github.com/zacharyburnett/ioscustomringtones/actions/workflows/build.yml/badge.svg)](https://github.com/zacharyburnett/ioscustomringtones/actions/workflows/build.yml)
![PyPI - Python Version](https://img.shields.io/pypi/pyversions/ioscustomringtones)
![PyPI - Version](https://img.shields.io/pypi/v/ioscustomringtones)

# manage custom iOS alert and ring tones on an iOS device

> [!IMPORTANT]
> This process requires you to connect your iOS device to a computer via USB and browse / edit its files. 
> These instructions use `ifuse` for Linux and macOS; 
> on Windows you can try [iMazing](https://imazing.com/download) or something similar.

> [!IMPORTANT]
> The audio files need to have the `.m4r` extension (identical to `.m4a` but the name is important). 
> I used `ffmpeg` here, but you can use whatever software you like to convert the files.

> [!NOTE]
> This does *not* require a jailbroken device.

#### Instructions

1. convert the audio files to `.m4a`, then rename the extension to `.m4r`:
    ```shell
    ffmpeg -i some_tone.wav some_tone.m4a
    mv some_tone.m4a some_tone.m4r
    ```

2. mount the `/var/root/Media/` directory of your iOS device to local filesystem: 
    ```shell
    mkdir ~/iPhone_Media/
    ifuse ~/iPhone_Media/
    ```

3. copy your `.m4r` files to `/Media/iTunes_Control/Ringtones/` on the device:
    ```shell
    cp ~/Music/Ringtones/*.m4r ~/iPhone_Media/iTunes_Control/Ringtones/
    ```

4. download an executable file for your OS from [the Releases page](https://github.com/zacharyburnett/ioscustomringtones/releases)

5. run the `write` command on the mounted `/var/root/Media/` directory:
    ```shell
    ./ioscustomringtones write ~/iPhone_Media/
    ```

6. unmount the `/var/root/Media/` directory from your local filesystem:
    ```shell
    fusermount -u ~/iPhone_Media/
    ```

7. restart your device

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
