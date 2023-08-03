# Usage

First install ffmpeg and make sure it is on the PATH.

- [Windows](https://phoenixnap.com/kb/ffmpeg-windows)
- [Linux](https://phoenixnap.com/kb/install-ffmpeg-ubuntu)
- [MacOS](https://phoenixnap.phoenixnap/kb/ffmpeg-windows)

To test, type `ffmpeg -version` into your command line.

```shell
# Compile the program
cargo build --release

# Run the program
cargo run --release -- -r 30 -f <frame directory> -v <mp4 video path>
```

# Example
```shell
cargo run --release -- -r 30 -f ./frames -v ./nyan_cat.mp4
```


# TODO

- Download videos from youtube
- Use aspect ratio and framerate of video
- Different rendering options
- More efficient memeory usage
