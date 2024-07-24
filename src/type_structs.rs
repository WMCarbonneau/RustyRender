pub(crate) static EPSILON: f64 = 0.000001;
pub(crate) static INFINITY: f64 = f64::MAX;

// Types to use in the renderer

/// # An RGB colour for use in the rendering engine
/// Contains elements r, g, b each of type u8
pub(crate) struct DiffuseColour {
    r: u8,
    g: u8,
    b: u8,
}
/// A ray containing an origin and a direction in 3D space
pub(crate) struct Ray {
    origin: Vec3D,
    direction: Vec3D
}
/// Intersection container containing the distance to the intersection and a reference to the object implementing the SceneObject trait
pub(crate) struct Intersection {
    distance: f64,
    object: Box<dyn SceneObject>
}
/// # The scene containing objects to be rendered
/// Contains a Vec<dyn Box> in which the Box type is a generic type for all structs implementing SceneObject
pub(crate) struct RenderScene {
    objects_list: Vec<Box<dyn SceneObject>>
}

///# A 3-dimensional vector with custom-implemented behaviour
#[derive(Copy, Clone)]
pub(crate) struct Vec3D {
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) z: f64,
}

/// # A sphere for use in creating the 3-dimensional scene
pub(crate) struct Sphere {
    center: Vec3D,
    radius: f64,
    colour: DiffuseColour,
    material_type: u8,
    emission: f64,
    refractive_index: f64,
}

/// # A plane for use in creating the 3-dimensional scene
pub(crate) struct Plane {
    normal: Vec3D, // must be normalized using normalize_plane function
    distance_to_origin: f64,
    colour: DiffuseColour,
    material_type: u8,
    emission: f64,
    refractive_index: f64,
}

// ***shared traits
pub(crate) trait SceneObject {
    /// Computes the surface normal of the object at a given point, returns a reference
    fn normal(&self, intersect_point: &Vec3D) -> Vec3D;
    /// Computes the intersection distance of the object with a given ray
    fn intersect(&self, intersect_ray: &Ray) -> f64;
}
impl SceneObject for Sphere {
    fn normal(&self, intersect_point: &Vec3D) -> Vec3D {
        self.center.scalar_mult(-1.0).add(intersect_point).scalar_mult(1.0/self.radius)
    }
    /// compute the intersection distance of the ray and the sphere
    fn intersect(&self, intersect_ray: &Ray) -> f64 {
        let mut return_type = 0.0;
        let ray_minus_origin  = intersect_ray.origin.subtract(&self.center);
        let component = ray_minus_origin.scalar_mult(2.0).dot(&intersect_ray.direction);
        let new_origin = ray_minus_origin.dot(&ray_minus_origin) - self.radius*self.radius;
        let mut disc = component*component - 4.0*new_origin;
        // avoid expensive square root if possible
        if disc >= 0.0 {
            disc = disc.sqrt();
            let solution1 = -component + disc;
            let solution2 = -component - disc;
            if solution2>EPSILON {
                return_type = solution2/2.0;
            }else if solution1 > EPSILON {
                return_type = solution1/2.0;
            }
        }
        return_type
    }
}

impl SceneObject for Plane {
    /// return reference to the normalized normal vector (Vec3D) of the plane
    fn normal(&self, intersect_point: &Vec3D) -> Vec3D {
        self.normal.clone()
    }

    /// compute intersection distance of the plane and the given ray
    fn intersect(&self, intersect_ray: &Ray) -> f64 {
        let intersect_direction_component = self.normal.dot(&intersect_ray.direction);
        if intersect_direction_component != 0.0 {
            let temp_result = -1.0 * (self.normal.dot(&intersect_ray.origin)+self.distance_to_origin)/intersect_direction_component;
            if temp_result > EPSILON {
                temp_result
            }else {
                0.0
            }
        }else {
            0.0
        }
    }
}

// ***implemented functions

impl DiffuseColour {

}

impl Vec3D {
    /// Add a Vec3D to another and return the result
    pub(crate) fn add(&self, other: &Vec3D) -> Vec3D {
        let result = Vec3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
        result
    }
    /// Subtract a Vec3D from another and return the result
    pub(crate) fn subtract(&self, other: &Vec3D) -> Vec3D {
        let result = Vec3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        };
        result
    }
    /// Do a cross-product of 2 Vec3D and return the result
    pub(crate) fn cross(&self, other: &Vec3D) -> Vec3D {
        let result = Vec3D {
            x: self.y*other.z - self.z*other.y,
            y: self.z*other.x - self.x*other.z,
            z: self.x*other.y - self.y*other.x,
        };
        result
    }
    /// Multiply the Vec3D by a scalar
    pub(crate) fn scalar_mult(&self, scalar: f64) -> Vec3D {
        let result = Vec3D {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        };
        result
    }
    /// Divide the Vec3D by a scalar
    pub(crate) fn scalar_div(&self, scalar: f64) -> Vec3D{
        let result = Vec3D {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        };
        result
    }
    /// Perform a dot-product of two Vec3D and return the resulting scalar f64
    pub(crate) fn dot(&self, other: &Vec3D) -> f64 {
        self.x*other.x + self.y*other.y + self.z*other.z
    }
    /// Compute the length of the Vec3D and return the scalar f64
    pub(crate) fn length(&self) -> f64 {
        (self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
    }
    /// Normalize the current Vec3D - mutates self
    /// Return zero vector if the norm will result in division by 0
    pub(crate) fn norm(&mut self) {
        let length = self.length();
        if length == 0.0 {
            self.x = 0.0;
            self.y = 0.0;
            self.z = 0.0;
        }else {
            self.x = self.x/length;
            self.y = self.y/length;
            self.z = self.z/length;
        }
    }
    /// Perform Hadamard (element-wise) product of two Vec3D and return the result.
    pub(crate) fn hadamard(&self, other: &Vec3D) -> Vec3D {
        let result = Vec3D {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        };
        result.clone()
    }
    /// Create an orthonormal system of 3 vectors Vec3D assuming self is normalized - mutates self, vec1, and vec 2
    /// This relies on vec 2 and vec 3 being empty but mutable. -> Done this way to preserve exterior scope of the two vectors
    pub(crate) fn orthonormal_system(&self, vec2: &mut Vec3D, vec3: &mut Vec3D) {
        if self.x.abs() > self.y.abs() {
            let target_length = 1.0_f64/((self.x*self.x+self.z*self.z).sqrt());
            vec2.x = -self.z* target_length;
            vec2.y = 0.0;
            vec2.z = self.x* target_length;
        }else {
            let target_length = 1.0_f64/((self.y*self.y+self.z*self.z).sqrt());
            vec2.x = 0.0;
            vec2.y = self.z* target_length;
            vec2.z = -self.y* target_length;
        }
        // do a cross-product of self and vec2 to get the third
        vec3.x = self.y*vec2.z - self.z*vec2.y;
        vec3.y = self.z*vec2.x - self.x*vec2.z;
        vec3.z = self.x*vec2.y - self.y*vec2.x;
    }
    /// Print the elements of the Vec3D in order.
    pub(crate) fn print(&self) {
        println!("{}, {}, {}", self.x, self.y, self.z)
    }
}

impl Sphere {

}

impl Plane {
    /// use this the safely create a plane
    fn normalize_plane(plane: &mut Plane) {
        plane.normal.norm();
    }
}

impl Ray {
    fn set_direction(&mut self, direction: &Vec3D) {
        self.direction.x = direction.x;
        self.direction.x = direction.y;
        self.direction.x = direction.z;

    }
}

impl RenderScene {
    /// Get the closest intersection, returns in an Option<> in case of no intersection
    fn intersect(&mut self, ray: &Ray) -> Option<Intersection> {
        let mut closest_intersection = -1;
        let mut closest_distance = 0.0;
        for (i,obj) in &mut self.objects_list.iter().enumerate() {
            let intersect_temp = obj.intersect(ray);
            if intersect_temp > EPSILON {
                if intersect_temp < closest_distance {
                    closest_intersection = i;
                    closest_distance = intersect_temp;
                }
            }
        }
        if closest_intersection == -1 {
            return None;
        }
        return Some(Intersection {
            distance: closest_distance,
            object: self.objects_list[closest_intersection].clone(),
        });
    }
}

// todo redo tests- changed mutability of Vec3D operations

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec3d_add_test() {
        let test_vec = Vec3D {
            x: 5.0,
            y: 3.0,
            z: 4.0,
        };
        let add_vec = Vec3D {
            x: 5.0,
            y: 3.0,
            z: 4.0,
        };

        let result = test_vec.add(&add_vec);
        assert_eq!(result.x, 10.0);
        assert_eq!(result.y, 6.0);
        assert_eq!(result.z, 8.0);
    }

    #[test]
    fn vec3d_subtract_test() {
        let test_vec = Vec3D {
            x: 5.0,
            y: 3.0,
            z: 4.0,
        };
        let add_vec = Vec3D {
            x: 2.0,
            y: 1.0,
            z: 3.0,
        };

        let result = test_vec.subtract(&add_vec);
        assert_eq!(result.x, 3.0);
        assert_eq!(result.y, 2.0);
        assert_eq!(result.z, 1.0);
    }

    #[test]
    fn vec3d_dot_test() {
        let test_vec = Vec3D {
            x: 5.0,
            y: 3.0,
            z: 4.0,
        };
        let add_vec = Vec3D {
            x: 5.0,
            y: 3.0,
            z: 4.0,
        };

        assert_eq!(test_vec.dot(&add_vec), 50.0);
    }

    #[test]
    fn vec3d_cross_test() {
        let test_vec = Vec3D {
            x: 5.0,
            y: 3.0,
            z: 4.0,
        };
        let add_vec = Vec3D {
            x: 5.0,
            y: 3.0,
            z: 5.0,
        };

        let result = test_vec.cross(&add_vec);
        assert_eq!(result.x, 3.0);
        assert_eq!(result.y, -5.0);
        assert_eq!(result.z, 0.0);
    }

    #[test]
    fn vec3d_scalar_mult_test() {
        let mut test_vec = Vec3D {
            x: 5.0,
            y: 3.0,
            z: 4.0,
        };

        test_vec.scalar_mult(4.0);
        assert_eq!(test_vec.x, 20.0);
        assert_eq!(test_vec.y, 12.0);
        assert_eq!(test_vec.z, 16.0);
    }

    #[test]
    fn vec3d_scalar_div_test() {
        let mut test_vec = Vec3D {
            x: 5.0,
            y: 6.0,
            z: 4.0,
        };

        test_vec.scalar_div(2.0);
        assert_eq!(test_vec.x, 2.5);
        assert_eq!(test_vec.y, 3.0);
        assert_eq!(test_vec.z, 2.0);
    }

    #[test]
    fn vec3d_length_test() {
        let test_vec = Vec3D {
            x: 0.0,
            y: 3.0,
            z: 4.0,
        };

        assert_eq!(test_vec.length(), 5.0);
    }

    #[test]
    fn vec3d_norm_test() {
        let mut test_vec1 = Vec3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        let mut test_vec2 = Vec3D {
            x: 0.0,
            y: 5.0,
            z: 0.0,
        };

        let mut test_vec3 = Vec3D {
            x: 2.0,
            y: 0.0,
            z: 0.0,
        };

        let mut test_vec4 = Vec3D {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        let mut test_vec5 = Vec3D {
            x: 0.0,
            y: 0.0,
            z: 4.0,
        };

        test_vec1.norm();
        assert_eq!(test_vec1.x, 0.0);
        assert_eq!(test_vec1.y, 0.0);
        assert_eq!(test_vec1.z, 0.0);

        test_vec2.norm();
        assert_eq!(test_vec2.x, 0.0);
        assert_eq!(test_vec2.y, 1.0);
        assert_eq!(test_vec2.z, 0.0);

        test_vec3.norm();
        assert_eq!(test_vec3.x, 1.0);
        assert_eq!(test_vec3.y, 0.0);
        assert_eq!(test_vec3.z, 0.0);

        test_vec5.norm();
        assert_eq!(test_vec5.x, 0.0);
        assert_eq!(test_vec5.y, 0.0);
        assert_eq!(test_vec5.z, 1.0);

        test_vec4.norm();
        let component = 1.0/3.0_f64.sqrt();
        assert_eq!(test_vec4.x, component);
        assert_eq!(test_vec4.y, component);
        assert_eq!(test_vec4.z, component);
    }

    #[test]
    fn vec3d_hadamard_test() {
        let test_vec = Vec3D {
            x: 5.0,
            y: 3.0,
            z: 4.0,
        };
        let add_vec = Vec3D {
            x: 2.0,
            y: 1.0,
            z: 0.0,
        };

        let result = test_vec.hadamard(&add_vec);
        assert_eq!(result.x, 10.0);
        assert_eq!(result.y, 3.0);
        assert_eq!(result.z, 0.0);
    }
    #[test]
    fn vec3d_orthonormal_system_test() {
        // normalized
        let test_vec = Vec3D {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        let mut test_vec1 = Vec3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let mut test_vec2 = Vec3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        test_vec.orthonormal_system(&mut test_vec1, &mut test_vec2);

        assert_eq!(test_vec1.x, 0.0);
        assert_eq!(test_vec1.y, 0.0);
        assert_eq!(test_vec1.z, 1.0);

        assert_eq!(test_vec2.x, 0.0);
        assert_eq!(test_vec2.y, -1.0);
        assert_eq!(test_vec2.z, 0.0);

        // test orthogonal with dot product
        assert_eq!(test_vec.dot(&test_vec1), 0.0);
        assert_eq!(test_vec.dot(&test_vec2), 0.0);
        assert_eq!(test_vec1.dot(&test_vec2), 0.0);
    }
}