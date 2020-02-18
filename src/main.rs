mod tracing;

use std::fs::File;
use std::io::Write;
use itertools::Itertools;
use crate::tracing::{Vec3f, Sphere, Ray};

const WIDTH: i32 = 1024;
const HEIGHT: i32 = 768;
const FRAME_SIZE: usize = (WIDTH * HEIGHT) as usize;

fn generate_frame_buffer() -> Vec<Vec3f> {
    let mut frame_buffer: Vec<Vec3f> = vec![Vec3f(0f32, 0f32, 0f32); FRAME_SIZE];
    for (i, j) in (0..WIDTH).cartesian_product(0..HEIGHT) {
        let index: usize = (i + j * WIDTH) as usize;
        let element = Vec3f(j as f32 / HEIGHT as f32, i as f32 / WIDTH as f32, 0f32);
        frame_buffer[index] = element;
    }
    frame_buffer
}

fn write_image(pixel_array: Vec<String>) {
    let mut file = File::create("./out.ppm").unwrap();
    let output = pixel_array.join("\n");
    file.write_all(output.as_bytes());
}

fn get_pixels(frame_buffer: &Vec<Vec3f>) -> Vec<String> {
    let mut pixel_array = vec![format!("P3\n{} {}\n255", WIDTH, HEIGHT)];
    for i in 0..FRAME_SIZE {
        let ir = (255.99 * frame_buffer[i].0) as u32;
        let ig = (255.99 * frame_buffer[i].1) as u32;
        let ib = (255.99 * frame_buffer[i].2) as u32;
        pixel_array.push(format!("{} {} {}", ir, ig, ib));
    }
    pixel_array
}

fn render() {
    let frame_buffer = generate_frame_buffer();
    let pixels = get_pixels(&frame_buffer);
    write_image(pixels);
}

fn main() {
    render();
    let s = Sphere::new(Vec3f(0f32, 0f32, 0f32), 10f32);
    s.intersect(Vec3f(0f32, 0f32, 0f32), Vec3f(0f32, 0f32, 0f32));
}
