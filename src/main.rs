#![allow(dead_code, unused_imports)]

mod primitives;
mod ray;
mod scene;
mod util;
mod vec3;

use image::{ImageBuffer, Rgb, RgbImage};
use nalgebra::{vector, Rotation3, Vector3};
use pixels::{Pixels, SurfaceTexture};
use primitives::*;
use ray::*;
use rayon::prelude::*;
use scene::*;
use std::ops::Mul;
use std::time::Duration;
use util::*;
use vec3::*;
use winit::dpi::LogicalSize;
use winit::event::Event;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

const MAX_COLOR_DIFFERENCE: f64 = 10.0;

fn main() {
    let rotation: Rotation3<f64> = Rotation3::from_euler_angles(0.0, -0.25, 0.0);

    let mut scene: Scene = scene::new_scene();
    init_scene(&mut scene);
    let mut img = RgbImage::new(scene.canvas_width as u32, scene.canvas_height as u32);

    //draw entire image without supersampling (1 ray per pixel)

    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Rays")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture).unwrap()
    };

    let img2 = &mut img as *mut ImageBuffer<Rgb<u8>, Vec<u8>>;
    std::thread::spawn(move || {
        img.par_enumerate_pixels_mut()
            .into_par_iter()
            .for_each(|(canvas_x, canvas_y, color)| {
                // std::thread::sleep(Duration::from_nanos(1));
                let (canvas_x, canvas_y) =
                    topleft_to_middle_by_dims(WIDTH, HEIGHT, canvas_x, canvas_y);
                let draw_color = generate_pixel(&scene, (canvas_x, canvas_y), &rotation, 1);
                let draw_color = v3_to_rgb(draw_color);
                *color = draw_color;
                // draw_pixel(&mut img, canvas_x, canvas_y, draw_color).unwrap();
            });

        for canvas_x in 1..scene.canvas_width - 1 {
            for canvas_y in 1..scene.canvas_height - 1 {
                let current_color = img.get_pixel(canvas_x as u32, canvas_y as u32);
                let current_color = new_vec(
                    current_color[0].into(),
                    current_color[1].into(),
                    current_color[2].into(),
                );
                let mut should_redraw: bool = false;

                'outer: for x in -1..=1 {
                    for y in -1..=1 {
                        if x == 0 && y == 0 {
                            continue;
                        }

                        let checking = img.get_pixel((canvas_x + x) as u32, (canvas_y + y) as u32);
                        let checking =
                            new_vec(checking[0].into(), checking[1].into(), checking[2].into());

                        if (current_color - checking).length() > MAX_COLOR_DIFFERENCE {
                            should_redraw = true;
                            break 'outer;
                        }
                    }
                }

                if should_redraw {
                    let (corrected_x, corrected_y) =
                        topleft_to_middle(&img, canvas_x as u32, canvas_y as u32);

                    let draw_color =
                        generate_pixel(&scene, (corrected_x, corrected_y), &rotation, 4);

                    let draw_color = v3_to_rgb(draw_color);
                    img.put_pixel(canvas_x as u32, canvas_y as u32, draw_color);
                }
            }
        }
    });

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            let a = unsafe {
                let a: Vec<u8> = (*img2)
                    .as_flat_samples()
                    .samples
                    .chunks(3)
                    .map(|rgb| [rgb[0], rgb[1], rgb[2], 255])
                    .flatten()
                    .collect();
                a
            };
            pixels.frame_mut().copy_from_slice(&a);
            if let Err(err) = pixels.render() {
                println!("Err: {:?}", err);
                control_flow.set_exit();
                return;
            }
        }

        window.request_redraw();
    });
    // let elapsed_time = now.elapsed();
    // println!("Rendered in {:.2} Seconds", elapsed_time.as_secs_f32());
    // println!("Redrew {} Pixels", times);
    // img.save("output.png").unwrap();
}

fn generate_pixel(
    scene: &Scene,
    current: (i32, i32),
    rotation: &Rotation3<f64>,
    n_samples: i8,
) -> V3 {
    //divides each pixel into subpixels (n_samples + 1)^2 big
    let mut final_color: V3 = new_vec(0.0, 0.0, 0.0);

    for sample_x in -n_samples / 2..=n_samples / 2 {
        for sample_y in -n_samples / 2..=n_samples / 2 {
            let viewframe_x: f64 = (current.0) as f64
                * (scene.viewframe_width / scene.canvas_width as f64)
                + sample_x as f64
                    * (scene.viewframe_width / (scene.canvas_width as f64 * n_samples as f64));
            let viewframe_y: f64 = (current.1) as f64
                * (scene.viewframe_height / scene.canvas_height as f64)
                + sample_y as f64
                    * (scene.viewframe_height / (scene.canvas_height as f64 * n_samples as f64));

            let na_ray = vector![viewframe_x, viewframe_y, scene.viewframe_distance]; //convert to a vector that nalgebra can use
            let rotated_ray = rotation.mul(na_ray);
            //after rotation, we'll convert it back to V3 (I should have just used nalgebra from the beginning)

            let ray = new_ray(
                scene.camera,
                new_vec(rotated_ray[0], rotated_ray[1], rotated_ray[2]),
            ); //ray from the camera to a physical point on the viewframe
            let draw_color = trace_ray(&scene, ray, EPSILON, INF, scene.recursion_limit);

            let divisions_in_pixel = match n_samples {
                1 => 1,
                _ => (n_samples + 1).pow(2),
            };

            final_color = final_color + (draw_color / divisions_in_pixel as f64);
        }
    }

    final_color
}

const WIDTH: i32 = 1600;
const HEIGHT: i32 = 900;

fn init_scene(scene: &mut Scene) {
    scene.canvas_width = WIDTH;
    scene.canvas_height = HEIGHT;

    scene.recursion_limit = 8;

    scene.viewframe_width = 2.0;
    scene.viewframe_height = 1.125;
    scene.viewframe_distance = 1.0;

    scene.camera = new_vec(1.0, 0.0, 0.0);

    scene.primitives.push(new_sphere(
        //red
        new_vec(0.0, -1.0, 3.0),
        1.0,
        new_vec(255.0, 0.0, 0.0),
        500.0,
        0.0,
    ));

    scene.primitives.push(new_sphere(
        //blue
        new_vec(2.0, 0.0, 4.0),
        1.0,
        new_vec(0.0, 0.0, 255.0),
        500.0,
        1.0,
    ));

    scene.primitives.push(new_sphere(
        //green
        new_vec(-2.0, 0.0, 4.0),
        1.0,
        new_vec(0.0, 255.0, 0.0),
        10.0,
        0.0,
    ));

    scene.primitives.push(new_sphere(
        //mirror
        new_vec(0.0, 3.0, 6.0),
        2.0,
        new_vec(255.0, 0.0, 255.0),
        500.0,
        1.0,
    ));

    scene.primitives.push(new_sphere(
        //sussy
        new_vec(0.0, 0.0, -4.9),
        5.0,
        new_vec(200.0, 230.0, 255.0),
        1000.0,
        0.0,
    ));

    scene.primitives.push(new_sphere(
        //ground
        new_vec(0.0, -5001.0, 0.0),
        5000.0,
        new_vec(255.0, 255.0, 0.0),
        1000.0,
        0.0,
    ));

    scene.ambient_light = 0.2;
    scene
        .lights
        .push(Light::Point((new_vec(-10.0, 10.0, -10.0), 2.0)));
    scene
        .lights
        .push(Light::Directional((new_vec(1.0, 4.0, 4.0), 0.2)));
}
