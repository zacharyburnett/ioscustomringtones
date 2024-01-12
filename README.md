# generate `Ringtones.plist` for custom iOS ring and alert tones
### list custom tones in iOS settings by generating `/Media/iTunes_Control/iTunes/Ringtones.plist`

```shell
pip install ioscustomringtones
```

#### Instructions

1. convert audio files to `.m4a`, then rename to `.m4r`:
    ```shell
    ffmpeg -i some_tone.wav some_tone.m4a
    mv some_tone.m4a some_tone.m4r
    ```

2. mount the `/Media/` directory of your iOS device to your local filesystem (this does not require a jailbroken device):
    ```shell
    mkdir ~/iPhone_Media/
    ifuse ~/iPhone_Media/
    ```

3. copy your `.m4r` files to `~/iPhone_Media/iTunes_Control/Ringtones/`:
    ```shell
    cp ~/Music/Ringtones/*.m4r ~/iPhone_Media/iTunes_Control/Ringtones/
    ```

4. install `ioscustomringtones` with `pip`:
    ```shell
    pip install ioscustomringtones
    ```

5. pass the directory you mounted to the `write_ios_ringtones_plist` command (see more options below):
    ```shell
    write_ios_ringtones_plist ~/iPhone_Media/
    ```
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
    sudo umount ~/iPhone/
    ```

