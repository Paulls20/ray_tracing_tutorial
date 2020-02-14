use std::fs::File;
use std::io::Write;

const WIDTH: i32 = 1024;
const HEIGHT: i32 = 768;
const FRAME_SIZE: usize = (WIDTH * HEIGHT) as usize;

type Vec2f = [f32; 2];
type Vec3f = [f32; 3];
type Vec3i = [i32; 3];
type Vec4f = [f32; 4];

fn generate_frame_buffer() -> Vec<Vec3f> {
    let mut frame_buffer: Vec<Vec3f> = vec![[0f32, 0f32, 0f32]; FRAME_SIZE];

    for j in 0..HEIGHT {
        for i in 0..WIDTH {
            let index: usize = (i + j * WIDTH) as usize;
            let element: Vec3f = [(j as f32 / HEIGHT as f32), (i as f32 / WIDTH as f32), 0f32];
            frame_buffer[index] = element;
        }
    }

    frame_buffer
}

fn write_image(pixel_array: Vec<u8>) {
    let mut image = File::create("./out.ppm").unwrap();
    image.write_fmt(format_args!("P6\n{} {} \n255\n", WIDTH, HEIGHT));
    image.write(pixel_array.as_slice());
}

fn get_pixels(frame_buffer: &Vec<Vec3f>) -> Vec<u8> {
    let mut pixel_array = vec![0u8; FRAME_SIZE * 3];
    for i in 0..FRAME_SIZE {
        for j in 0..3 {
            let pixel = 255f32 * frame_buffer[i][j].min(1f32).max(0f32);
            pixel_array.push(pixel as u8);
        }
    }
    pixel_array
}

fn main() {
    let frame_buffer = generate_frame_buffer();
    let pixels = get_pixels(&frame_buffer);
    write_image(pixels);
}
