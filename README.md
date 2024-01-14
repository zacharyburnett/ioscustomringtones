[![build](https://github.com/zacharyburnett/ioscustomringtones/actions/workflows/build.yml/badge.svg)](https://github.com/zacharyburnett/ioscustomringtones/actions/workflows/build.yml)
![PyPI - Python Version](https://img.shields.io/pypi/pyversions/ioscustomringtones)
![PyPI - Version](https://img.shields.io/pypi/v/ioscustomringtones)

# use arbitrary audio files as iOS ring and alert tones

```shell
pip install ioscustomringtones
```

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

4. install `ioscustomringtones` with `pip`:
    ```shell
    pip install ioscustomringtones
    ```

5. run the `write_ios_ringtones_plist` command on the mounted `/var/root/Media/` directory:
    ```shell
    write_ios_ringtones_plist ~/iPhone_Media/
    ```

    the `write_ios_ringtones_plist` command provides several options:
    ```shell
    Usage: write_ios_ringtones_plist [OPTIONS] MEDIA_DIRECTORY

      on a mounted iOS filesystem, reads existing `.m4r` files at
      `/var/root/Media/iTunes_Control/Ringtones/` and generates
      `/var/root/Media/iTunes_Control/iTunes/Ringtones.plist`

    Arguments:
      MEDIA_DIRECTORY  path to the mounted iOS `/var/root/Media/`
                       directory  [required]

    Options:
      --alerts-threshold INTEGER    number of seconds under which to
                                    assume sound is an alert tone, as
                                    opposed to a ringtone  [default: 10]
      --binary / --no-binary        whether to write `.plist` in binary
                                    format (as opposed to XML)  [default:
                                    binary]
      --write / --no-write          write to file; otherwise print to
                                    stdout  [default: write]
      --overwrite / --no-overwrite  write over an existing file
                                    [default: no-overwrite]
      --verbose / --no-verbose      print individual tones to stdout
                                    [default: no-verbose]
      --help                        Show this message and exit.
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
