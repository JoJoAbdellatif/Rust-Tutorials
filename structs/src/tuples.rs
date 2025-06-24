// A tuple struct is created and instantiated with parenthesis.
struct Point3D(f64 , f64 , f64);

// The implementation of the logic for the point uses the "impl" keyword followed by the struct name.
// Inside the "impl" code block, all the functions can be written.
// This is done such that any instance created of the struct can use the implemented functions.
// Each function written in the "impl" code block should have a parameter referencing the instance as "&self".
impl Point3D {
    // The "&self" reference in the "describe" and "multiply_by_2" functions is immutable.
    // The values in the instance referenced by "&self" cannot be modified.
    fn describe(&self) {
        println!("Point is at ({} , {} , {})" , self.0 , self.1 , self.2);
    }

    // This function multiplies each value in the point by 2 and returns a new point
    fn multiply_by_2(&self) -> Point3D {
        Point3D(self.0 * 2.0 , self.1 * 2.0 , self.2 * 2.0)
    }

    // The following function takes a mutable referece "&mut self".
    fn multiply_by_2_mut(&mut self){
        self.0 *= 2.0;
        self.1 *= 2.0;
        self.2 *= 2.0;
    }
}

// There can be multiple implementation blocks which is useful for readability and proper code segmentation.
// All "impl"s have to have the same name as the struct name.
impl Point3D {
    
    // some functions are called a non-method associated function.
    // This kind of functions does not need the associative reference of "&self".
    // The following function returns a point that sets all fields to 0.
    fn zero() -> Point3D{
        Point3D(0.0 , 0.0 , 0.0)
    }
}

pub fn instantiation_3d(){
    let origin = Point3D(0.0 , 0.0 , 0.0);

    // The values of each element in the tuple are accessed through the dot notation as well.
    println!("x = {} , y = {} , z = {}" , origin.0 , origin.1 , origin.2);

    // since "origin" is of type "Point3D", functions from the implementation can be used.
    origin.describe();

    // The following segment instantiates a point, multiplies the values in it by 2, and takes the point returned into a new point.
    let point_a = Point3D(5.0 , 10.0 , 15.0);
    let _point_b = point_a.multiply_by_2();

    // The call to this function modifies the values in point_c without putting the values in another point.
    let mut point_c = Point3D(5.0 , 10.0 , 15.0);
    point_c.describe();
    point_c.multiply_by_2_mut();
    point_c.describe();

    // The following call instantiates a point with all values set to 0
    let _point_d = Point3D::zero();

}