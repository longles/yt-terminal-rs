use std::fs;
use std::path::{Path, PathBuf};
use std::io::{Error, ErrorKind};

// use metadata;
use term_size;
use ffpb;

// const SCALE_FACTOR: f32 = 0.45;
pub struct VideoConfigs {
    pub fps: Option<i32>,
    pub width: Option<usize>,
    pub height: Option<usize>,
    video_path: PathBuf,
    pub frame_path: PathBuf,
}

impl VideoConfigs {
    pub fn new(video_path: PathBuf, frame_path: PathBuf) -> VideoConfigsBuilder {
        return VideoConfigsBuilder {
            fps: None,
            width: None,
            height: None,
            video_path,
            frame_path
        }
    }

    pub fn get_video_path(&self) -> PathBuf {
        return self.video_path.clone();
    }

    pub fn get_frame_path(&self) -> PathBuf {
        return self.frame_path.clone();
    }
}

pub struct VideoConfigsBuilder {
    fps: Option<i32>,
    width: Option<usize>,
    height: Option<usize>,
    video_path: PathBuf,
    frame_path: PathBuf,
}

impl VideoConfigsBuilder {
    pub fn build(self) -> VideoConfigs {
        return VideoConfigs {
            fps: self.fps,
            width: self.width,
            height: self.height,
            video_path: self.video_path,
            frame_path: self.frame_path
        }
    }

    pub fn set_fps(mut self, fps: i32) -> Self {
        self.fps = Some(fps);
        return self;
    }

    pub fn use_terminal(mut self) -> Self {
        if let Some((width, height)) = term_size::dimensions() {
            self.width = Some(width);
            self.height = Some(height);
        } else {
            panic!("Terminal brokey");
        }

        return self;
    }

    // pub fn use_video(&mut self, width: usize) -> Self {
    //     self.width = Some(width);

    //     return self
    // }

}

pub fn fetch_frames(configs: &VideoConfigs) -> Result<(), Error> {
    let width = configs.width.unwrap();
    let height = configs.height.unwrap();
    let fps = configs.fps.unwrap();

    let args = [
        String::from("-i"), configs.get_video_path().display().to_string(),
        String::from("-vf"), format!("fps={},scale={}:{}", fps, width, height),
        format!("{}/%05d.png", configs.get_frame_path().display().to_string())
    ];

    if !Path::new(&configs.get_frame_path()).exists() {
        fs::create_dir(&configs.get_frame_path()).unwrap();
    }

    if !Path::new(&configs.get_video_path()).exists() {
        return Err(Error::new(ErrorKind::NotFound, format!("FileNotFoundError: {}", &configs.get_video_path().display().to_string())))
    }

    return ffpb::ffmpeg(&args);
}