use std::f64::consts::PI;

use rand::Rng;

use crate::type_structs::{DiffuseColour, Ray, RenderScene, Vec3D};

pub(crate) const WIDTH: i32 = 800;
pub(crate) const HEIGHT: i32 = 800;
static FIELD_OF_VIEW_HORIZONTAL: f64 = PI/4.0;
static FIELD_OF_VIEW_VERTICAL: f64 = (HEIGHT/WIDTH) as f64 * PI/4.0;


/// Get the coordinates on the camera plane
fn cam_plane_coordinate(x: i32,y: i32) -> Vec3D {
    Vec3D {
        x: (((2*x - WIDTH) as f64)/WIDTH as f64)*FIELD_OF_VIEW_HORIZONTAL.tan(),
        y: -((2*y-HEIGHT)as f64/HEIGHT as f64)*FIELD_OF_VIEW_VERTICAL.tan(),
        z: -1.0,
    }
}

/// Get a hemisphere sample vector
fn hemisphere() -> Vec3D {
    let rand: f64 = rand::thread_rng().gen_range(0.0, 1.0);
    // let rand2: f64 = rand::thread_rng().gen_range(0.0, 1.0);
    // let radius = (1.0-rand*rand);
    // let phi = 2.0*PI*rand2;
    // return Vec3D {
    //     x: phi.cos()*radius,
    //     y: phi.sin()*radius,
    //     z: rand
    // };

    let radius = rand.sqrt();
    let angle = 2.0*PI*rand::thread_rng().gen_range(0.0, 1.0);
    let x_pos = radius*angle.cos();
    let y_pos = radius*angle.sin();
    return Vec3D {
        x: x_pos,
        y: y_pos,
        z: f64::max(0.0,1.0-rand)
    }
}

fn trace(ray: &mut Ray, render_scene: &RenderScene, recursion_depth: i32, colour: &mut DiffuseColour) {
    let mut roulette_factor = 1.0;
    // exit condition
    if recursion_depth >= 5 {
        if rand::thread_rng().gen_range(-1.0, 1.0) <= 0.1 {
            return;
        }else {
            // weight of subsequent results
            roulette_factor = 1.0/(0.9);
        }
    }

    let intersection = render_scene.intersect(ray);
    // if the intersection is none, return, else extract it
    let intersection_validated = match intersection {
        None => {return;}
        Some(x) => {
            x
        }
    };

    // todo remove
    // colour.add(intersection_validated.object.colour().mult_return(21.25));
    // return;

    let hit_point = ray.origin.add(&ray.direction.scalar_mult(intersection_validated.distance));
    let mut normal = intersection_validated.object.normal(&hit_point);

    ray.origin = hit_point.clone();
    // at this point we have detected the nearest object and can access its properties
    let mut emission_factor = DiffuseColour {
        r: intersection_validated.object.colour().r as f64 /12.0*intersection_validated.object.emission(),
        g: intersection_validated.object.colour().g as f64 /12.0*intersection_validated.object.emission(),
        b: intersection_validated.object.colour().b as f64 /12.0*intersection_validated.object.emission(),
    };
    emission_factor.mult(roulette_factor);
    colour.add(emission_factor);



    if intersection_validated.object.material_type() == 1 { // diffuse material
        let mut rotation_x = Vec3D {x: 0.0, y: 0.0, z: 0.0};
        let mut rotation_y = Vec3D {x: 0.0, y: 0.0, z: 0.0};
        normal.orthonormal_system(&mut rotation_x,&mut rotation_y);

        let sample_direction = hemisphere();

        let rotated_direction = Vec3D {
            x: Vec3D {x: rotation_x.x, y: rotation_y.x, z: normal.x}.dot(&sample_direction),
            y: Vec3D {x: rotation_x.y, y: rotation_y.y, z: normal.y}.dot(&sample_direction),
            z: Vec3D {x: rotation_x.z, y: rotation_y.z, z: normal.z}.dot(&sample_direction),
        };

        ray.direction = rotated_direction;

        let cosine_direction = ray.direction.dot(&normal);

        let mut temp_colour = DiffuseColour {r: 0.0,g: 0.0, b: 0.0};

        trace(ray, render_scene, recursion_depth+1, &mut temp_colour);

        // todo might have some ownership issues here
        colour.add(temp_colour.mult_colour_return(intersection_validated.object.colour()).mult_return(cosine_direction*0.1*roulette_factor));
        // println!("r:{},g:{},b:{}", colour.r, colour.g,colour.b)
    }else if intersection_validated.object.material_type() == 2 { // specular material
        let cosine_direction = ray.direction.dot(&normal);
        ray.set_direction(&ray.direction.subtract(&normal.scalar_mult(cosine_direction*2.0)));
        ray.direction.norm();

        let mut temp_colour = DiffuseColour {r: 0.0,g: 0.0, b: 0.0};

        trace(ray, render_scene, recursion_depth+1, &mut temp_colour);

        colour.add(temp_colour.mult_return(roulette_factor));
        // println!("r:{},g:{},b:{}", colour.r, colour.g,colour.b)

    }else if intersection_validated.object.material_type() == 3 { // refractive material
        let mut r_index = intersection_validated.object.refractive_index();
        let ratio = ((1.0-r_index)/(1.0+r_index)).powi(2);

        // if inside the medium
        if normal.dot(&ray.direction) > 0.0 {
            normal = normal.scalar_mult(-1.0);
        }else {
            r_index = 1.0/r_index;
        }

        let cosine_direction_1 = -1.0 * normal.dot(&ray.direction);
        let cosine_direction_2 = 1.0 - (r_index*r_index*(1.0-(cosine_direction_1*cosine_direction_1)));
        // Schlick approximation
        let fresnel_probability_factor = ratio + (1.0-ratio)*((1.0-cosine_direction_1).powi(5));

        if cosine_direction_2 > 0.0 && rand::thread_rng().gen_range(0.0, 1.0) > fresnel_probability_factor {
            ray.set_direction(&ray.direction.scalar_mult(r_index).add(&normal.scalar_mult(r_index*cosine_direction_1-cosine_direction_2.sqrt())));
            ray.direction.norm();
        }else {
            ray.set_direction(&ray.direction.add(&normal.scalar_mult(cosine_direction_1*2.0)));
            ray.direction.norm();
        }

        let mut temp_colour = DiffuseColour {r: 0.0,g: 0.0, b: 0.0};

        trace(ray, render_scene, recursion_depth+1, &mut temp_colour);

        colour.add(temp_colour.mult_return(1.15*roulette_factor));
        // println!("r:{},g:{},b:{}", colour.r, colour.g,colour.b)

    }
}

pub(crate) fn simulate_per_pixel(column: i32, row: i32, render_scene: &RenderScene, samples: i32, image_pixels: &mut Vec<DiffuseColour>) {
    for _ in 0..samples {
        let mut colour_master = DiffuseColour {r:0.0,g:0.0,b:0.0};

        let mut camera = cam_plane_coordinate(column, row);

        // randomized anti-aliasing
        camera.x = camera.x + rand::thread_rng().gen_range(-1.0, 1.0)/700.0;
        camera.y = camera.y + rand::thread_rng().gen_range(-1.0, 1.0)/700.0;

        camera.norm();

        let mut ray = Ray {
            origin: Vec3D {x:0.0,y:0.0,z:0.0},
            direction: camera.clone()
        };
        trace(&mut ray, render_scene, 0, &mut colour_master);

        // if colour_master.r != 0.0 || colour_master.g != 0.0 || colour_master.b != 0.0 {
        //     println!("r:{},g:{},b:{}", colour_master.r, colour_master.g, colour_master.b); // todo no colour out of trace
        // } 

        // set the pixel
        image_pixels[(row+column*WIDTH) as usize] = image_pixels[(row+column*WIDTH) as usize].add_return(colour_master.mult_return(1.0/samples as f64));
        // println!("Set {column},{row} as r:{},g:{},b:{}", image_pixels[(row+column*WIDTH) as usize].r, image_pixels[(row+column*WIDTH) as usize].g,image_pixels[(row+column*WIDTH) as usize].b)
    }
}