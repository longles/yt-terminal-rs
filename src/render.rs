
use std::fs::{self, DirEntry};
use std::collections::VecDeque;
use std::time::Instant;

use image::{Rgba, GenericImageView};
use rayon::prelude::*;
use spin_sleep::LoopHelper;

use crate::frames::VideoConfigs;

type Pixel = (u32, u32, Rgba<u8>);

fn draw_pixel(pixel: Pixel) {
    let x: u32 = pixel.1;
    let y = pixel.0;
    let [r, g, b, _a] = pixel.2.0;
    // ANSI black magic
    print!("\x1b[{};{}H\x1b[48;2;{};{};{}m \x1b[0;0m", x, y, r, g, b)
}

fn frame_diff(prev: &Vec<Pixel>, curr: &Vec<Pixel>) -> Vec<Pixel> {
    let mut diff: Vec<Pixel> = Vec::new();
    for i in 0..curr.len() {
        if prev[i].2 != curr[i].2 {
            diff.push(curr[i])
        }
    }
    return diff;
}

fn to_pixels(path: &DirEntry) -> Vec<Pixel> {
    let mut frame: Vec<Pixel> = Vec::new();
    let filename = path.path().to_str().unwrap().to_string();
    let img = image::open(filename).expect("File not found!");

    for pixel in img.pixels() {
        frame.push(pixel);
    }
    return frame;
}

pub fn pixel_render(configs: &VideoConfigs) {
    // Ensure frames are read in correct order
    let mut paths: Vec<_> = fs::read_dir(configs.get_frame_path())
        .unwrap().map(|r| r.unwrap()).collect();
    paths.sort_by_key(|dir| dir.path());

    // Parsing pixel data from image frames
    let mut all_frames: VecDeque<_> = paths.par_iter().map(|p| to_pixels(p)).collect();

    paths.clear();

    // // Getting pixel difference between frames
    let mut all_frame_diff: VecDeque<Vec<Pixel>> = VecDeque::new();
    let first = all_frames.pop_front().unwrap();
    let mut prev = first.clone();
    all_frame_diff.push_back(first);

    while !all_frames.is_empty() {
        let curr = all_frames.pop_front().unwrap();
        all_frame_diff.push_back(frame_diff(&prev, &curr));
        prev = curr;
    }

    // Clear screen
    print!("\x1bc");

    // Rendering to the terminal
    let mut fps_setter = LoopHelper::builder().build_with_target_rate((&configs).fps.unwrap() as f32);
    let now = Instant::now();

    while !all_frame_diff.is_empty() {
        fps_setter.loop_start();
        for pixel in all_frame_diff.pop_front().unwrap() {
            draw_pixel(pixel);
        }
        fps_setter.loop_sleep();
    }

    println!("\nTime taken: {} seconds", now.elapsed().as_secs())

}
