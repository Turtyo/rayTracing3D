pub mod math {
    pub mod types {
        #[derive(Clone, Copy)]
        pub struct Point {
            pub x: f64,
            pub y: f64,
            pub z: f64,
        }

        #[derive(Clone, Copy)]
        pub struct Vector {
            pub x: f64,
            pub y: f64,
            pub z: f64,
        }

        pub struct UnitVector {
            x: f64,
            y: f64,
            z: f64,
        }

        impl UnitVector {
			// ? how to automatically call new
            fn new(x: f64, y: f64, z: f64) -> Self {
                let vector = Vector { x, y, z };
                let norme = super::norme(&vector);
                let mut unit_vector = Self {
                    x: 0.,
                    y: 0.,
                    z: 0.,
                };
                unit_vector.x = x / norme;
                unit_vector.y = y / norme;
                unit_vector.z = z / norme;
				
				// ? is vector discarded here

                unit_vector
            }
        }

        pub struct Ray {
            pub origin: Point,
            pub direction: UnitVector,
        }
    }

    pub fn make_vector(origin: &types::Point, destination: &types::Point) -> types::Vector {
        let mut vector = types::Vector {
            x: 0.,
            y: 0.,
            z: 0.,
        };
        vector.x = destination.x - origin.x;
        vector.y = destination.y - origin.y;
        vector.z = destination.z - origin.z;

        vector
    }

    pub fn scalar_product(vector_1: &types::Vector, vector_2: &types::Vector) -> f64 {
        vector_1.x * vector_2.x + vector_1.y * vector_2.y + vector_1.z * vector_2.z
    }

    pub fn norme(vector: &types::Vector) -> f64 {
        f64::sqrt(scalar_product(vector, vector))
    }

    pub fn make_unit_vector(vector: &types::Vector) -> types::Vector {
        let norme = norme(vector);
        // ! need to actually copy the data, not the reference
        let mut unit_vector = *vector;
        unit_vector.x /= norme;
        unit_vector.y /= norme;
        unit_vector.z /= norme;

        unit_vector
    }
}
