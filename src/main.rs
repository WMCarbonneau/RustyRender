use arrayvec::ArrayVec;
use image::{ImageBuffer, ImageResult, Rgb};
use crate::rendering_equation::{HEIGHT, simulate_per_pixel, WIDTH};
use crate::type_structs::{DiffuseColour, Plane, RenderScene, Sphere, Vec3D};

pub mod type_structs;
mod rendering_equation;

static SAMPLES: i32 = 8;
fn build_diffuse_colour() -> DiffuseColour{
    DiffuseColour {
        r: 0.0,
        g: 0.0,
        b: 0.0
    }
}

fn main() {
    println!("Starting");

    // create scene
    let mut scene: RenderScene = RenderScene {
        objects_list: Vec::new()
    };
    // add objects to the scene
    // spheres
    scene.objects_list.push(Box::new(Sphere { center: Vec3D { x: -0.75, y: -1.45, z: -4.4 }, radius: 1.05, colour: DiffuseColour { r: 4.0, g: 8.0, b: 4.0 }, material_type: 2, emission: 0.0, refractive_index: 1.3 }));
    scene.objects_list.push(Box::new(Sphere { center: Vec3D { x: 2.0, y: -2.05, z: -3.7 }, radius: 0.5, colour: DiffuseColour { r: 10.0, g: 10.0, b: 1.0 }, material_type: 3, emission: 0.0, refractive_index: 1.3 }));
    scene.objects_list.push(Box::new(Sphere { center: Vec3D { x: -1.75, y: -1.95, z: -3.1 }, radius: 0.6, colour: DiffuseColour { r: 4.0, g: 4.0, b: 12.0 }, material_type: 1, emission: 0.0, refractive_index: 1.3 }));
    scene.objects_list.push(Box::new(Sphere { center: Vec3D { x: 0.0, y: 1.9, z: -3.0 }, radius: 0.5, colour: DiffuseColour { r: 12.0, g: 12.0, b: 12.0 }, material_type: 1, emission: 10000.0, refractive_index: 1.3 }));

    // planes
    scene.objects_list.push(Box::new(Plane {normal: Vec3D {x: 0.0, y: 1.0, z: 0.0}, distance_to_origin: 2.5, colour: DiffuseColour {r: 6.0, g: 6.0, b: 6.0}, material_type: 1, emission: 0.0, refractive_index: 0.0}));
    scene.objects_list.push(Box::new(Plane {normal: Vec3D {x: 0.0, y: 0.0, z: 1.0}, distance_to_origin: 5.5, colour: DiffuseColour {r: 6.0, g: 6.0, b: 6.0}, material_type: 1, emission: 0.0, refractive_index: 0.0}));
    scene.objects_list.push(Box::new(Plane {normal: Vec3D {x: 1.0, y: 0.0, z: 0.0}, distance_to_origin: 2.75, colour: DiffuseColour {r: 10.0, g: 2.0, b: 2.0}, material_type: 1, emission: 0.0, refractive_index: 0.0}));
    scene.objects_list.push(Box::new(Plane {normal: Vec3D {x: -1.0, y: 0.0, z: 0.0}, distance_to_origin: 2.75, colour: DiffuseColour {r: 2.0, g: 10.0, b: 2.0}, material_type: 1, emission: 0.0, refractive_index: 0.0}));
    scene.objects_list.push(Box::new(Plane {normal: Vec3D {x: 0.0, y: -1.0, z: 0.0}, distance_to_origin: 3.0, colour: DiffuseColour {r: 6.0, g: 6.0, b: 6.0}, material_type: 1, emission: 0.0, refractive_index: 0.0}));
    scene.objects_list.push(Box::new(Plane {normal: Vec3D {x: 0.0, y: 0.0, z: -1.0}, distance_to_origin: 0.5, colour: DiffuseColour {r: 6.0, g: 6.0, b: 6.0}, material_type: 1, emission: 0.0, refractive_index: 0.0}));

    // create and simulate pixels
    // create pixels array and initialize all of them
    let mut image_pixels = vec![build_diffuse_colour(); (WIDTH * HEIGHT) as usize];

    // main loop
    for i in 0..WIDTH {
        for j in 0..HEIGHT {
            simulate_per_pixel(i,j,&scene,SAMPLES,&mut image_pixels);

        }
    }

    let mut buff: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(WIDTH as u32, HEIGHT as u32);
    // print all pixels
    for (i,pixel) in image_pixels.iter().enumerate() {
        buff.put_pixel((i as i32 / WIDTH) as u32,(i as i32 % WIDTH) as u32 , Rgb([u8::min(pixel.r as u8, 255),u8::min(pixel.g as u8, 255),u8::min(pixel.b as u8, 255)]));
        // println!("{},{},{}", i.r, i.g,i.b);
    }
    match buff.save("Converged.png") {
        Ok(_) => {println!("Saved as: Converged.png")}
        Err(_) => {println!("Something went wrong")}
    }
    println!("finished");
}
