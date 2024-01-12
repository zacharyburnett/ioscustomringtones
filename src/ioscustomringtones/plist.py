from pathlib import Path
import typer
from typing import Optional
from typing_extensions import Annotated
from mutagen import mp4
import plistlib
from io import BytesIO


def write_ios_ringtones_plist(
    media_directory: Annotated[
        Path, typer.Argument(help="path to `/Media/` on the iOS device")
    ],
    alerts_threshold: Annotated[
        Optional[int],
        typer.Option(
            help="number of seconds under which to assume sound is an alert tone vs a ringtone"
        ),
    ] = 10,
    binary: Annotated[
        Optional[bool],
        typer.Option(
            help="whether to write `.plist` in binary format (as opposed to XML)"
        ),
    ] = True,
    write: Annotated[
        Optional[bool],
        typer.Option(help="write to file; otherwise print to stdout"),
    ] = True,
    overwrite: Annotated[
        Optional[bool],
        typer.Option(
            help="write over an existing file",
        ),
    ] = False,
    verbose: Annotated[
        Optional[bool],
        typer.Option(help="print individual tones to stdout"),
    ] = False,
) -> str:
    """
    on a mounted iOS filesystem,
    reads existing `.m4r` files at `/Media/iTunes_Control/Ringtones/`
    and generates `/Media/iTunes_Control/iTunes/Ringtones.plist`
    """

    data = {"Ringtones": {}}

    for index, filename in enumerate(
        media_directory.glob("iTunes_Control/Ringtones/*.m4r")
    ):
        audio_length = mp4.MP4(filename).info.length
        audio_type = "tone" if audio_length < alerts_threshold else "ringtone"

        if verbose:
            print(f"{audio_type:<8} ({audio_length:.1f}s) - {filename.stem}")

        data["Ringtones"][filename.name] = {
            "Name": filename.stem,
            "GUID": f"B9753FD82AE718E{2+index}",
            "Total Time": audio_length,
            "PID": 4918251813855823052 + index,
            "Protected Content": False,
            "Media Kind": audio_type,
        }
    del index, filename

    if len(data["Ringtones"]) == 0:
        raise RuntimeError(
            f"no `.m4r` files found in {media_directory/'iTunes_Control/Ringtones'}"
        )

    if not write:
        file_object = BytesIO()
    else:
        filename = media_directory / "iTunes_Control" / "iTunes" / "Ringtones.plist"
        if not filename.exists() or overwrite:
            file_object = open(filename, "wb")
        else:
            raise FileExistsError("file exists; use `--overwrite` to overwrite")

    plistlib.dump(
        data,
        file_object,
        fmt=plistlib.FMT_BINARY if binary else plistlib.FMT_XML,
    )

    if not write:
        print(file_object.getvalue())


def main():
    typer.run(write_ios_ringtones_plist)
