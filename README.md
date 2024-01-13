[![build](https://github.com/zacharyburnett/ioscustomringtones/actions/workflows/build.yml/badge.svg)](https://github.com/zacharyburnett/ioscustomringtones/actions/workflows/build.yml)
![PyPI - Python Version](https://img.shields.io/pypi/pyversions/ioscustomringtones)
![PyPI - Version](https://img.shields.io/pypi/v/ioscustomringtones)

# generate `Ringtones.plist` for custom iOS ring and alert tones
### list custom tones in iOS settings by generating `/Media/iTunes_Control/iTunes/Ringtones.plist`

```shell
pip install ioscustomringtones
```

> [!IMPORTANT]
> This process requires you to connect your iOS device to a computer via USB and browse / edit its files. 
> These instructions use `ifuse` for Linux and macOS; on Windows you can use [iMazing](https://imazing.com/download).

#### Instructions

1. convert audio files to `.m4a`, then rename the extension to `.m4r`:
    ```shell
    ffmpeg -i some_tone.wav some_tone.m4a
    mv some_tone.m4a some_tone.m4r
    ```

    > [!IMPORTANT]
    > The audio files need to have the `.m4r` extension (identical to `.m4a` but the name is important). 
    > I used `ffmpeg` here, but you can use whatever software you like.

2. mount the `/Media/` directory of your iOS device to local filesystem: 
    ```shell
    mkdir ~/iPhone_Media/
    ifuse ~/iPhone_Media/
    ```

    > [!NOTE]
    > Mounting `/Media/` does *not* require a jailbroken device.

3. copy your `.m4r` files to `/Media/iTunes_Control/Ringtones/` on the device:
    ```shell
    cp ~/Music/Ringtones/*.m4r ~/iPhone_Media/iTunes_Control/Ringtones/
    ```

4. install `ioscustomringtones` with `pip`:
    ```shell
    pip install ioscustomringtones
    ```

5. run the `write_ios_ringtones_plist` command on the mounted `/Media/` directory:
    ```shell
    write_ios_ringtones_plist ~/iPhone_Media/
    ```

    the `write_ios_ringtones_plist` command provides several options:
    ```shell
    Usage: write_ios_ringtones_plist [OPTIONS] MEDIA_DIRECTORY

      on a mounted iOS filesystem, reads existing `.m4r` files at
      `/Media/iTunes_Control/Ringtones/` and generates
      `/Media/iTunes_Control/iTunes/Ringtones.plist`

    Arguments:
      MEDIA_DIRECTORY  path to `/Media/` on the iOS device  [required]

    Options:
      --alerts-threshold INTEGER    number of seconds under which to
                                    assume sound is an alert tone vs a
                                    ringtone  [default: 10]
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

6. unmount the `/Media/` directory from your local filesystem:
    ```shell
    sudo umount ~/iPhone_Media/
    ```

7. restart your device
