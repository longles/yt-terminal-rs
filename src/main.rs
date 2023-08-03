mod render;
mod frames;

use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short = 'r', long, value_name = "video fps")]
    fps: Option<i32>,

    #[arg(short = 'w', long, value_name = "display width")]
    width: Option<i32>,

    #[arg(short = 'v', long, value_name = "video file path")]
    video_path: Option<PathBuf>,

    #[arg(short = 'f', long, value_name = "frame directory")]
    frame_path: Option<PathBuf>
}

fn main() {
    let _ = enable_ansi_support::enable_ansi_support();
    let args = Args::parse();

    let config = frames::VideoConfigs::new(
        args.video_path.unwrap(),
        args.frame_path.unwrap()
    ).set_fps(args.fps.unwrap()).use_terminal().build();

    // Call FFmpeg
    let status = frames::fetch_frames(&config);
    match status {
        Ok(_status) => (),
        Err(e) => {
            println!("{:?}", e);
            std::process::exit(1)
        }
    };

    render::pixel_render(&config);
}
