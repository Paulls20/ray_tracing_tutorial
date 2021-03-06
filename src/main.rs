mod tracing;

use std::fs::File;
use std::io::Write;
use itertools::Itertools;
use crate::tracing::{Vec3f, Sphere, Ray, Material, Light};

const WIDTH: i32 = 1024;
const HEIGHT: i32 = 768;
const FRAME_SIZE: usize = (WIDTH * HEIGHT) as usize;

#[derive(Copy, Clone)]
struct Intersect {
    material: Material,
    hit: Vec3f,
    N: Vec3f
}

fn scene_intersect(orig: Vec3f, dir: Vec3f, spheres: &Vec<Sphere>) -> Option<Intersect> {
    let mut intersect_ret = None;
    for sphere in spheres {

        if let Some(dist_i) = sphere.intersect(orig, dir) {
            let mut intersect: Intersect = Intersect {
                material: Material { diffuse_color: Vec3f(0.0, 0.0, 0.0) },
                hit: Vec3f(0.0, 0.0, 0.0),
                N: Vec3f(0.0, 0.0, 0.0)
            };
            intersect.material = sphere.get_material();
            intersect.hit = orig + dir * dist_i;
            intersect.N = (intersect.hit - sphere.get_center()).normalize();

            intersect_ret = Some(intersect);
        }
    }
    intersect_ret
}

fn cast_ray(orig: Vec3f, dir: Vec3f, spheres: &Vec<Sphere>, lights: &Vec<Light>) -> Vec3f {
    match scene_intersect(orig, dir, spheres) {
        None => Vec3f(0.2, 0.7, 0.8),
        Some(intersect) => {
            //intersect.material.diffuse_color
            let mut diffuse_light_intensity = 0f32;
            for light in lights {
                let light_dir = (light.position - intersect.hit).normalize();
                diffuse_light_intensity += light.intensity * 0f32.max(light_dir*intersect.N);
            }
            intersect.material.diffuse_color * diffuse_light_intensity
        },
    }
}

fn generate_frame_buffer(spheres: &Vec<Sphere>, lights: &Vec<Light>) -> Vec<Vec3f> {
    let mut frame_buffer: Vec<Vec3f> = vec![Vec3f(0f32, 0f32, 0f32); FRAME_SIZE];
    const FOV: f32 = std::f32::consts::PI / 2f32;
    for (i, j) in (0..WIDTH).cartesian_product(0..HEIGHT) {
        let index: usize = (i + j * WIDTH) as usize;
        //let element = Vec3f(j as f32 / HEIGHT as f32, i as f32 / WIDTH as f32, 0f32);
        let x =  (2f32 * (i as f32 + 0.5f32) / WIDTH as f32  - 1f32) * (FOV / 2f32).tan() * WIDTH as f32 / HEIGHT as f32;
        let y = -(2f32 * (j as f32 + 0.5f32)/ HEIGHT as f32 - 1f32) * (FOV / 2f32).tan();
        let dir = Vec3f(x, y, -1f32).normalize();
        frame_buffer[index] = cast_ray(Vec3f(0f32, 0f32, 0f32), dir, spheres, lights);
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

fn render(spheres: Vec<Sphere>, lights: &Vec<Light>) {
    let frame_buffer = generate_frame_buffer(&spheres, lights);
    let pixels = get_pixels(&frame_buffer);
    write_image(pixels);
}

fn main() {
    let ivory = Material::new(Vec3f(0.4, 0.4, 0.3));
    let red_rubber = Material::new(Vec3f(0.3, 0.1, 0.1));
    let spheres = vec![Sphere::new(Vec3f(-3f32, 0f32,   -16f32), 2f32,      ivory.clone()),
                       Sphere::new(Vec3f(-1.0f32, -1.5f32, -12f32), 2f32, red_rubber.clone()),
                       Sphere::new(Vec3f( 1.5f32, -0.5f32, -18f32), 3f32, red_rubber.clone()),
                       Sphere::new(Vec3f( 7f32, 5f32, -18f32), 4f32,      ivory.clone())];


    let lights: Vec<Light> = vec![Light{ position: Vec3f(-20f32, 20f32,  20f32), intensity: 1.5f32}];
    render(spheres, &lights);
}
